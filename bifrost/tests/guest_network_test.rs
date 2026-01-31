//! Guest Mesh Manager tests (Phase 12.1.1, TDD). Guest = isolated mesh segment (no VPN).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::guest::{GuestMeshManager, MAIN_MESH_ID};

#[test]
fn main_mesh_id_is_constant() {
    assert_eq!(MAIN_MESH_ID, "main");
}

#[test]
fn create_guest_mesh_returns_distinct_id() {
    let manager = GuestMeshManager::new();
    let id1 = manager.create_guest_mesh();
    let id2 = manager.create_guest_mesh();
    assert_ne!(id1.as_str(), id2.as_str());
    assert_ne!(id1.as_str(), MAIN_MESH_ID);
}

#[test]
fn is_guest_mesh_returns_true_for_created_guest_id() {
    let manager = GuestMeshManager::new();
    let id = manager.create_guest_mesh();
    assert!(manager.is_guest_mesh(id.as_str()));
}

#[test]
fn is_guest_mesh_returns_false_for_main() {
    let manager = GuestMeshManager::new();
    assert!(!manager.is_guest_mesh(MAIN_MESH_ID));
}

#[test]
fn is_guest_mesh_returns_false_for_unknown_id() {
    let manager = GuestMeshManager::new();
    assert!(!manager.is_guest_mesh("unknown"));
}

#[test]
fn list_guest_meshes_returns_created_segments() {
    let manager = GuestMeshManager::new();
    let id1 = manager.create_guest_mesh();
    let id2 = manager.create_guest_mesh();
    let list = manager.list_guest_meshes();
    assert!(list.contains(&id1));
    assert!(list.contains(&id2));
    assert_eq!(list.len(), 2);
}
