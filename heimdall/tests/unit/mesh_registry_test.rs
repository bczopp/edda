#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use heimdall::mesh::MeshDeviceRegistry;
    use heimdall::utils::device_repository::DeviceRepository;
    use uuid::Uuid;
    use crate::common::TestDatabase;

    fn setup_mesh_registry(test_db: &TestDatabase) -> MeshDeviceRegistry {
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        MeshDeviceRegistry::new(test_db.pool.clone(), device_repo)
    }

    #[tokio::test]
    async fn register_device_registers_new_device_returns_true() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(
                &dev_id,
                owner_id,
                "device-public-key",
                Some("Mesh Device 1"),
                Some("desktop"),
            )
            .await
            .unwrap();

        let registry = setup_mesh_registry(&test_db);
        let (mesh_device, is_new) = registry
            .register_device(
                &dev_id,
                "Mesh Device 1",
                "desktop",
                "mesh-public-key-base64",
                owner_id,
            )
            .await
            .unwrap();

        assert!(is_new);
        assert_eq!(mesh_device.mesh_public_key, "mesh-public-key-base64");
        assert_eq!(mesh_device.role, "user");
        assert_eq!(mesh_device.owner_user_id, owner_id);
    }

    #[tokio::test]
    async fn register_device_already_registered_returns_false() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let registry = setup_mesh_registry(&test_db);
        let (_, first) = registry
            .register_device(&dev_id, "D2", "mobile", "key2", owner_id)
            .await
            .unwrap();
        let (mesh_device, second) = registry
            .register_device(&dev_id, "D2", "mobile", "key2", owner_id)
            .await
            .unwrap();

        assert!(first);
        assert!(!second);
        assert_eq!(mesh_device.mesh_public_key, "key2");
    }

    #[tokio::test]
    async fn register_device_unknown_device_id_returns_device_not_found() {
        let test_db = TestDatabase::new().await.unwrap();
        let registry = setup_mesh_registry(&test_db);
        let owner_id = Uuid::new_v4();

        let err = registry
            .register_device(
                "nonexistent-device-id",
                "No Device",
                "desktop",
                "key",
                owner_id,
            )
            .await
            .unwrap_err();

        assert!(matches!(err, heimdall::mesh::MeshRegistryError::DeviceNotFound));
    }

    #[tokio::test]
    async fn get_by_device_id_returns_mesh_device() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let registry = setup_mesh_registry(&test_db);
        registry
            .register_device(&dev_id, "D3", "mobile", "mesh-key", owner_id)
            .await
            .unwrap();

        let mesh_device = registry.get_by_device_id(&dev_id).await.unwrap();
        assert_eq!(mesh_device.mesh_public_key, "mesh-key");
        assert_eq!(mesh_device.owner_user_id, owner_id);
    }

    #[tokio::test]
    async fn get_by_device_id_unregistered_returns_device_not_found() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let registry = setup_mesh_registry(&test_db);
        // Device exists in devices table but not in mesh_devices
        let err = registry.get_by_device_id(&dev_id).await.unwrap_err();
        assert!(matches!(err, heimdall::mesh::MeshRegistryError::DeviceNotFound));
    }

    #[tokio::test]
    async fn get_by_device_id_unknown_device_returns_device_not_found() {
        let test_db = TestDatabase::new().await.unwrap();
        let registry = setup_mesh_registry(&test_db);
        let err = registry.get_by_device_id("no-such-device").await.unwrap_err();
        assert!(matches!(err, heimdall::mesh::MeshRegistryError::DeviceNotFound));
    }

    #[tokio::test]
    async fn update_last_seen_succeeds() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let registry = setup_mesh_registry(&test_db);
        registry
            .register_device(&dev_id, "D5", "mobile", "key", owner_id)
            .await
            .unwrap();

        let result = registry.update_last_seen(&dev_id).await;
        assert!(result.is_ok());
    }
}
