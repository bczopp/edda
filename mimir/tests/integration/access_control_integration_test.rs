#[cfg(test)]
mod tests {
    use mimir::storage::database::EncryptedDatabase;
    use mimir::encryption::EncryptionManager;
    use mimir::access_control::{AccessControlManager, Role, Action};
    use mimir::audit::{AuditLogManager, AuditEvent};
    use tests::common::TestDatabase;
    use ring::rand::{SecureRandom, SystemRandom};
    use std::sync::Arc;

    fn generate_key() -> Vec<u8> {
        let mut key = vec![0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut key).unwrap();
        key
    }

    #[tokio::test]
    async fn test_store_data_with_access_control() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let access_control = Arc::new(AccessControlManager::new());
        let audit_logger = Arc::new(AuditLogManager::new(test_db.pool.clone()));
        
        // Register user
        access_control.register_user("user1".to_string(), Role::User).await;
        
        let database = EncryptedDatabase::new_with_access_control_and_audit(
            &test_db.pool,
            encryption_manager,
            access_control.clone(),
            audit_logger.clone(),
        ).await.unwrap();
        
        let user_id = "user1";
        let data = b"Secret user data";
        
        // Store data - should succeed with proper access control
        let data_id = database.store_data_with_access_control(user_id, data, user_id).await.unwrap();
        assert!(!data_id.is_empty());
        
        // Verify audit log was created
        let logs = audit_logger.get_user_audit_logs(user_id).await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataStored)));
    }

    #[tokio::test]
    async fn test_store_data_access_denied() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let access_control = Arc::new(AccessControlManager::new());
        let audit_logger = Arc::new(AuditLogManager::new(test_db.pool.clone()));
        
        // Register user but don't grant permission
        access_control.register_user("user1".to_string(), Role::ReadOnly).await;
        
        let database = EncryptedDatabase::new_with_access_control_and_audit(
            &test_db.pool,
            encryption_manager,
            access_control.clone(),
            audit_logger.clone(),
        ).await.unwrap();
        
        let user_id = "user1";
        let data = b"Secret user data";
        
        // Store data - should fail due to access control (ReadOnly cannot create)
        let result = database.store_data_with_access_control(user_id, data, user_id).await;
        assert!(result.is_err());
        
        // Verify access denied was logged
        let logs = audit_logger.get_user_audit_logs(user_id).await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::AccessDenied)));
    }

    #[tokio::test]
    async fn test_retrieve_data_with_access_control() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let access_control = Arc::new(AccessControlManager::new());
        let audit_logger = Arc::new(AuditLogManager::new(test_db.pool.clone()));
        
        // Register users
        access_control.register_user("user1".to_string(), Role::User).await;
        access_control.register_user("user2".to_string(), Role::User).await;
        
        let database = EncryptedDatabase::new_with_access_control_and_audit(
            &test_db.pool,
            encryption_manager,
            access_control.clone(),
            audit_logger.clone(),
        ).await.unwrap();
        
        let user1_id = "user1";
        let user2_id = "user2";
        let data = b"User 1 secret data";
        
        // Store data for user1
        let data_id = database.store_data_with_access_control(user1_id, data, user1_id).await.unwrap();
        
        // User1 can retrieve their own data
        let retrieved = database.retrieve_data_with_access_control(&data_id, user1_id, user1_id).await.unwrap();
        assert_eq!(retrieved, data);
        
        // Verify audit log for retrieval
        let logs = audit_logger.get_user_audit_logs(user1_id).await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataRetrieved)));
        
        // User2 cannot retrieve user1's data
        let result = database.retrieve_data_with_access_control(&data_id, user2_id, user1_id).await;
        assert!(result.is_err());
        
        // Verify access denied was logged
        let logs = audit_logger.get_user_audit_logs(user2_id).await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::AccessDenied)));
    }

    #[tokio::test]
    async fn test_delete_data_with_access_control() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let access_control = Arc::new(AccessControlManager::new());
        let audit_logger = Arc::new(AuditLogManager::new(test_db.pool.clone()));
        
        // Register users
        access_control.register_user("user1".to_string(), Role::User).await;
        access_control.register_user("user2".to_string(), Role::User).await;
        
        let database = EncryptedDatabase::new_with_access_control_and_audit(
            &test_db.pool,
            encryption_manager,
            access_control.clone(),
            audit_logger.clone(),
        ).await.unwrap();
        
        let user1_id = "user1";
        let user2_id = "user2";
        let data = b"User 1 data";
        
        // Store data for user1
        let data_id = database.store_data_with_access_control(user1_id, data, user1_id).await.unwrap();
        
        // User1 can delete their own data
        database.delete_data_with_access_control(&data_id, user1_id, user1_id).await.unwrap();
        
        // Verify audit log for deletion
        let logs = audit_logger.get_user_audit_logs(user1_id).await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataDeleted)));
        
        // Store data again
        let data_id2 = database.store_data_with_access_control(user1_id, data, user1_id).await.unwrap();
        
        // User2 cannot delete user1's data
        let result = database.delete_data_with_access_control(&data_id2, user2_id, user1_id).await;
        assert!(result.is_err());
        
        // Verify access denied was logged
        let logs = audit_logger.get_user_audit_logs(user2_id).await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::AccessDenied)));
    }

    #[tokio::test]
    async fn test_admin_can_access_any_data() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let access_control = Arc::new(AccessControlManager::new());
        let audit_logger = Arc::new(AuditLogManager::new(test_db.pool.clone()));
        
        // Register admin and user
        access_control.register_user("admin".to_string(), Role::Admin).await;
        access_control.register_user("user1".to_string(), Role::User).await;
        
        let database = EncryptedDatabase::new_with_access_control_and_audit(
            &test_db.pool,
            encryption_manager,
            access_control.clone(),
            audit_logger.clone(),
        ).await.unwrap();
        
        let user1_id = "user1";
        let admin_id = "admin";
        let data = b"User 1 secret data";
        
        // Store data for user1
        let data_id = database.store_data_with_access_control(user1_id, data, user1_id).await.unwrap();
        
        // Admin can retrieve user1's data
        let retrieved = database.retrieve_data_with_access_control(&data_id, admin_id, user1_id).await.unwrap();
        assert_eq!(retrieved, data);
        
        // Admin can delete user1's data
        database.delete_data_with_access_control(&data_id, admin_id, user1_id).await.unwrap();
        
        // Verify audit logs
        let admin_logs = audit_logger.get_user_audit_logs(admin_id).await.unwrap();
        assert!(admin_logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataRetrieved)));
        assert!(admin_logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataDeleted)));
    }
}
