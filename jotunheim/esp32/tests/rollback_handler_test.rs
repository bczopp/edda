// RollbackHandler tests (Phase 7.3.1, TDD).

use jotunheim_esp32::ota::RollbackHandler;

#[test]
fn can_rollback_false_when_no_previous() {
    let h = RollbackHandler::new("v1".to_string());
    assert!(!h.can_rollback());
}

#[test]
fn can_rollback_true_after_set_previous() {
    let h = RollbackHandler::new("v2".to_string()).with_previous_version("v1".to_string());
    assert!(h.can_rollback());
}

#[test]
fn rollback_restores_previous_version() {
    let h = RollbackHandler::new("v2".to_string()).with_previous_version("v1".to_string());
    assert_eq!(h.current_version(), "v2");
    h.rollback().unwrap();
    assert_eq!(h.current_version(), "v1");
}

#[test]
fn rollback_fails_when_no_previous() {
    let mut h = RollbackHandler::new("v1".to_string());
    assert!(h.rollback().is_err());
}
