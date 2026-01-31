use ratatoskr::protocol::MessageValidator;
use ratatoskr::messages::*;
use ratatoskr::proto::ratatoskr::MessageType;

#[tokio::test]
async fn test_validate_request_schema_success() {
    let validator = MessageValidator::new();
    
    let mut request = RatatoskrRequest::new_business_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        b"payload".to_vec(),
    );
    request.nonce = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    request.signature = vec![0u8; 64];

    let result = validator.validate_request_schema(&request);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_request_schema_empty_request_id() {
    let validator = MessageValidator::new();
    
    let mut request = RatatoskrRequest::new_business_request(
        "".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        b"payload".to_vec(),
    );

    let result = validator.validate_request_schema(&request);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("request_id is required"));
}

#[tokio::test]
async fn test_validate_nonce_success() {
    let validator = MessageValidator::new();
    let nonce = vec![1, 2, 3, 4, 5, 6, 7, 8];
    
    let result = validator.validate_nonce(&nonce);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_nonce_too_short() {
    let validator = MessageValidator::new();
    let nonce = vec![1, 2, 3];
    
    let result = validator.validate_nonce(&nonce);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("nonce must be at least"));
}

#[tokio::test]
async fn test_validate_timestamp_success() {
    let validator = MessageValidator::new();
    let timestamp = chrono::Utc::now().timestamp();
    
    let result = validator.validate_timestamp(timestamp);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_timestamp_too_old() {
    let validator = MessageValidator::new();
    let old_timestamp = chrono::Utc::now().timestamp() - 400; // 400 seconds ago
    
    let result = validator.validate_timestamp(old_timestamp);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("timestamp is too old"));
}

#[tokio::test]
async fn test_validate_response_schema_success() {
    let validator = MessageValidator::new();
    
    let response = RatatoskrResponse::new_success(
        MessageType::BusinessRequest,
        "req-123".to_string(),
        b"payload".to_vec(),
    );

    let result = validator.validate_response_schema(&response);
    assert!(result.is_ok());
}
