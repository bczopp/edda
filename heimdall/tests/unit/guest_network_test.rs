#[cfg(test)]
mod tests {
    use heimdall::utils::DeviceRepository;
    use heimdall::guest::{GuestNetworkManager, GuestNetworkIsolator, DataTransferPermissionManager, ExplicitAccessManager};
    use uuid::Uuid;
    use std::sync::Arc;
    use sqlx::Row;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_create_guest_network() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let manager = GuestNetworkManager::new(test_db.pool.clone(), device_repo);
        
        let owner_user_id = Uuid::new_v4();
        let network_id = manager.create_guest_network(owner_user_id).await.unwrap();
        
        assert!(!network_id.is_empty());
        
        // Verify network was created in database
        let row = sqlx::query("SELECT * FROM guest_networks WHERE network_id = $1")
            .bind(&network_id)
            .fetch_optional(&test_db.pool)
            .await
            .unwrap();
        assert!(row.is_some());
        let row = row.unwrap();
        assert_eq!(row.get::<Uuid, _>("owner_user_id"), owner_user_id);
    }

    #[tokio::test]
    async fn test_guest_network_isolation_guest_to_guest() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let manager = GuestNetworkManager::new(test_db.pool.clone(), device_repo.clone());
        let isolator = GuestNetworkIsolator::new(test_db.pool.clone(), device_repo);
        
        let owner_user_id = Uuid::new_v4();
        let network_id = manager.create_guest_network(owner_user_id).await.unwrap();
        
        // Create guest devices (unique ids to avoid duplicate key across tests)
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let guest_id1 = Uuid::new_v4().to_string();
        let guest_id2 = Uuid::new_v4().to_string();
        let guest_device1 = device_repo.create(
            &guest_id1,
            Uuid::new_v4(), // Different user (guest)
            "guest-key-1",
            Some("Guest Device 1"),
            Some("mobile"),
        ).await.unwrap();
        
        let guest_device2 = device_repo.create(
            &guest_id2,
            Uuid::new_v4(), // Different user (guest)
            "guest-key-2",
            Some("Guest Device 2"),
            Some("mobile"),
        ).await.unwrap();
        
        // Add devices to guest network
        manager.add_device_to_network(&network_id, guest_device1.id).await.unwrap();
        manager.add_device_to_network(&network_id, guest_device2.id).await.unwrap();
        
        // Guest devices can communicate with each other
        assert!(isolator.can_communicate(guest_device1.id, guest_device2.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_guest_network_isolation_guest_to_main_network() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let manager = GuestNetworkManager::new(test_db.pool.clone(), device_repo.clone());
        let isolator = GuestNetworkIsolator::new(test_db.pool.clone(), device_repo.clone());
        
        let owner_user_id = Uuid::new_v4();
        let network_id = manager.create_guest_network(owner_user_id).await.unwrap();
        
        // Create guest device (unique id to avoid duplicate key across tests)
        let guest_id = Uuid::new_v4().to_string();
        let guest_device = device_repo.create(
            &guest_id,
            Uuid::new_v4(), // Guest user
            "guest-key",
            Some("Guest Device"),
            Some("mobile"),
        ).await.unwrap();
        
        // Create main network device (different user)
        let main_id = Uuid::new_v4().to_string();
        let main_device = device_repo.create(
            &main_id,
            Uuid::new_v4(), // Different user (main network)
            "main-key",
            Some("Main Device"),
            Some("desktop"),
        ).await.unwrap();
        
        // Add guest device to network
        manager.add_device_to_network(&network_id, guest_device.id).await.unwrap();
        
        // Guest cannot communicate with main network device
        assert!(!isolator.can_communicate(guest_device.id, main_device.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_guest_can_communicate_with_own_devices() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let manager = GuestNetworkManager::new(test_db.pool.clone(), device_repo.clone());
        let isolator = GuestNetworkIsolator::new(test_db.pool.clone(), device_repo.clone());
        let owner_user_id = Uuid::new_v4();
        let network_id = manager.create_guest_network(owner_user_id).await.unwrap();
        let guest_id = Uuid::new_v4().to_string();
        let owner_id = Uuid::new_v4().to_string();

        // Create guest device
        let guest_device = device_repo.create(
            &guest_id,
            owner_user_id, // Same user as owner
            "guest-key",
            Some("Guest Device"),
            Some("mobile"),
        ).await.unwrap();

        // Create owner's main network device
        let owner_device = device_repo.create(
            &owner_id,
            owner_user_id, // Same user
            "owner-key",
            Some("Owner Device"),
            Some("desktop"),
        ).await.unwrap();
        
        // Add guest device to network
        manager.add_device_to_network(&network_id, guest_device.id).await.unwrap();
        
        // Guest can communicate with own devices in main network
        assert!(isolator.can_communicate(guest_device.id, owner_device.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_data_transfer_permission_required() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let manager = GuestNetworkManager::new(test_db.pool.clone(), device_repo.clone());
        let permission_manager = DataTransferPermissionManager::new(test_db.pool.clone(), device_repo);
        
        let owner_user_id = Uuid::new_v4();
        let network_id = manager.create_guest_network(owner_user_id).await.unwrap();
        
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let g1 = Uuid::new_v4().to_string();
        let g2 = Uuid::new_v4().to_string();
        let guest_device1 = device_repo.create(
            &g1,
            Uuid::new_v4(),
            "guest-key-1",
            Some("Guest Device 1"),
            Some("mobile"),
        ).await.unwrap();
        
        let guest_device2 = device_repo.create(
            &g2,
            Uuid::new_v4(),
            "guest-key-2",
            Some("Guest Device 2"),
            Some("mobile"),
        ).await.unwrap();
        
        manager.add_device_to_network(&network_id, guest_device1.id).await.unwrap();
        manager.add_device_to_network(&network_id, guest_device2.id).await.unwrap();
        
        // Data transfer should require explicit permission
        assert!(!permission_manager.has_permission(guest_device1.id, guest_device2.id).await.unwrap());
        
        // Grant permission
        permission_manager.grant_permission(guest_device1.id, guest_device2.id, None).await.unwrap();
        
        // Now should have permission
        assert!(permission_manager.has_permission(guest_device1.id, guest_device2.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_explicit_main_network_access() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let manager = GuestNetworkManager::new(test_db.pool.clone(), device_repo.clone());
        let access_manager = ExplicitAccessManager::new(test_db.pool.clone(), device_repo.clone());
        
        let owner_user_id = Uuid::new_v4();
        let network_id = manager.create_guest_network(owner_user_id).await.unwrap();
        
        let guest_id = Uuid::new_v4().to_string();
        let guest_device = device_repo.create(
            &guest_id,
            Uuid::new_v4(),
            "guest-key",
            Some("Guest Device"),
            Some("mobile"),
        ).await.unwrap();
        
        let main_id = Uuid::new_v4().to_string();
        let main_device = device_repo.create(
            &main_id,
            Uuid::new_v4(),
            "main-key",
            Some("Main Device"),
            Some("desktop"),
        ).await.unwrap();
        
        manager.add_device_to_network(&network_id, guest_device.id).await.unwrap();
        
        // Initially no access
        assert!(!access_manager.has_access(guest_device.id, main_device.id).await.unwrap());
        
        // Request access (first confirmation)
        access_manager.request_access(guest_device.id, main_device.id).await.unwrap();
        assert!(!access_manager.has_access(guest_device.id, main_device.id).await.unwrap());
        
        // Second confirmation
        access_manager.confirm_access(guest_device.id, main_device.id).await.unwrap();
        assert!(!access_manager.has_access(guest_device.id, main_device.id).await.unwrap());
        
        // Third confirmation (should grant access)
        access_manager.confirm_access(guest_device.id, main_device.id).await.unwrap();
        assert!(access_manager.has_access(guest_device.id, main_device.id).await.unwrap());
    }
}
