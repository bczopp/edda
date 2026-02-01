//! Tests for Phase 5.2.1: ConnectionValidationHandler (send request, sign with device key, process response).

use bifrost::heimdall::{ConnectionValidationHandler, HeimdallClient, HeimdallStub};
use bifrost::security::KeyGenerator;
use std::sync::Arc;

#[tokio::test]
async fn validate_connection_sends_signed_request_and_returns_response() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let stub = HeimdallStub::allow();
    let client = HeimdallClient::new(Arc::new(stub), 1);
    let handler = ConnectionValidationHandler::new(client);
    let result = handler
        .validate_connection("user-1", "dev-1", kp.secret_key())
        .await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(resp.allowed());
    assert_eq!(resp.validation_token(), Some("token-1"));
}

#[tokio::test]
async fn validate_connection_deny_from_heimdall() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let stub = HeimdallStub::deny();
    let client = HeimdallClient::new(Arc::new(stub), 1);
    let handler = ConnectionValidationHandler::new(client);
    let result = handler
        .validate_connection("u", "d", kp.secret_key())
        .await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(!resp.allowed());
}

#[tokio::test]
async fn request_includes_signature_when_signed() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let req = ConnectionValidationHandler::build_signed_request(
        "user-1",
        "dev-1",
        kp.secret_key(),
    )
    .unwrap();
    assert_eq!(req.user_id, "user-1");
    assert_eq!(req.device_id, "dev-1");
    assert!(req.request_signature().is_some());
    assert!(!req.request_signature().unwrap().is_empty());
}

#[test]
fn handler_new_accepts_client() {
    let stub = HeimdallStub::allow();
    let client = HeimdallClient::new(Arc::new(stub), 1);
    let _ = ConnectionValidationHandler::new(client);
}
