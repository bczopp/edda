//! Security test: SQL-Injection prevention (Phase 12.3.1).
//!
//! Verifies that user_id and data_id are treated as parameters (no SQL injection).
//! All storage uses sqlx::query! with $1, $2, etc., so injection attempts are safe.

#[cfg(test)]
mod tests {
    use mimir::storage::database::EncryptedDatabase;
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
    async fn test_sql_injection_user_id_treated_as_parameter() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();

        let database = Arc::new(
            EncryptedDatabase::new_with_encryption(&test_db.pool, encryption_manager)
                .await
                .unwrap(),
        );

        // user_id that would be dangerous if concatenated into SQL
        let malicious_user_id = "user'; DELETE FROM encrypted_data; --";
        let data = b"secret payload";

        let data_id = database
            .store_data(malicious_user_id, data)
            .await
            .unwrap();
        assert!(!data_id.is_empty());

        let retrieved = database
            .retrieve_data(&data_id, malicious_user_id)
            .await
            .unwrap();
        assert_eq!(retrieved.as_slice(), data, "Data must be stored and retrieved unchanged (no injection)");

        // Table must still contain the row (DELETE was not executed)
        let all = database.get_all_user_data(malicious_user_id).await.unwrap();
        assert_eq!(all.len(), 1, "Row must still exist; SQL injection must not have executed");
    }

    #[tokio::test]
    async fn test_sql_injection_like_data_id_retrieve_no_injection() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();

        let database = EncryptedDatabase::new_with_encryption(&test_db.pool, encryption_manager)
            .await
            .unwrap();

        let user_id = "normal_user";
        let data = b"normal data";
        let _data_id = database.store_data(user_id, data).await.unwrap();

        // Attempt retrieve with data_id that looks like SQL (parameterized, so treated as literal)
        let malicious_data_id = "x' OR '1'='1";
        let result = database.retrieve_data(malicious_data_id, user_id).await;
        assert!(result.is_err(), "Non-existent id must return NotFound, not match other rows");
    }
}
