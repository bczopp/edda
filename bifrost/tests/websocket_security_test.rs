//! Tests for Phase 15.3.2: WebSocket Security – Unauthorized-Access-Prevention.
//! Ensures validation deny leads to no access, cross-user direct connections are blocked, and threat triggers block.

use bifrost::connection::{
    ConnectionBlocker, ConnectionStatus, ConnectionStatusTracker,
};
use bifrost::heimdall::{
    ConnectionValidationResponse, CrossUserConnectionBlocker,
    ValidationOutcome, ValidationResponseHandler, UserIdentityVerifier,
};
use std::sync::Arc;

// --- Unauthorized access: validation DENY must not grant access ---

#[test]
fn validation_denied_prevents_connection_access() {
    let resp = ConnectionValidationResponse::deny();
    let handler = ValidationResponseHandler;
    let outcome = handler.handle_response(&resp).unwrap();
    match outcome {
        ValidationOutcome::Allowed { .. } => panic!("denied response must not yield Allowed"),
        ValidationOutcome::Denied => {}
    }
    // Policy: when Denied, connection must not be accepted; no validation_token.
}

#[test]
fn validation_allowed_with_token_grants_connection_access() {
    let resp = ConnectionValidationResponse::allow(Some("token-1".to_string()));
    let handler = ValidationResponseHandler;
    let outcome = handler.handle_response(&resp).unwrap();
    match &outcome {
        ValidationOutcome::Allowed { validation_token } => {
            assert!(validation_token.as_deref() == Some("token-1"));
        }
        ValidationOutcome::Denied => panic!("allowed response must yield Allowed"),
    }
}

// --- Unauthorized data access: cross-user direct connection must be blocked ---

#[test]
fn cross_user_direct_connection_blocked() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(
        !blocker.allow_direct_connection("user-a", "user-b"),
        "direct connection between different users must be blocked"
    );
    assert!(
        blocker.requires_relay("user-a", "user-b"),
        "cross-user traffic must require relay"
    );
}

#[test]
fn same_user_direct_connection_allowed() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(
        blocker.allow_direct_connection("user-a", "user-a"),
        "same user may have direct connection"
    );
    assert!(!blocker.requires_relay("user-a", "user-a"));
}

// --- Threat: connection must be blocked and token revoked ---

#[test]
fn connection_blocked_on_threat_revokes_access() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let revoked = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    struct Revoker(std::sync::Arc<std::sync::Mutex<Vec<String>>>);
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
    blocker.block_connection("conn-1", "threat-detected");
    assert_eq!(tracker.get_status("conn-1"), Some(ConnectionStatus::Blocked));
    assert_eq!(revoked.lock().unwrap().as_slice(), ["conn-1"]);
}
