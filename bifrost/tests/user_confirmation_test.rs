//! Tests for Phase 12.2.2: User Confirmation Manager (Allow/Deny, 2-3 confirmations, 5s interval).

use bifrost::guest::user_confirmation::{
    ConfirmationChoice, ConfirmationOutcome, UserConfirmationManager, UserConfirmationRequest,
};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

fn request_id() -> String {
    "req-1".to_string()
}

fn sample_request() -> UserConfirmationRequest {
    UserConfirmationRequest {
        request_id: request_id(),
        guest_device_id: "device-guest".to_string(),
        target_user_id: "user-1".to_string(),
        mesh_id: "main".to_string(),
    }
}

#[test]
fn deny_returns_denied_immediately() {
    let mgr = Arc::new(UserConfirmationManager::new(2, Duration::from_secs(5)));
    let (tx, _rx) = mpsc::channel();
    let req = sample_request();
    let id = mgr.start_request(req.clone(), tx).unwrap();
    let out = mgr.add_confirmation(&id, ConfirmationChoice::Deny).unwrap();
    assert!(matches!(out, ConfirmationOutcome::Denied));
}

#[test]
fn two_allows_with_interval_returns_allowed() {
    let mgr = Arc::new(UserConfirmationManager::new(2, Duration::from_millis(10)));
    let (tx, rx) = mpsc::channel();
    let req = sample_request();
    let id = mgr.start_request(req, tx).unwrap();
    // First Allow -> Pending
    let out1 = mgr.add_confirmation(&id, ConfirmationChoice::Allow).unwrap();
    assert!(matches!(out1, ConfirmationOutcome::Pending(_)));
    // Wait min interval
    std::thread::sleep(Duration::from_millis(15));
    let out2 = mgr.add_confirmation(&id, ConfirmationChoice::Allow).unwrap();
    assert!(matches!(out2, ConfirmationOutcome::Allowed));
    // Notify channel received the request
    assert!(rx.recv().is_ok());
}

#[test]
fn allow_before_interval_returns_error() {
    let mgr = Arc::new(UserConfirmationManager::new(2, Duration::from_secs(5)));
    let (tx, _rx) = mpsc::channel();
    let req = sample_request();
    let id = mgr.start_request(req, tx).unwrap();
    let _ = mgr.add_confirmation(&id, ConfirmationChoice::Allow).unwrap();
    // Second allow immediately (before 5s) should be rejected
    let out = mgr.add_confirmation(&id, ConfirmationChoice::Allow);
    assert!(out.is_err());
    assert!(matches!(out.unwrap_err(), bifrost::guest::ConfirmationError::TooSoon));
}

#[test]
fn unknown_request_id_returns_error() {
    let mgr = UserConfirmationManager::new(2, Duration::from_secs(5));
    let out = mgr.add_confirmation("unknown-id", ConfirmationChoice::Allow);
    assert!(out.is_err());
}

#[test]
fn three_confirmations_required_when_configured() {
    let mgr = Arc::new(UserConfirmationManager::new(3, Duration::from_millis(10)));
    let (tx, rx) = mpsc::channel();
    let req = sample_request();
    let id = mgr.start_request(req, tx).unwrap();
    let out1 = mgr.add_confirmation(&id, ConfirmationChoice::Allow).unwrap();
    assert!(matches!(out1, ConfirmationOutcome::Pending(_)));
    std::thread::sleep(Duration::from_millis(15));
    let out2 = mgr.add_confirmation(&id, ConfirmationChoice::Allow).unwrap();
    assert!(matches!(out2, ConfirmationOutcome::Pending(_)));
    std::thread::sleep(Duration::from_millis(15));
    let out3 = mgr.add_confirmation(&id, ConfirmationChoice::Allow).unwrap();
    assert!(matches!(out3, ConfirmationOutcome::Allowed));
    let _ = rx.recv();
}
