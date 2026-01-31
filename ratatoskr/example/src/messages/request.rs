use crate::proto::ratatoskr::*;
use chrono::Utc;
use prost::Message;

/// Helper functions for creating RatatoskrRequest messages
impl RatatoskrRequest {
    /// Create a new connection request
    pub fn new_connection_request(
        request_id: String,
        device_id: String,
        user_id: String,
        device_identity: String,
        authentication_token: String,
        version: String,
    ) -> Self {
        let payload = ConnectionRequestPayload {
            device_identity,
            authentication_token,
            version,
        };

        Self {
            message_type: MessageType::ConnectionRequest as i32,
            request_id,
            device_id,
            user_id,
            timestamp: Utc::now().timestamp(),
            nonce: vec![],
            signature: vec![],
            payload: payload.encode_to_vec(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a new business request
    /// The payload is generic and will be interpreted by the receiving service (Nornen, Heidrun, etc.)
    pub fn new_business_request(
        request_id: String,
        device_id: String,
        user_id: String,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            message_type: MessageType::BusinessRequest as i32,
            request_id,
            device_id,
            user_id,
            timestamp: Utc::now().timestamp(),
            nonce: vec![],
            signature: vec![],
            payload,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a heartbeat request
    pub fn new_heartbeat(request_id: String, device_id: String, user_id: String) -> Self {
        Self {
            message_type: MessageType::Heartbeat as i32,
            request_id,
            device_id,
            user_id,
            timestamp: Utc::now().timestamp(),
            nonce: vec![],
            signature: vec![],
            payload: vec![],
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a disconnect request
    pub fn new_disconnect(request_id: String, device_id: String, user_id: String) -> Self {
        Self {
            message_type: MessageType::Disconnect as i32,
            request_id,
            device_id,
            user_id,
            timestamp: Utc::now().timestamp(),
            nonce: vec![],
            signature: vec![],
            payload: vec![],
            metadata: std::collections::HashMap::new(),
        }
    }
}
