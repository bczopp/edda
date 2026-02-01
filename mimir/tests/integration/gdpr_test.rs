#[cfg(test)]
mod tests {
    use mimir::gdpr::compliance::GDPRCompliance;
    use mimir::storage::EncryptedDatabase;
    use mimir::encryption::EncryptionManager;
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
    async fn test_export_user_data() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user_id = "user123";
        
        // Store multiple data entries
        database.store_data(user_id, b"Data 1").await.unwrap();
        database.store_data(user_id, b"Data 2").await.unwrap();
        database.store_data(user_id, b"Data 3").await.unwrap();
        
        // Export user data (Right to Access - GDPR Art. 15)
        let exported = gdpr.export_user_data(user_id).await.unwrap();
        
        // Exported data should be JSON containing all user data
        assert!(!exported.is_empty());
        // The format should be parseable JSON
        let json_str = String::from_utf8(exported).unwrap();
        assert!(json_str.contains("user_id"));
        assert!(json_str.contains("export_timestamp"));
        assert!(json_str.contains("data_entries"));
    }

    #[tokio::test]
    async fn test_delete_user_data_right_to_be_forgotten() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user_id = "user123";
        
        // Store multiple data entries
        let data_id1 = database.store_data(user_id, b"Data 1").await.unwrap();
        let data_id2 = database.store_data(user_id, b"Data 2").await.unwrap();
        
        // Verify data exists
        assert!(database.retrieve_data(&data_id1, user_id).await.is_ok());
        assert!(database.retrieve_data(&data_id2, user_id).await.is_ok());
        
        // Delete all user data (Right to Erasure - GDPR Art. 17)
        gdpr.delete_user_data(user_id).await.unwrap();
        
        // Verify all data is deleted
        assert!(database.retrieve_data(&data_id1, user_id).await.is_err());
        assert!(database.retrieve_data(&data_id2, user_id).await.is_err());
    }

    #[tokio::test]
    async fn test_delete_user_data_only_affects_one_user() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user1_id = "user1";
        let user2_id = "user2";
        
        // Store data for both users
        let user1_data_id = database.store_data(user1_id, b"User 1 data").await.unwrap();
        let user2_data_id = database.store_data(user2_id, b"User 2 data").await.unwrap();
        
        // Delete user 1's data
        gdpr.delete_user_data(user1_id).await.unwrap();
        
        // User 1's data should be deleted
        assert!(database.retrieve_data(&user1_data_id, user1_id).await.is_err());
        
        // User 2's data should still exist
        let user2_data = database.retrieve_data(&user2_data_id, user2_id).await.unwrap();
        assert_eq!(user2_data, b"User 2 data");
    }

    #[tokio::test]
    async fn test_rectify_user_data() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user_id = "user123";
        let original_data = b"Original incorrect data";
        let corrected_data = b"Corrected data";
        
        // Store original data
        let data_id = database.store_data(user_id, original_data).await.unwrap();
        
        // Verify original data
        let retrieved = database.retrieve_data(&data_id, user_id).await.unwrap();
        assert_eq!(retrieved, original_data);
        
        // Rectify data (Right to Rectification - GDPR Art. 16)
        gdpr.rectify_user_data(user_id, &data_id, corrected_data).await.unwrap();
        
        // Verify data was updated (not deleted and recreated)
        let retrieved = database.retrieve_data(&data_id, user_id).await.unwrap();
        assert_eq!(retrieved, corrected_data);
    }

    #[tokio::test]
    async fn test_anonymize_user_data() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user_id = "user123";
        
        // Store data
        let data_id = database.store_data(user_id, b"User data").await.unwrap();
        
        // Verify data exists
        assert!(database.retrieve_data(&data_id, user_id).await.is_ok());
        
        // Anonymize user data
        gdpr.anonymize_user_data(user_id).await.unwrap();
        
        // Original data should be deleted
        assert!(database.retrieve_data(&data_id, user_id).await.is_err());
        
        // Verify user has no data under their original ID
        let user_data = database.get_all_user_data(user_id).await.unwrap();
        assert!(user_data.is_empty());
    }

    #[tokio::test]
    async fn test_has_user_data() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user_id = "user123";
        
        // User should not have data initially
        assert!(!gdpr.has_user_data(user_id).await.unwrap());
        
        // Store data
        database.store_data(user_id, b"User data").await.unwrap();
        
        // User should now have data
        assert!(gdpr.has_user_data(user_id).await.unwrap());
        
        // Delete data
        gdpr.delete_user_data(user_id).await.unwrap();
        
        // User should not have data anymore
        assert!(!gdpr.has_user_data(user_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_check_retention_compliance() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user_id = "user123";
        
        // Store data
        database.store_data(user_id, b"User data").await.unwrap();
        
        // Check retention compliance
        // Note: Current implementation is simplified
        let is_compliant = gdpr.check_retention_compliance(user_id, 365).await.unwrap();
        assert!(is_compliant); // Has data = compliant (simplified check)
    }

    #[tokio::test]
    async fn test_gdpr_workflow_complete() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = Arc::new(EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap());
        
        let gdpr = GDPRCompliance::new_with_database(database.clone());
        
        let user_id = "user123";
        
        // 1. Store user data
        database.store_data(user_id, b"Personal data 1").await.unwrap();
        database.store_data(user_id, b"Personal data 2").await.unwrap();
        
        // 2. Right to Access - User can export their data
        let exported = gdpr.export_user_data(user_id).await.unwrap();
        assert!(!exported.is_empty());
        
        // 3. Right to Rectification - User can correct their data
        // (Tested separately due to implementation limitations)
        
        // 4. User can check if they have data
        assert!(gdpr.has_user_data(user_id).await.unwrap());
        
        // 5. Right to Erasure - User can delete all their data
        gdpr.delete_user_data(user_id).await.unwrap();
        
        // 6. Verify all data is deleted
        assert!(!gdpr.has_user_data(user_id).await.unwrap());
    }
}

