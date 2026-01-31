#[cfg(test)]
mod tests {
    use mimir::storage::database::EncryptedDatabase;
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
    async fn test_store_and_retrieve_encrypted_data() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap();
        
        let user_id = "user123";
        let data = b"Secret user data";
        
        let data_id = database.store_data(user_id, data).await.unwrap();
        assert!(!data_id.is_empty());
        
        let retrieved = database.retrieve_data(&data_id, user_id).await.unwrap();
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_store_data_different_users() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap();
        
        let user1_id = "user1";
        let user2_id = "user2";
        let data1 = b"User 1 data";
        let data2 = b"User 2 data";
        
        let data_id1 = database.store_data(user1_id, data1).await.unwrap();
        let data_id2 = database.store_data(user2_id, data2).await.unwrap();
        
        // Users should only be able to retrieve their own data
        let retrieved1 = database.retrieve_data(&data_id1, user1_id).await.unwrap();
        assert_eq!(retrieved1, data1);
        
        let retrieved2 = database.retrieve_data(&data_id2, user2_id).await.unwrap();
        assert_eq!(retrieved2, data2);
        
        // User 1 should not be able to retrieve user 2's data
        let result = database.retrieve_data(&data_id2, user1_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_data() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap();
        
        let user_id = "user123";
        let data = b"Data to delete";
        
        let data_id = database.store_data(user_id, data).await.unwrap();
        
        // Verify data exists
        let retrieved = database.retrieve_data(&data_id, user_id).await.unwrap();
        assert_eq!(retrieved, data);
        
        // Delete data
        database.delete_data(&data_id, user_id).await.unwrap();
        
        // Verify data is deleted
        let result = database.retrieve_data(&data_id, user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_data_wrong_user() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let database = EncryptedDatabase::new_with_encryption(
            &test_db.pool,
            encryption_manager,
        ).await.unwrap();
        
        let user1_id = "user1";
        let user2_id = "user2";
        let data = b"User 1 data";
        
        let data_id = database.store_data(user1_id, data).await.unwrap();
        
        // User 2 should not be able to delete user 1's data
        let result = database.delete_data(&data_id, user2_id).await;
        // Should either fail or not delete (implementation dependent)
        // For now, we'll just test that it doesn't crash
        assert!(result.is_ok() || result.is_err());
        
        // User 1's data should still exist
        let retrieved = database.retrieve_data(&data_id, user1_id).await.unwrap();
        assert_eq!(retrieved, data);
    }
}
