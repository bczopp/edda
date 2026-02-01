//! Security Test Suite (Phase 20.3.1).
//! Aggregates: WebSocket-Security, Unauthorized-Access-Prevention,
//! Connection-Authentication-Tests, Message-Validation-Tests.
//! See also: websocket_security_test.rs, connection_blocker_test.rs,
//! challenge_*_test.rs, message_validator_test.rs.
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::connection::{
    ConnectionBlocker, ConnectionStatus, ConnectionStatusTracker,
};
use bifrost::heimdall::{
    ConnectionValidationResponse, CrossUserConnectionBlocker,
    ValidationOutcome, ValidationResponseHandler, UserIdentityVerifier,
};
use bifrost::message::{BifrostMessage, MessageType, MessageValidator, ValidationError};
use bifrost::security::challenge::ChallengeRequestHandler;
use bifrost::security::KeyGenerator;
use std::sync::Arc;

// --- WebSocket-Security: validation DENY must not grant access ---

#[test]
fn security_suite_websocket_validation_denied_no_access() {
    let resp = ConnectionValidationResponse::deny();
    let outcome = ValidationResponseHandler.handle_response(&resp).unwrap();
    match outcome {
        ValidationOutcome::Allowed { .. } => panic!("DENY must not yield Allowed"),
        ValidationOutcome::Denied => {}
    }
}

#[test]
fn security_suite_websocket_validation_allowed_grants_access() {
    let resp = ConnectionValidationResponse::allow(Some("token-1".to_string()));
    let outcome = ValidationResponseHandler.handle_response(&resp).unwrap();
    match &outcome {
        ValidationOutcome::Allowed { validation_token } => {
            assert_eq!(validation_token.as_deref(), Some("token-1"));
        }
        ValidationOutcome::Denied => panic!("ALLOW must yield Allowed"),
    }
}

// --- Unauthorized-Access-Prevention: cross-user blocked, threat blocks ---

#[test]
fn security_suite_unauthorized_cross_user_blocked() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(!blocker.allow_direct_connection("user-a", "user-b"));
    assert!(blocker.requires_relay("user-a", "user-b"));
}

#[test]
fn security_suite_threat_blocks_connection_and_revokes() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let revoked = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    struct Revoker(Arc<std::sync::Mutex<Vec<String>>>);
    impl bifrost::connection::TokenRevoker for Revoker {
        fn revoke_connection(&self, connection_id: &str) {
            self.0.lock().unwrap().push(connection_id.to_string());
        }
    }
    let blocker = ConnectionBlocker::new(
        Arc::clone(&tracker),
        Some(Arc::new(Revoker(Arc::clone(&revoked)))),
        None,
        None,
    );
    tracker.update_status("conn-1", ConnectionStatus::Active);
    blocker.block_connection("conn-1", "threat");
    assert_eq!(tracker.get_status("conn-1"), Some(ConnectionStatus::Blocked));
    assert_eq!(revoked.lock().unwrap().as_slice(), ["conn-1"]);
}

// --- Connection-Authentication-Tests: challenge request/validation ---

#[test]
fn security_suite_authentication_challenge_request_valid() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeRequestHandler::build_challenge_request(
        "dev-1",
        "target-1",
        kp.public_key(),
        kp.secret_key(),
    )
    .unwrap();
    assert_eq!(msg.message_type, MessageType::ChallengeRequest);
    assert!(msg.payload.get("public_key").and_then(|v| v.as_str()).unwrap_or("").len() > 0);
    assert!(msg.payload.get("signature").and_then(|v| v.as_str()).unwrap_or("").len() > 0);
}

// --- Message-Validation-Tests: invalid messages rejected ---

#[test]
fn security_suite_message_validation_rejects_empty_message_id() {
    let validator = MessageValidator::new(1024, false);
    let msg = BifrostMessage {
        message_id: String::new(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };
    let res = validator.validate(&msg);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), ValidationError::InvalidFormat(_)));
}

#[test]
fn security_suite_message_validation_rejects_oversized_payload() {
    let validator = MessageValidator::new(10, false);
    let msg = BifrostMessage {
        message_id: "m1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({"x": "too long"}),
        timestamp: 0,
        protocol_version: None,
    };
    let res = validator.validate(&msg);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), ValidationError::PayloadTooLarge));
}

#[test]
fn security_suite_message_validation_sanitize_removes_control_chars() {
    let validator = MessageValidator::new(1024, false);
    let msg = BifrostMessage {
        message_id: "id\x00\x01".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };
    let sanitized = validator.sanitize(msg);
    assert!(!sanitized.message_id.contains('\x00'));
}
