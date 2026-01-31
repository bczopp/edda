#[cfg(test)]
mod tests {
    use mimir::gdpr::compliance::GDPRCompliance;
    use mimir::storage::EncryptedDatabase;
    use mimir::encryption::EncryptionManager;
    use tests::common::TestDatabase;
    use ring::rand::{SecureRandom, SystemRandom};

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
        
        let database = EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap();
        
        let gdpr = GDPRCompliance::new(database);
        
        let user_id = "user123";
        
        // Store multiple data entries
        database.store_data(user_id, b"Data 1").await.unwrap();
        database.store_data(user_id, b"Data 2").await.unwrap();
        database.store_data(user_id, b"Data 3").await.unwrap();
        
        // Export user data
        let exported = gdpr.export_user_data(user_id).await.unwrap();
        
        // Exported data should be JSON containing all user data
        assert!(!exported.is_empty());
        // The format should be parseable JSON
        let json_str = String::from_utf8(exported).unwrap();
        assert!(json_str.contains("user_id"));
    }

    #[tokio::test]
    async fn test_delete_user_data_right_to_be_forgotten() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap();
        
        let gdpr = GDPRCompliance::new(database.clone());
        
        let user_id = "user123";
        
        // Store multiple data entries
        let data_id1 = database.store_data(user_id, b"Data 1").await.unwrap();
        let data_id2 = database.store_data(user_id, b"Data 2").await.unwrap();
        
        // Verify data exists
        assert!(database.retrieve_data(&data_id1, user_id).await.is_ok());
        assert!(database.retrieve_data(&data_id2, user_id).await.is_ok());
        
        // Delete all user data (Right to be forgotten)
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
        
        let database = EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap();
        
        let gdpr = GDPRCompliance::new(database.clone());
        
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
}
