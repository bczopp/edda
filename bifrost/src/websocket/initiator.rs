//! Connection Initiator (Phase 7.1.2). Send connection request; process response; connection establishment.

use crate::message::{BifrostMessage, MessageHandler, MessageType};
use uuid::Uuid;

/// Builds CONNECTION_REQUEST; parses CONNECTION_RESPONSE; connection establishment is done by caller (send/recv).
pub struct ConnectionInitiator;

impl ConnectionInitiator {
    pub fn new() -> Self {
        Self
    }

    /// Builds a CONNECTION_REQUEST message (caller sends over stream).
    pub fn build_connection_request(
        device_id: &str,
        user_id: &str,
        target_device_id: &str,
    ) -> BifrostMessage {
        BifrostMessage {
            message_id: format!("req-{}", Uuid::new_v4()),
            message_type: MessageType::ConnectionRequest,
            source_device_id: device_id.to_string(),
            target_device_id: target_device_id.to_string(),
            payload: serde_json::json!({ "user_id": user_id }),
            timestamp: chrono::Utc::now().timestamp(),
            protocol_version: Some(1),
        }
    }

    /// Parses text as BifrostMessage; returns Ok(msg) if message_type is ConnectionResponse.
    pub fn parse_connection_response(
        text: &str,
    ) -> Result<BifrostMessage, Box<dyn std::error::Error + Send + Sync>> {
        let msg = MessageHandler::parse_message(text)?;
        if msg.message_type != MessageType::ConnectionResponse {
            return Err(format!("expected CONNECTION_RESPONSE, got {:?}", msg.message_type).into());
        }
        Ok(msg)
    }
}

impl Default for ConnectionInitiator {
    fn default() -> Self {
        Self::new()
    }
}
