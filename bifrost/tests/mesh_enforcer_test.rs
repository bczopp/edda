//! Mesh Connection Enforcer tests (Phase 11.2.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::mesh::enforcer::MeshConnectionEnforcer;
use bifrost::mesh::membership::{MeshMembershipChecker, MeshMembershipStub};

#[test]
fn enforcer_allows_connection_when_user_and_device_in_mesh() {
    let stub = MeshMembershipStub::all_allowed();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let enforcer = MeshConnectionEnforcer::new(checker);

    let result = enforcer.allow_connection("user1", "device1");
    assert!(result.is_ok());
}

#[test]
fn enforcer_blocks_connection_when_user_not_in_mesh() {
    let stub = MeshMembershipStub::custom(false, true, true);
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let enforcer = MeshConnectionEnforcer::new(checker);

    let result = enforcer.allow_connection("user1", "device1");
    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.client_message().to_lowercase();
    assert!(msg.contains("mesh") || msg.contains("denied"));
}

#[test]
fn enforcer_blocks_connection_when_device_not_in_mesh() {
    let stub = MeshMembershipStub::custom(true, false, true);
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let enforcer = MeshConnectionEnforcer::new(checker);

    let result = enforcer.allow_connection("user1", "device1");
    assert!(result.is_err());
}

#[test]
fn enforcer_error_has_client_message() {
    let stub = MeshMembershipStub::all_denied();
    let checker = MeshMembershipChecker::new(Box::new(stub));
    let enforcer = MeshConnectionEnforcer::new(checker);

    let err = enforcer.allow_connection("u", "d").unwrap_err();
    let msg = err.client_message();
    assert!(!msg.is_empty());
    assert!(msg.to_lowercase().contains("mesh") || msg.to_lowercase().contains("denied"));
}
