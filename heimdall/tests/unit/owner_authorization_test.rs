#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use uuid::Uuid;

    use heimdall::mesh::{MeshDeviceRegistry, OwnerAuthorizationManager};
    use heimdall::utils::device_repository::DeviceRepository;
    use crate::common::TestDatabase;

    fn setup(test_db: &TestDatabase) -> OwnerAuthorizationManager {
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            device_repo,
        ));
        OwnerAuthorizationManager::new(mesh_registry)
    }

    #[tokio::test]
    async fn approve_device_as_owner_updates_role_and_activates() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", Some("Device 1"), Some("desktop"))
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        mesh_registry
            .register_device(&dev_id, "Device 1", "desktop", "mesh-key", owner_id)
            .await
            .unwrap();

        let manager = setup(&test_db);
        let mesh_device = manager
            .approve_device(owner_id, &dev_id, "admin")
            .await
            .unwrap();
        assert_eq!(mesh_device.role, "admin");
        assert!(mesh_device.is_active);
    }

    #[tokio::test]
    async fn approve_device_as_non_owner_returns_not_owner() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        mesh_registry
            .register_device(&dev_id, "D2", "mobile", "key", owner_id)
            .await
            .unwrap();

        let manager = setup(&test_db);
        let err = manager
            .approve_device(other_id, &dev_id, "admin")
            .await
            .unwrap_err();
        assert!(matches!(err, heimdall::mesh::MeshRegistryError::NotOwner));
    }

    #[tokio::test]
    async fn reject_device_as_owner_sets_inactive() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            device_repo.clone(),
        ));
        mesh_registry
            .register_device(&dev_id, "D3", "mobile", "key", owner_id)
            .await
            .unwrap();
        let manager = OwnerAuthorizationManager::new(mesh_registry.clone());
        manager.reject_device(owner_id, &dev_id).await.unwrap();
        let mesh_device = mesh_registry.get_by_device_id(&dev_id).await.unwrap();
        assert!(!mesh_device.is_active);
    }

    #[tokio::test]
    async fn reject_device_as_non_owner_returns_not_owner() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        let other_id = Uuid::new_v4();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        mesh_registry
            .register_device(&dev_id, "D4", "mobile", "key", owner_id)
            .await
            .unwrap();

        let manager = setup(&test_db);
        let err = manager.reject_device(other_id, &dev_id).await.unwrap_err();
        assert!(matches!(err, heimdall::mesh::MeshRegistryError::NotOwner));
    }

    #[tokio::test]
    async fn get_device_details_for_owner_returns_details() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(
                &dev_id,
                owner_id,
                "key",
                Some("My Device"),
                Some("desktop"),
            )
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        mesh_registry
            .register_device(&dev_id, "My Device", "desktop", "key", owner_id)
            .await
            .unwrap();

        let manager = setup(&test_db);
        let details = manager
            .get_device_details_for_owner(&dev_id, owner_id)
            .await
            .unwrap();
        assert_eq!(details.device_id, dev_id);
        assert_eq!(details.device_name, Some("My Device".to_string()));
        assert_eq!(details.device_type, Some("desktop".to_string()));
        assert_eq!(details.role, "user");
        assert!(details.is_active);
    }

    #[tokio::test]
    async fn approve_device_invalid_role_returns_error() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        mesh_registry
            .register_device(&dev_id, "D", "mobile", "key", owner_id)
            .await
            .unwrap();
        let manager = setup(&test_db);
        let err = manager
            .approve_device(owner_id, &dev_id, "superuser")
            .await
            .unwrap_err();
        assert!(matches!(err, heimdall::mesh::MeshRegistryError::InvalidRole));
    }

    #[tokio::test]
    async fn get_device_details_for_non_owner_returns_not_owner() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        mesh_registry
            .register_device(&dev_id, "D6", "mobile", "key", owner_id)
            .await
            .unwrap();

        let manager = setup(&test_db);
        let err = manager
            .get_device_details_for_owner(&dev_id, other_id)
            .await
            .unwrap_err();
        assert!(matches!(err, heimdall::mesh::MeshRegistryError::NotOwner));
    }
}
