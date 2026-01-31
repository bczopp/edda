//! Tests for Phase 12.3.1: Guest Mesh Cleanup Manager (connection close, timeout, resource release).

use bifrost::guest::{GuestMeshCleanupManager, GuestMeshManager, MAIN_MESH_ID};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn remove_guest_mesh_releases_from_manager() {
    let manager = GuestMeshManager::new();
    let id = manager.create_guest_mesh();
    assert!(manager.is_guest_mesh(id.as_str()));
    manager.remove_guest_mesh(id.as_str());
    assert!(!manager.is_guest_mesh(id.as_str()));
}

#[test]
fn remove_guest_mesh_ignores_main() {
    let manager = GuestMeshManager::new();
    manager.remove_guest_mesh(MAIN_MESH_ID);
    assert!(!manager.is_guest_mesh(MAIN_MESH_ID));
}

#[test]
fn cleanup_after_connection_closed_and_timeout_removes_guest_mesh() {
    let guest_manager = Arc::new(GuestMeshManager::new());
    let mesh_id = guest_manager.create_guest_mesh();
    let mesh_str = mesh_id.as_str().to_string();
    let cleanup = GuestMeshCleanupManager::new(Arc::clone(&guest_manager), Duration::from_millis(10));
    cleanup.register_connection(&mesh_str);
    cleanup.on_connection_closed(&mesh_str);
    std::thread::sleep(Duration::from_millis(20));
    cleanup.cleanup_idle();
    assert!(!guest_manager.is_guest_mesh(&mesh_str));
}

#[test]
fn cleanup_does_not_remove_while_connections_exist() {
    let guest_manager = Arc::new(GuestMeshManager::new());
    let mesh_id = guest_manager.create_guest_mesh();
    let mesh_str = mesh_id.as_str().to_string();
    let cleanup = GuestMeshCleanupManager::new(Arc::clone(&guest_manager), Duration::from_millis(5));
    cleanup.register_connection(&mesh_str);
    cleanup.register_connection(&mesh_str);
    cleanup.on_connection_closed(&mesh_str);
    std::thread::sleep(Duration::from_millis(15));
    cleanup.cleanup_idle();
    assert!(guest_manager.is_guest_mesh(&mesh_str));
}

#[test]
fn cleanup_does_not_remove_before_timeout() {
    let guest_manager = Arc::new(GuestMeshManager::new());
    let mesh_id = guest_manager.create_guest_mesh();
    let mesh_str = mesh_id.as_str().to_string();
    let cleanup = GuestMeshCleanupManager::new(Arc::clone(&guest_manager), Duration::from_secs(60));
    cleanup.register_connection(&mesh_str);
    cleanup.on_connection_closed(&mesh_str);
    cleanup.cleanup_idle();
    assert!(guest_manager.is_guest_mesh(&mesh_str));
}
