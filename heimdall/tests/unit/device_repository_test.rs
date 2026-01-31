#[cfg(test)]
mod tests {
    use heimdall::utils::device_repository::DeviceRepository;
    use uuid::Uuid;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_create_device() {
        let test_db = TestDatabase::new().await.unwrap();
        let repo = DeviceRepository::new(test_db.pool);
        let user_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        let device = repo.create(
            &dev_id,
            user_id,
            "test-public-key",
            Some("Test Device"),
            Some("desktop"),
        ).await.unwrap();
        
        assert_eq!(device.device_id, dev_id);
        assert_eq!(device.user_id, user_id);
        assert_eq!(device.public_key, "test-public-key");
    }

    #[tokio::test]
    async fn test_get_device_by_id() {
        let test_db = TestDatabase::new().await.unwrap();
        let repo = DeviceRepository::new(test_db.pool);
        let user_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        let created = repo.create(
            &dev_id,
            user_id,
            "test-public-key",
            None,
            None,
        ).await.unwrap();
        
        let retrieved = repo.get_by_id(created.id).await.unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.device_id, dev_id);
    }

    #[tokio::test]
    async fn test_get_device_by_device_id() {
        let test_db = TestDatabase::new().await.unwrap();
        let repo = DeviceRepository::new(test_db.pool);
        let user_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        repo.create(
            &dev_id,
            user_id,
            "test-public-key",
            None,
            None,
        ).await.unwrap();
        
        let retrieved = repo.get_by_device_id(&dev_id).await.unwrap();
        assert_eq!(retrieved.device_id, dev_id);
    }

    #[tokio::test]
    async fn test_update_device() {
        let test_db = TestDatabase::new().await.unwrap();
        let repo = DeviceRepository::new(test_db.pool);
        let user_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        let created = repo.create(
            &dev_id,
            user_id,
            "test-public-key",
            None,
            None,
        ).await.unwrap();
        
        let updated = repo.update(
            created.id,
            Some("Updated Name"),
            Some("mobile"),
            Some(false),
        ).await.unwrap();
        
        assert_eq!(updated.device_name, Some("Updated Name".to_string()));
        assert_eq!(updated.device_type, Some("mobile".to_string()));
        assert_eq!(updated.is_active, false);
    }

    #[tokio::test]
    async fn test_list_devices_by_user() {
        let test_db = TestDatabase::new().await.unwrap();
        let repo = DeviceRepository::new(test_db.pool);
        let user_id = Uuid::new_v4();
        let dev1 = Uuid::new_v4().to_string();
        let dev2 = Uuid::new_v4().to_string();
        repo.create(&dev1, user_id, "key1", None, None).await.unwrap();
        repo.create(&dev2, user_id, "key2", None, None).await.unwrap();
        
        let devices = repo.list_by_user_id(user_id).await.unwrap();
        assert_eq!(devices.len(), 2);
    }
}
