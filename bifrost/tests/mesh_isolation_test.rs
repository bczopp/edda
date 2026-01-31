//! Mesh Isolation Enforcer tests (Phase 12.1.2, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::guest::{GuestMeshManager, MeshIsolationEnforcer, MAIN_MESH_ID};
use std::sync::Arc;

#[test]
fn main_to_main_allowed() {
    let manager = Arc::new(GuestMeshManager::new());
    let enforcer = MeshIsolationEnforcer::new(manager);

    assert!(enforcer.can_deliver(MAIN_MESH_ID, MAIN_MESH_ID));
}

#[test]
fn guest_to_main_blocked() {
    let manager = Arc::new(GuestMeshManager::new());
    let guest_id = manager.create_guest_mesh();
    let enforcer = MeshIsolationEnforcer::new(Arc::clone(&manager));

    assert!(!enforcer.can_deliver(guest_id.as_str(), MAIN_MESH_ID));
}

#[test]
fn main_to_guest_blocked() {
    let manager = Arc::new(GuestMeshManager::new());
    let guest_id = manager.create_guest_mesh();
    let enforcer = MeshIsolationEnforcer::new(Arc::clone(&manager));

    assert!(!enforcer.can_deliver(MAIN_MESH_ID, guest_id.as_str()));
}

#[test]
fn guest_to_same_guest_allowed() {
    let manager = Arc::new(GuestMeshManager::new());
    let guest_id = manager.create_guest_mesh();
    let enforcer = MeshIsolationEnforcer::new(Arc::clone(&manager));

    assert!(enforcer.can_deliver(guest_id.as_str(), guest_id.as_str()));
}

#[test]
fn guest_to_other_guest_blocked() {
    let manager = Arc::new(GuestMeshManager::new());
    let g1 = manager.create_guest_mesh();
    let g2 = manager.create_guest_mesh();
    let enforcer = MeshIsolationEnforcer::new(Arc::clone(&manager));

    assert!(!enforcer.can_deliver(g1.as_str(), g2.as_str()));
}
