//! Mesh-based Connection Lifecycle tests (Phase 11.2.2, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::mesh::lifecycle::{LifecycleAction, MeshConnectionLifecycleManager};
use bifrost::mesh::membership::{MeshMembershipChecker, MeshMembershipStub};
use bifrost::mesh::MeshStatusMonitor;

#[test]
fn tick_when_no_change_returns_none() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let monitor = MeshStatusMonitor::new(checker);
    let manager = MeshConnectionLifecycleManager::new(monitor);

    let _ = manager.tick().unwrap();
    let action = manager.tick().unwrap();
    assert!(action.is_none());
}

#[test]
fn tick_when_failure_detected_returns_mesh_failure() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub.clone()));
    let monitor = MeshStatusMonitor::new(checker);
    let manager = MeshConnectionLifecycleManager::new(monitor);

    let _ = manager.tick().unwrap();
    stub.set_connected(false);
    let action = manager.tick().unwrap();
    assert_eq!(action, Some(LifecycleAction::MeshFailure));
}

#[test]
fn tick_when_recovery_detected_returns_mesh_recovery() {
    let stub = MeshMembershipStub::all_denied();
    let checker = MeshMembershipChecker::new(Box::new(stub.clone()));
    let monitor = MeshStatusMonitor::new(checker);
    let manager = MeshConnectionLifecycleManager::new(monitor);

    let _ = manager.tick().unwrap();
    stub.set_connected(true);
    let action = manager.tick().unwrap();
    assert_eq!(action, Some(LifecycleAction::MeshRecovery));
}
