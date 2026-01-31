use crate::proto::ratatoskr::*;
use chrono::Utc;
use prost::Message;

/// Helper functions for creating RatatoskrResponse messages
impl RatatoskrResponse {
    /// Create a successful response
    pub fn new_success(
        message_type: MessageType,
        request_id: String,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            message_type: message_type as i32,
            request_id,
            timestamp: Utc::now().timestamp(),
            success: true,
            error_code: String::new(),
            error_message: String::new(),
            payload,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a connection response
    pub fn new_connection_response(
        request_id: String,
        accepted: bool,
        session_id: String,
        expires_at: i64,
        server_version: String,
    ) -> Self {
        let payload = ConnectionResponsePayload {
            accepted,
            session_id,
            expires_at,
            server_version,
        };

        Self {
            message_type: MessageType::ConnectionResponse as i32,
            request_id,
            timestamp: Utc::now().timestamp(),
            success: accepted,
            error_code: if accepted {
                String::new()
            } else {
                "CONNECTION_REJECTED".to_string()
            },
            error_message: if accepted {
                String::new()
            } else {
                "Connection was rejected".to_string()
            },
            payload: payload.encode_to_vec(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create an error response
    pub fn new_error(
        message_type: MessageType,
        request_id: String,
        error_code: String,
        error_message: String,
    ) -> Self {
        Self {
            message_type: message_type as i32,
            request_id,
            timestamp: Utc::now().timestamp(),
            success: false,
            error_code,
            error_message,
            payload: vec![],
            metadata: std::collections::HashMap::new(),
        }
    }
}
