use ratatoskr::protocol::MessageSerializer;
use ratatoskr::messages::*;
use ratatoskr::proto::ratatoskr::MessageType;

#[tokio::test]
async fn test_serialize_deserialize_request() {
    let serializer = MessageSerializer::new();
    
    let request = RatatoskrRequest::new_business_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        b"test payload".to_vec(),
    );

    let serialized = serializer.serialize_request(&request).unwrap();
    let deserialized = serializer.deserialize_request(&serialized).unwrap();

    assert_eq!(request.request_id, deserialized.request_id);
    assert_eq!(request.device_id, deserialized.device_id);
    assert_eq!(request.user_id, deserialized.user_id);
    assert_eq!(request.message_type, deserialized.message_type);
    assert_eq!(request.payload, deserialized.payload);
}

#[tokio::test]
async fn test_serialize_deserialize_response() {
    let serializer = MessageSerializer::new();
    
    let response = RatatoskrResponse::new_success(
        MessageType::BusinessRequest,
        "req-123".to_string(),
        b"response payload".to_vec(),
    );

    let serialized = serializer.serialize_response(&response).unwrap();
    let deserialized = serializer.deserialize_response(&serialized).unwrap();

    assert_eq!(response.request_id, deserialized.request_id);
    assert_eq!(response.success, deserialized.success);
    assert_eq!(response.payload, deserialized.payload);
    assert_eq!(response.message_type, deserialized.message_type);
}

#[tokio::test]
async fn test_serialize_connection_request() {
    let serializer = MessageSerializer::new();
    
    let request = RatatoskrRequest::new_connection_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        "device-identity".to_string(),
        "auth-token".to_string(),
        "1.0.0".to_string(),
    );

    let serialized = serializer.serialize_request(&request).unwrap();
    let deserialized = serializer.deserialize_request(&serialized).unwrap();

    assert_eq!(request.request_id, deserialized.request_id);
    assert_eq!(request.message_type, MessageType::ConnectionRequest as i32);
}
