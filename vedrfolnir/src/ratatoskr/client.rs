use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;
use tracing::{info, error, warn};
use ratatoskr::protocol::{MessageSerializer, MessageValidator, ConnectionProtocol, MessageSigner, NonceManager};
use ratatoskr::messages::{RatatoskrRequest, RatatoskrResponse};
use ratatoskr::proto::ratatoskr::{MessageType, ConnectionRequestPayload, ConnectionResponsePayload};
use chrono::{Utc, Duration as ChronoDuration};
use uuid::Uuid;
use prost::Message;

#[derive(Debug, Error)]
pub enum RatatoskrError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Message serialization failed: {0}")]
    SerializationError(#[from] ratatoskr::protocol::SerializationError),
    #[error("Message validation failed: {0}")]
    ValidationError(#[from] ratatoskr::protocol::ValidationError),
    #[error("Connection closed"),
    ConnectionClosed,
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

pub struct RatatoskrClient {
    url: String,
    device_id: String,
    user_id: String,
    device_identity: String,
    authentication_token: String,
    signer: MessageSigner,
    nonce_manager: NonceManager,
    serializer: MessageSerializer,
    validator: MessageValidator,
    connection_protocol: ConnectionProtocol,
}

impl RatatoskrClient {
    pub fn new(
        url: String,
        device_id: String,
        user_id: String,
        device_identity: String,
        authentication_token: String,
    ) -> Self {
        Self {
            url,
            device_id,
            user_id,
            device_identity,
            authentication_token,
            signer: MessageSigner::from_bytes(&[0u8; 32]).unwrap(), // TODO: Load from config
            nonce_manager: NonceManager::new(),
            serializer: MessageSerializer::new(),
            validator: MessageValidator::new(),
            connection_protocol: ConnectionProtocol::new(),
        }
    }

    pub async fn connect(&mut self) -> Result<RatatoskrConnection, RatatoskrError> {
        info!("Connecting to Ratatoskr server at {}", self.url);
        
        let (ws_stream, _) = connect_async(&self.url)
            .await
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("WebSocket connection failed: {}", e)))?;
        
        info!("WebSocket connection established");
        
        let (mut write, mut read) = ws_stream.split();
        
        // Create and send connection request
        let request_id = Uuid::new_v4().to_string();
        let mut request = RatatoskrRequest::new_connection_request(
            request_id.clone(),
            self.device_id.clone(),
            self.user_id.clone(),
            self.device_identity.clone(),
            self.authentication_token.clone(),
            "1.0".to_string(),
        );
        
        // Generate nonce and sign the request
        request.nonce = self.nonce_manager.generate_nonce();
        self.signer.sign_request(&mut request)
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Signing failed: {:?}", e)))?;
        
        // Serialize and send
        let serialized = self.serializer.serialize_request(&request)?;
        write.send(Message::Binary(serialized)).await
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Send failed: {}", e)))?;
        
        // Wait for connection response
        let response_msg = read.next().await
            .ok_or(RatatoskrError::ConnectionClosed)??
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Receive failed: {}", e)))?;
        
        match response_msg {
            Message::Binary(data) => {
                let response = self.serializer.deserialize_response(&data)?;
                self.validator.validate_response(&response)?;
                
                if response.message_type != MessageType::ConnectionResponse as i32 {
                    return Err(RatatoskrError::InvalidResponse(
                        format!("Expected ConnectionResponse, got {}", response.message_type)
                    ));
                }
                
                if !response.success {
                    return Err(RatatoskrError::ConnectionFailed(
                        format!("Connection rejected: {}", response.error_message)
                    ));
                }
                
                // Parse connection response payload
                let payload = self.connection_protocol.parse_connection_response_payload(&response)
                    .map_err(|e| RatatoskrError::InvalidResponse(format!("Failed to parse payload: {}", e)))?;
                
                info!("Connection established with session_id: {}", payload.session_id);
                
                Ok(RatatoskrConnection {
                    write: Arc::new(RwLock::new(write)),
                    read: Arc::new(RwLock::new(read)),
                    session_id: payload.session_id,
                    device_id: self.device_id.clone(),
                    user_id: self.user_id.clone(),
                    signer: self.signer.clone(),
                    nonce_manager: self.nonce_manager.clone(),
                    serializer: self.serializer.clone(),
                    validator: self.validator.clone(),
                })
            }
            Message::Close(_) => Err(RatatoskrError::ConnectionClosed),
            _ => Err(RatatoskrError::InvalidResponse("Unexpected message type".to_string())),
        }
    }
}

pub struct RatatoskrConnection {
    write: Arc<RwLock<futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>>>,
    read: Arc<RwLock<futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>>>,
    session_id: String,
    device_id: String,
    user_id: String,
    signer: MessageSigner,
    nonce_manager: NonceManager,
    serializer: MessageSerializer,
    validator: MessageValidator,
}

impl RatatoskrConnection {
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub async fn send_business_request(
        &self,
        payload: Vec<u8>,
    ) -> Result<RatatoskrResponse, RatatoskrError> {
        let request_id = Uuid::new_v4().to_string();
        let mut request = RatatoskrRequest::new_business_request(
            request_id.clone(),
            self.device_id.clone(),
            self.user_id.clone(),
            payload,
        );
        
        // Generate nonce and sign the request
        request.nonce = self.nonce_manager.generate_nonce();
        self.signer.sign_request(&mut request)
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Signing failed: {:?}", e)))?;
        
        // Serialize and send
        let serialized = self.serializer.serialize_request(&request)?;
        let mut write = self.write.write().await;
        write.send(Message::Binary(serialized)).await
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Send failed: {}", e)))?;
        drop(write);
        
        // Wait for response
        let mut read = self.read.write().await;
        let response_msg = read.next().await
            .ok_or(RatatoskrError::ConnectionClosed)??
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Receive failed: {}", e)))?;
        drop(read);
        
        match response_msg {
            Message::Binary(data) => {
                let response = self.serializer.deserialize_response(&data)?;
                self.validator.validate_response(&response)?;
                Ok(response)
            }
            Message::Close(_) => Err(RatatoskrError::ConnectionClosed),
            _ => Err(RatatoskrError::InvalidResponse("Unexpected message type".to_string())),
        }
    }

    pub async fn send_heartbeat(&self) -> Result<(), RatatoskrError> {
        let request_id = Uuid::new_v4().to_string();
        let mut request = RatatoskrRequest::new_heartbeat(
            request_id,
            self.device_id.clone(),
            self.user_id.clone(),
        );
        
        // Generate nonce and sign the request
        request.nonce = self.nonce_manager.generate_nonce();
        self.signer.sign_request(&mut request)
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Signing failed: {:?}", e)))?;
        
        // Serialize and send
        let serialized = self.serializer.serialize_request(&request)?;
        let mut write = self.write.write().await;
        write.send(Message::Binary(serialized)).await
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Send failed: {}", e)))?;
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), RatatoskrError> {
        let request_id = Uuid::new_v4().to_string();
        let mut request = RatatoskrRequest::new_disconnect(
            request_id,
            self.device_id.clone(),
            self.user_id.clone(),
        );
        
        // Generate nonce and sign the request
        request.nonce = self.nonce_manager.generate_nonce();
        self.signer.sign_request(&mut request)
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Signing failed: {:?}", e)))?;
        
        // Serialize and send
        let serialized = self.serializer.serialize_request(&request)?;
        let mut write = self.write.write().await;
        write.send(Message::Binary(serialized)).await
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Send failed: {}", e)))?;
        write.close().await
            .map_err(|e| RatatoskrError::ConnectionFailed(format!("Close failed: {}", e)))?;
        Ok(())
    }
}
