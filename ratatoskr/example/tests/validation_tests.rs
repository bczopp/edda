#[cfg(test)]
mod tests {
    use ratatoskr_example::messages::*;
    use ratatoskr_example::protocol::validator::MessageValidator;
    use ratatoskr_example::proto::ratatoskr::*;

    #[test]
    fn test_validate_request_schema() {
        // Test: Validate a valid request schema
        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let validator = MessageValidator::new();
        let result = validator.validate_request_schema(&request);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_schema_missing_fields() {
        // Test: Validate request with missing required fields
        let mut request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        // Clear required field
        request.request_id.clear();

        let validator = MessageValidator::new();
        let result = validator.validate_request_schema(&request);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_nonce() {
        // Test: Validate request with nonce
        let mut request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        // Add nonce
        request.nonce = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let validator = MessageValidator::new();
        let result = validator.validate_nonce(&request.nonce);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_nonce_empty() {
        // Test: Validate request with empty nonce should fail
        let validator = MessageValidator::new();
        let result = validator.validate_nonce(&vec![]);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_nonce_too_short() {
        // Test: Validate request with nonce that's too short
        let validator = MessageValidator::new();
        let result = validator.validate_nonce(&vec![1, 2, 3]);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_signature() {
        // Test: Validate request with signature
        let mut request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        // Add signature (mock - in real implementation this would be a real signature)
        request.signature = vec![1; 64]; // 64 bytes for Ed25519 signature

        let validator = MessageValidator::new();
        let result = validator.validate_signature_length(&request.signature);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_signature_empty() {
        // Test: Validate request with empty signature should fail
        let validator = MessageValidator::new();
        let result = validator.validate_signature_length(&vec![]);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_timestamp() {
        // Test: Validate request with valid timestamp
        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let validator = MessageValidator::new();
        let result = validator.validate_timestamp(request.timestamp);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_timestamp_too_old() {
        // Test: Validate request with timestamp that's too old
        let validator = MessageValidator::new();
        let old_timestamp = 1000000000; // Very old timestamp
        let result = validator.validate_timestamp(old_timestamp);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_response_schema() {
        // Test: Validate a valid response schema
        let response = RatatoskrResponse::new_success(
            MessageType::ConnectionResponse,
            "req-123".to_string(),
            vec![1, 2, 3],
        );

        let validator = MessageValidator::new();
        let result = validator.validate_response_schema(&response);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_response_schema_missing_request_id() {
        // Test: Validate response with missing request_id
        let mut response = RatatoskrResponse::new_success(
            MessageType::ConnectionResponse,
            "req-123".to_string(),
            vec![1, 2, 3],
        );

        response.request_id.clear();

        let validator = MessageValidator::new();
        let result = validator.validate_response_schema(&response);

        assert!(result.is_err());
    }
}
