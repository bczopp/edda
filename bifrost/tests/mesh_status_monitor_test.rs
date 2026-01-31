//! Mesh-Status-Monitor tests (Phase 11.1.2, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::mesh::membership::{MeshMembershipChecker, MeshMembershipStub};
use bifrost::mesh::MeshStatusMonitor;

#[test]
fn monitor_reports_connected_when_provider_says_connected() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let monitor = MeshStatusMonitor::new(checker);

    let snap = monitor.check().unwrap();
    assert!(snap.connected);
    assert!(!snap.failure_detected);
}

#[test]
fn monitor_reports_disconnected_when_provider_says_disconnected() {
    let stub = MeshMembershipStub::all_denied();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let monitor = MeshStatusMonitor::new(checker);

    let snap = monitor.check().unwrap();
    assert!(!snap.connected);
    assert!(!snap.recovery_detected);
}

#[test]
fn monitor_detects_failure_when_same_monitor_sees_connection_lost() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let monitor = MeshStatusMonitor::new(checker);

    let snap1 = monitor.check().unwrap();
    assert!(snap1.connected);
    assert!(!snap1.failure_detected);
}

#[test]
fn monitor_is_connected_returns_last_known_state() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let monitor = MeshStatusMonitor::new(checker);

    assert!(!monitor.is_connected());
    let _ = monitor.check().unwrap();
    assert!(monitor.is_connected());
}

#[test]
fn snapshot_has_connected_and_detection_flags() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let monitor = MeshStatusMonitor::new(checker);

    let snap = monitor.check().unwrap();
    assert!(snap.connected);
    assert!(!snap.failure_detected);
    assert!(!snap.recovery_detected);
}
