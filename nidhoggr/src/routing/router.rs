use crate::connection::ConnectionManager;
use crate::clients::ClientManager;
use ratatoskr::messages::RatatoskrRequest;
use ratatoskr::proto::ratatoskr::MessageType;
use std::sync::Arc;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoutingError {
    #[error("Unknown message type: {0}")]
    UnknownMessageType(i32),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Routing failed: {0}")]
    RoutingFailed(String),
    #[error("Client error: {0}")]
    ClientError(#[from] crate::clients::manager::ClientManagerError),
}

pub struct MessageRouter {
    connection_manager: Arc<ConnectionManager>,
    client_manager: Arc<ClientManager>,
}

impl MessageRouter {
    pub fn new(connection_manager: Arc<ConnectionManager>, client_manager: Arc<ClientManager>) -> Self {
        Self {
            connection_manager,
            client_manager,
        }
    }

    pub async fn route_message(&self, request: RatatoskrRequest) -> Result<Option<Vec<u8>>, RoutingError> {
        match request.message_type {
            x if x == MessageType::ConnectionRequest as i32 => {
                // Connection requests are handled by the WebSocket server
                Ok(None)
            }
            x if x == MessageType::ConnectionResponse as i32 => {
                // Connection responses are handled by the WebSocket server
                Ok(None)
            }
            x if x == MessageType::BusinessRequest as i32 => {
                // Route to Nornen for business logic
                self.route_to_nornen(request).await
            }
            x if x == MessageType::Heartbeat as i32 => {
                // Heartbeats are handled by the WebSocket server
                Ok(None)
            }
            x if x == MessageType::Disconnect as i32 => {
                // Disconnects are handled by the WebSocket server
                Ok(None)
            }
            x if x == MessageType::Error as i32 => {
                // Errors are handled by the WebSocket server
                Ok(None)
            }
            _ => Err(RoutingError::UnknownMessageType(request.message_type)),
        }
    }

    async fn route_to_nornen(&self, request: RatatoskrRequest) -> Result<Option<Vec<u8>>, RoutingError> {
        // Parse the request payload to extract routing information
        // For now, we'll route all business requests to Nornen's coordinate_request
        // In a real implementation, we would parse the payload to determine the specific service
        
        let mut context = HashMap::new();
        context.insert("device_id".to_string(), request.device_id.clone());
        context.insert("user_id".to_string(), request.user_id.clone());
        context.insert("request_id".to_string(), request.request_id.clone());
        
        // Try to parse payload as JSON to extract request_type
        let request_type = match serde_json::from_slice::<serde_json::Value>(&request.payload) {
            Ok(json) => {
                json.get("request_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("business_request")
                    .to_string()
            }
            Err(_) => "business_request".to_string(),
        };
        
        let mut nornen_client = self.client_manager.nornen().lock().await;
        let response = nornen_client
            .coordinate_request(
                request.request_id,
                request_type,
                context,
            )
            .await
            .map_err(|e| RoutingError::ServiceUnavailable(format!("Nornen error: {}", e)))?;
        
        // Serialize response to bytes
        let response_bytes = serde_json::to_vec(&response)
            .map_err(|e| RoutingError::RoutingFailed(format!("Failed to serialize response: {}", e)))?;
        
        Ok(Some(response_bytes))
    }
}
