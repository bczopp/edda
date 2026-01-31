use crate::messages::*;
use crate::proto::ratatoskr::*;
use prost::Message;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Failed to decode payload: {0}")]
    DecodeError(#[from] prost::DecodeError),
    #[error("Invalid message type: expected {expected}, got {actual}")]
    InvalidMessageType { expected: String, actual: i32 },
}

/// ConnectionProtocol handles connection establishment and handshake
pub struct ConnectionProtocol;

impl ConnectionProtocol {
    /// Create a new ConnectionProtocol
    pub fn new() -> Self {
        Self
    }

    /// Create a connection response from a connection request
    pub fn create_connection_response(
        &self,
        request: &RatatoskrRequest,
        accepted: bool,
        session_id: String,
        expires_at: i64,
        server_version: String,
    ) -> Result<RatatoskrResponse, ConnectionError> {
        // Verify this is a connection request
        if request.message_type != MessageType::ConnectionRequest as i32 {
            return Err(ConnectionError::InvalidMessageType {
                expected: "CONNECTION_REQUEST".to_string(),
                actual: request.message_type,
            });
        }

        Ok(RatatoskrResponse::new_connection_response(
            request.request_id.clone(),
            accepted,
            session_id,
            expires_at,
            server_version,
        ))
    }

    /// Parse connection request payload
    pub fn parse_connection_request_payload(
        &self,
        request: &RatatoskrRequest,
    ) -> Result<ConnectionRequestPayload, ConnectionError> {
        if request.message_type != MessageType::ConnectionRequest as i32 {
            return Err(ConnectionError::InvalidMessageType {
                expected: "CONNECTION_REQUEST".to_string(),
                actual: request.message_type,
            });
        }

        ConnectionRequestPayload::decode(&*request.payload)
            .map_err(ConnectionError::DecodeError)
    }

    /// Parse connection response payload
    pub fn parse_connection_response_payload(
        &self,
        response: &RatatoskrResponse,
    ) -> Result<ConnectionResponsePayload, ConnectionError> {
        if response.message_type != MessageType::ConnectionResponse as i32 {
            return Err(ConnectionError::InvalidMessageType {
                expected: "CONNECTION_RESPONSE".to_string(),
                actual: response.message_type,
            });
        }

        ConnectionResponsePayload::decode(&*response.payload)
            .map_err(ConnectionError::DecodeError)
    }
}

impl Default for ConnectionProtocol {
    fn default() -> Self {
        Self::new()
    }
}
