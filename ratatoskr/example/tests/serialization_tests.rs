#[cfg(test)]
mod tests {
    use ratatoskr_example::messages::*;
    use ratatoskr_example::protocol::serializer::MessageSerializer;
    use ratatoskr_example::proto::ratatoskr::*;

    #[test]
    fn test_serialize_ratatoskr_request() {
        // Test: Serialize a RatatoskrRequest
        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let serializer = MessageSerializer::new();
        let serialized = serializer.serialize_request(&request).unwrap();

        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_deserialize_ratatoskr_request() {
        // Test: Deserialize a RatatoskrRequest
        let request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let serializer = MessageSerializer::new();
        let serialized = serializer.serialize_request(&request).unwrap();
        let deserialized = serializer.deserialize_request(&serialized).unwrap();

        assert_eq!(deserialized.request_id, request.request_id);
        assert_eq!(deserialized.device_id, request.device_id);
        assert_eq!(deserialized.user_id, request.user_id);
        assert_eq!(deserialized.message_type, request.message_type);
    }

    #[test]
    fn test_serialize_ratatoskr_response() {
        // Test: Serialize a RatatoskrResponse
        let response = RatatoskrResponse::new_success(
            MessageType::ConnectionResponse,
            "req-123".to_string(),
            vec![1, 2, 3],
        );

        let serializer = MessageSerializer::new();
        let serialized = serializer.serialize_response(&response).unwrap();

        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_deserialize_ratatoskr_response() {
        // Test: Deserialize a RatatoskrResponse
        let response = RatatoskrResponse::new_success(
            MessageType::ConnectionResponse,
            "req-123".to_string(),
            vec![1, 2, 3],
        );

        let serializer = MessageSerializer::new();
        let serialized = serializer.serialize_response(&response).unwrap();
        let deserialized = serializer.deserialize_response(&serialized).unwrap();

        assert_eq!(deserialized.request_id, response.request_id);
        assert_eq!(deserialized.success, response.success);
        assert_eq!(deserialized.message_type, response.message_type);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        // Test: Full roundtrip serialization/deserialization
        let request = RatatoskrRequest::new_business_request(
            "req-456".to_string(),
            "device-789".to_string(),
            "user-012".to_string(),
            vec![10, 20, 30],
        );

        let serializer = MessageSerializer::new();
        let serialized = serializer.serialize_request(&request).unwrap();
        let deserialized = serializer.deserialize_request(&serialized).unwrap();

        assert_eq!(deserialized.request_id, request.request_id);
        assert_eq!(deserialized.device_id, request.device_id);
        assert_eq!(deserialized.user_id, request.user_id);
        assert_eq!(deserialized.payload, request.payload);
    }

    #[test]
    fn test_serialize_invalid_data() {
        // Test: Deserialize invalid data should fail
        let serializer = MessageSerializer::new();
        let invalid_data = vec![0, 1, 2, 3, 4, 5];

        let result = serializer.deserialize_request(&invalid_data);
        assert!(result.is_err());
    }
}
