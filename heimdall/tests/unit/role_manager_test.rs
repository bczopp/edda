//! Tests for RoleManager (Phase 7.2.1): base roles, custom roles, hierarchy, inheritance.

use heimdall::authz::RoleManager;
use heimdall::utils::device_repository::DeviceRepository;
use std::sync::Arc;
use crate::common::TestDatabase;
use uuid::Uuid;

#[tokio::test]
async fn test_ensure_base_roles_creates_admin_user_guest() {
    let test_db = TestDatabase::new().await.unwrap();
    let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
    let mgr = RoleManager::new(test_db.pool.clone(), device_repo);

    mgr.ensure_base_roles().await.unwrap();

    let admin_id = mgr.get_role_id("admin").await.unwrap();
    let user_id = mgr.get_role_id("user").await.unwrap();
    let guest_id = mgr.get_role_id("guest").await.unwrap();
    assert!(admin_id.is_some());
    assert!(user_id.is_some());
    assert!(guest_id.is_some());
}

#[tokio::test]
async fn test_assign_role_to_device_and_get_roles() {
    let test_db = TestDatabase::new().await.unwrap();
    let pool = test_db.pool.clone();
    let device_repo = Arc::new(DeviceRepository::new(pool.clone()));
    let mgr = RoleManager::new(pool.clone(), device_repo.clone());

    mgr.ensure_base_roles().await.unwrap();

    let device_id = Uuid::new_v4().to_string();
    let user_id = Uuid::new_v4();
    device_repo
        .create(&device_id, user_id, "pubkey", None, None)
        .await
        .unwrap();

    mgr.assign_role_to_device(&device_id, "user").await.unwrap();
    let roles = mgr.get_roles_for_device(&device_id).await.unwrap();
    assert!(roles.contains(&"user".to_string()));
}

#[tokio::test]
async fn test_remove_role_from_device() {
    let test_db = TestDatabase::new().await.unwrap();
    let pool = test_db.pool.clone();
    let device_repo = Arc::new(DeviceRepository::new(pool.clone()));
    let mgr = RoleManager::new(pool.clone(), device_repo.clone());

    mgr.ensure_base_roles().await.unwrap();
    let device_id = Uuid::new_v4().to_string();
    let user_id = Uuid::new_v4();
    device_repo
        .create(&device_id, user_id, "pubkey", None, None)
        .await
        .unwrap();

    mgr.assign_role_to_device(&device_id, "guest").await.unwrap();
    mgr.remove_role_from_device(&device_id, "guest").await.unwrap();
    let roles = mgr.get_roles_for_device(&device_id).await.unwrap();
    assert!(!roles.contains(&"guest".to_string()));
}

#[tokio::test]
async fn test_create_custom_role_with_parent() {
    let test_db = TestDatabase::new().await.unwrap();
    let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
    let mgr = RoleManager::new(test_db.pool.clone(), device_repo);

    mgr.ensure_base_roles().await.unwrap();

    let custom_id = mgr
        .create_role("editor", Some("user"), Some("Can edit content"))
        .await
        .unwrap();
    assert!(custom_id != uuid::Uuid::nil());

    let role_id = mgr.get_role_id("editor").await.unwrap().unwrap();
    assert_eq!(role_id, custom_id);
}

#[tokio::test]
async fn test_get_inherited_role_ids_includes_parent() {
    let test_db = TestDatabase::new().await.unwrap();
    let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
    let mgr = RoleManager::new(test_db.pool.clone(), device_repo);

    mgr.ensure_base_roles().await.unwrap();
    mgr.create_role("editor", Some("user"), None).await.unwrap();

    let editor_id = mgr.get_role_id("editor").await.unwrap().unwrap();
    let inherited = mgr.get_inherited_role_ids(editor_id).await.unwrap();
    assert!(inherited.len() >= 1);
    let user_id = mgr.get_role_id("user").await.unwrap().unwrap();
    assert!(inherited.contains(&user_id));
}
