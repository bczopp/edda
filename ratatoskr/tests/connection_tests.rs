use ratatoskr::protocol::ConnectionProtocol;
use ratatoskr::messages::*;
use ratatoskr::proto::ratatoskr::*;

#[tokio::test]
async fn test_create_connection_response() {
    let protocol = ConnectionProtocol::new();
    
    let request = RatatoskrRequest::new_connection_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        "device-identity".to_string(),
        "auth-token".to_string(),
        "1.0.0".to_string(),
    );

    let response = protocol.create_connection_response(
        &request,
        true,
        "session-123".to_string(),
        1234567890,
        "1.0.0".to_string(),
    ).unwrap();

    assert_eq!(response.request_id, "req-123");
    assert_eq!(response.success, true);
    assert_eq!(response.message_type, MessageType::ConnectionResponse as i32);
}

#[tokio::test]
async fn test_create_connection_response_rejected() {
    let protocol = ConnectionProtocol::new();
    
    let request = RatatoskrRequest::new_connection_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        "device-identity".to_string(),
        "auth-token".to_string(),
        "1.0.0".to_string(),
    );

    let response = protocol.create_connection_response(
        &request,
        false,
        "".to_string(),
        0,
        "1.0.0".to_string(),
    ).unwrap();

    assert_eq!(response.success, false);
    assert!(!response.error_code.is_empty());
}

#[tokio::test]
async fn test_parse_connection_request_payload() {
    let protocol = ConnectionProtocol::new();
    
    let request = RatatoskrRequest::new_connection_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        "device-identity".to_string(),
        "auth-token".to_string(),
        "1.0.0".to_string(),
    );

    let payload = protocol.parse_connection_request_payload(&request).unwrap();
    
    assert_eq!(payload.device_identity, "device-identity");
    assert_eq!(payload.authentication_token, "auth-token");
    assert_eq!(payload.version, "1.0.0");
}

#[tokio::test]
async fn test_parse_connection_response_payload() {
    let protocol = ConnectionProtocol::new();
    
    let response = RatatoskrResponse::new_connection_response(
        "req-123".to_string(),
        true,
        "session-123".to_string(),
        1234567890,
        "1.0.0".to_string(),
    );

    let payload = protocol.parse_connection_response_payload(&response).unwrap();
    
    assert_eq!(payload.accepted, true);
    assert_eq!(payload.session_id, "session-123");
    assert_eq!(payload.expires_at, 1234567890);
    assert_eq!(payload.server_version, "1.0.0");
}

#[tokio::test]
async fn test_parse_connection_request_wrong_message_type() {
    let protocol = ConnectionProtocol::new();
    
    let request = RatatoskrRequest::new_business_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        b"payload".to_vec(),
    );

    let result = protocol.parse_connection_request_payload(&request);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("CONNECTION_REQUEST"));
}
