//! E2E-Test: Store → Encrypt → Audit → Retrieve → Decrypt (Phase 12.1.1).
//!
//! Verifies the full pipeline: store (encrypted + audited), then retrieve (decrypted)
//! and that both operations are reflected in the audit log.

#[cfg(test)]
mod tests {
    use mimir::storage::database::EncryptedDatabase;
    use mimir::encryption::EncryptionManager;
    use mimir::access_control::{AccessControlManager, Role};
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
    async fn test_e2e_store_encrypt_audit_retrieve_decrypt() {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();

        let access_control = Arc::new(AccessControlManager::new());
        let audit_logger = Arc::new(AuditLogManager::new(test_db.pool.clone()));
        access_control.register_user("user-e2e".to_string(), Role::User).await;

        let database = EncryptedDatabase::new_with_access_control_and_audit(
            &test_db.pool,
            encryption_manager,
            access_control,
            audit_logger.clone(),
        )
        .await
        .unwrap();

        let user_id = "user-e2e";
        let original_data = b"E2E secret payload: Store -> Encrypt -> Audit -> Retrieve -> Decrypt";

        // Store (encrypt + audit)
        let data_id = database
            .store_data_with_access_control(user_id, original_data, user_id)
            .await
            .unwrap();
        assert!(!data_id.is_empty());

        // Verify audit: DataStored
        let logs_after_store = audit_logger.get_user_audit_logs(user_id).await.unwrap();
        assert!(
            logs_after_store
                .iter()
                .any(|log| matches!(log.event_type, AuditEvent::DataStored)),
            "Audit must contain DataStored after store"
        );

        // Retrieve (decrypt)
        let retrieved = database
            .retrieve_data_with_access_control(&data_id, user_id, user_id)
            .await
            .unwrap();

        // Decrypted content must match original
        assert_eq!(retrieved, original_data, "Retrieved data must equal original (decryption ok)");

        // Verify audit: DataRetrieved
        let logs_after_retrieve = audit_logger.get_user_audit_logs(user_id).await.unwrap();
        assert!(
            logs_after_retrieve
                .iter()
                .any(|log| matches!(log.event_type, AuditEvent::DataRetrieved)),
            "Audit must contain DataRetrieved after retrieve"
        );
    }
}
