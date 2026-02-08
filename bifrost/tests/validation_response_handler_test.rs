//! Tests for Phase 5.2.2: ValidationResponseHandler (process ConnectionValidationResponse, status ALLOW/DENY, extract token).

use bifrost::heimdall::{
    ConnectionValidationResponse, ValidationOutcome, ValidationResponseHandler,
};

#[test]
fn handle_response_allow_with_token() {
    let resp = ConnectionValidationResponse::allow(Some("token-1".to_string()));
    let handler = ValidationResponseHandler;
    let outcome = handler.handle_response(&resp).unwrap();
    match outcome {
        ValidationOutcome::Allowed { validation_token } => {
            assert_eq!(validation_token.as_deref(), Some("token-1"));
        }
        ValidationOutcome::Denied => panic!("expected Allowed"),
    }
}

#[test]
fn handle_response_allow_without_token() {
    let resp = ConnectionValidationResponse::allow(None);
    let handler = ValidationResponseHandler;
    let outcome = handler.handle_response(&resp).unwrap();
    match outcome {
        ValidationOutcome::Allowed { validation_token } => {
            assert!(validation_token.is_none());
        }
        ValidationOutcome::Denied => panic!("expected Allowed"),
    }
}

#[test]
fn handle_response_deny() {
    let resp = ConnectionValidationResponse::deny();
    let handler = ValidationResponseHandler;
    let outcome = handler.handle_response(&resp).unwrap();
    match outcome {
        ValidationOutcome::Allowed { .. } => panic!("expected Denied"),
        ValidationOutcome::Denied => {}
    }
}

#[test]
fn handler_is_unit_like() {
    let _ = ValidationResponseHandler;
}
