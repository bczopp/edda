#[cfg(test)]
mod tests {
    use ratatoskr_example::messages::*;
    use ratatoskr_example::protocol::connection::ConnectionProtocol;
    use ratatoskr_example::proto::ratatoskr::*;

    #[test]
    fn test_connection_request_creation() {
        // Test: Create a connection request
        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        assert_eq!(request.message_type, MessageType::ConnectionRequest as i32);
        assert_eq!(request.device_id, "device-456");
        assert_eq!(request.user_id, "user-789");
    }

    #[test]
    fn test_connection_response_creation() {
        // Test: Create a connection response
        let response = RatatoskrResponse::new_connection_response(
            "req-123".to_string(),
            true,
            "session-456".to_string(),
            1234567890,
            "1.0.0".to_string(),
        );

        assert_eq!(response.message_type, MessageType::ConnectionResponse as i32);
        assert!(response.success);
    }

    #[test]
    fn test_connection_response_rejected() {
        // Test: Create a rejected connection response
        let response = RatatoskrResponse::new_connection_response(
            "req-123".to_string(),
            false,
            String::new(),
            0,
            "1.0.0".to_string(),
        );

        assert_eq!(response.message_type, MessageType::ConnectionResponse as i32);
        assert!(!response.success);
        assert!(!response.error_code.is_empty());
    }

    #[test]
    fn test_handshake_protocol() {
        // Test: Full handshake protocol flow
        let protocol = ConnectionProtocol::new();

        // Client sends connection request
        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        // Server processes request and creates response
        let response = protocol.create_connection_response(
            &request,
            true,
            "session-456".to_string(),
            1234567890,
            "1.0.0".to_string(),
        ).unwrap();

        assert_eq!(response.request_id, request.request_id);
        assert!(response.success);
    }

    #[test]
    fn test_handshake_protocol_rejection() {
        // Test: Handshake protocol with rejection
        let protocol = ConnectionProtocol::new();

        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "invalid-token".to_string(),
            "1.0.0".to_string(),
        );

        let response = protocol.create_connection_response(
            &request,
            false,
            String::new(),
            0,
            "1.0.0".to_string(),
        ).unwrap();

        assert_eq!(response.request_id, request.request_id);
        assert!(!response.success);
    }

    #[test]
    fn test_connection_request_payload_parsing() {
        // Test: Parse connection request payload
        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let protocol = ConnectionProtocol::new();
        let payload = protocol.parse_connection_request_payload(&request).unwrap();

        assert_eq!(payload.device_identity, "device-identity-123");
        assert_eq!(payload.authentication_token, "token-456");
        assert_eq!(payload.version, "1.0.0");
    }

    #[test]
    fn test_connection_response_payload_parsing() {
        // Test: Parse connection response payload
        let response = RatatoskrResponse::new_connection_response(
            "req-123".to_string(),
            true,
            "session-456".to_string(),
            1234567890,
            "1.0.0".to_string(),
        );

        let protocol = ConnectionProtocol::new();
        let payload = protocol.parse_connection_response_payload(&response).unwrap();

        assert!(payload.accepted);
        assert_eq!(payload.session_id, "session-456");
        assert_eq!(payload.expires_at, 1234567890);
        assert_eq!(payload.server_version, "1.0.0");
    }
}
