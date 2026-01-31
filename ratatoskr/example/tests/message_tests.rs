#[cfg(test)]
mod tests {
    use ratatoskr_example::messages::*;
    use ratatoskr_example::proto::ratatoskr::*;

    #[test]
    fn test_ratatoskr_request_creation() {
        // Test: Create a RatatoskrRequest with all required fields
        let request = RatatoskrRequest {
            message_type: MessageType::ConnectionRequest as i32,
            request_id: "req-123".to_string(),
            device_id: "device-456".to_string(),
            user_id: "user-789".to_string(),
            timestamp: 1234567890,
            nonce: vec![1, 2, 3, 4],
            signature: vec![5, 6, 7, 8],
            payload: vec![9, 10, 11, 12],
            metadata: std::collections::HashMap::new(),
        };

        assert_eq!(request.request_id, "req-123");
        assert_eq!(request.device_id, "device-456");
        assert_eq!(request.user_id, "user-789");
        assert_eq!(request.timestamp, 1234567890);
    }

    #[test]
    fn test_ratatoskr_response_creation() {
        // Test: Create a RatatoskrResponse with success
        let response = RatatoskrResponse {
            message_type: MessageType::ConnectionResponse as i32,
            request_id: "req-123".to_string(),
            timestamp: 1234567890,
            success: true,
            error_code: String::new(),
            error_message: String::new(),
            payload: vec![1, 2, 3],
            metadata: std::collections::HashMap::new(),
        };

        assert_eq!(response.request_id, "req-123");
        assert!(response.success);
    }

    #[test]
    fn test_ratatoskr_response_with_error() {
        // Test: Create a RatatoskrResponse with error
        let response = RatatoskrResponse {
            message_type: MessageType::Error as i32,
            request_id: "req-123".to_string(),
            timestamp: 1234567890,
            success: false,
            error_code: "INVALID_REQUEST".to_string(),
            error_message: "Request validation failed".to_string(),
            payload: vec![],
            metadata: std::collections::HashMap::new(),
        };

        assert!(!response.success);
        assert_eq!(response.error_code, "INVALID_REQUEST");
        assert_eq!(response.error_message, "Request validation failed");
    }

    #[test]
    fn test_message_type_enum() {
        // Test: Verify all message types are defined
        assert_eq!(MessageType::Unknown as i32, 0);
        assert_eq!(MessageType::ConnectionRequest as i32, 1);
        assert_eq!(MessageType::ConnectionResponse as i32, 2);
        assert_eq!(MessageType::BusinessRequest as i32, 3);
        assert_eq!(MessageType::Heartbeat as i32, 4);
        assert_eq!(MessageType::Disconnect as i32, 5);
        assert_eq!(MessageType::Error as i32, 6);
    }

    #[test]
    fn test_connection_request_payload() {
        // Test: Create ConnectionRequestPayload
        let payload = ConnectionRequestPayload {
            device_identity: "device-identity-123".to_string(),
            authentication_token: "token-456".to_string(),
            version: "1.0.0".to_string(),
        };

        assert_eq!(payload.device_identity, "device-identity-123");
        assert_eq!(payload.authentication_token, "token-456");
        assert_eq!(payload.version, "1.0.0");
    }

    #[test]
    fn test_connection_response_payload() {
        // Test: Create ConnectionResponsePayload
        let payload = ConnectionResponsePayload {
            accepted: true,
            session_id: "session-123".to_string(),
            expires_at: 1234567890,
            server_version: "1.0.0".to_string(),
        };

        assert!(payload.accepted);
        assert_eq!(payload.session_id, "session-123");
        assert_eq!(payload.expires_at, 1234567890);
    }
}
