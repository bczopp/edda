//! Mesh-Membership-Checker tests (Phase 11.1.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::mesh::membership::{MeshMembershipChecker, MeshMembershipStub};

#[test]
fn stub_all_true_returns_true_for_user_and_device() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));

    assert!(checker.is_user_in_mesh("user1").unwrap());
    assert!(checker.is_device_in_mesh("device1").unwrap());
    assert!(checker.is_mesh_connected().unwrap());
}

#[test]
fn stub_all_denied_returns_false_for_user_and_device() {
    let stub = MeshMembershipStub::all_denied();
    let checker = MeshMembershipChecker::new(Box::new(stub));

    assert!(!checker.is_user_in_mesh("user1").unwrap());
    assert!(!checker.is_device_in_mesh("device1").unwrap());
    assert!(!checker.is_mesh_connected().unwrap());
}

#[test]
fn stub_custom_user_allowed() {
    let stub = MeshMembershipStub::custom(true, false, false);
    let checker = MeshMembershipChecker::new(Box::new(stub));

    assert!(checker.is_user_in_mesh("any").unwrap());
    assert!(!checker.is_device_in_mesh("any").unwrap());
    assert!(!checker.is_mesh_connected().unwrap());
}

#[test]
fn user_mesh_membership_implies_user_check() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));

    assert!(checker.is_user_in_mesh("u1").unwrap());
}
