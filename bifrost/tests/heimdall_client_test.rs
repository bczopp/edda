//! Tests for Phase 5.1.1: HeimdallClient (connection validation, retry, stub).

use bifrost::heimdall::{ConnectionValidationRequest, HeimdallClient, HeimdallStub};
use std::sync::Arc;

#[tokio::test]
async fn validate_connection_returns_response_from_stub() {
    let stub = HeimdallStub::allow();
    let client = HeimdallClient::new(Arc::new(stub), 1);
    let req = ConnectionValidationRequest {
        user_id: "user-1".to_string(),
        device_id: "dev-1".to_string(),
        timestamp: None,
        request_signature: None,
    };
    let result = client.validate_connection(&req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(resp.allowed());
    assert_eq!(resp.validation_token(), Some("token-1"));
}

#[tokio::test]
async fn validate_connection_deny_from_stub() {
    let stub = HeimdallStub::deny();
    let client = HeimdallClient::new(Arc::new(stub), 1);
    let req = ConnectionValidationRequest {
        user_id: "u".to_string(),
        device_id: "d".to_string(),
        timestamp: None,
        request_signature: None,
    };
    let result = client.validate_connection(&req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(!resp.allowed());
}

#[tokio::test]
async fn validate_connection_retries_on_transient_error() {
    let stub = HeimdallStub::fail_then_allow(1);
    let client = HeimdallClient::new(Arc::new(stub), 2);
    let req = ConnectionValidationRequest {
        user_id: "u".to_string(),
        device_id: "d".to_string(),
        timestamp: None,
        request_signature: None,
    };
    let result = client.validate_connection(&req).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn validate_connection_fails_after_max_retries() {
    let stub = HeimdallStub::always_fail();
    let client = HeimdallClient::new(Arc::new(stub), 2);
    let req = ConnectionValidationRequest {
        user_id: "u".to_string(),
        device_id: "d".to_string(),
        timestamp: None,
        request_signature: None,
    };
    let result = client.validate_connection(&req).await;
    assert!(result.is_err());
}

#[test]
fn connection_validation_request_has_user_and_device() {
    let req = ConnectionValidationRequest {
        user_id: "u1".to_string(),
        device_id: "d1".to_string(),
        timestamp: None,
        request_signature: None,
    };
    assert_eq!(req.user_id, "u1");
    assert_eq!(req.device_id, "d1");
}
