//! Tests for TokenRotationManager (Phase 6.5.1): regular rotation, event-based rotation.

use heimdall::token::{TokenGenerator, TokenPayload, TokenRevocationManager, TokenRotationManager, TokenValidator};
use heimdall::utils::config::TokenConfiguration;
use heimdall::utils::token_repository::TokenRepository;
use heimdall::keys::KeyGenerator;
use std::sync::Arc;
use tempfile::TempDir;
use chrono::Utc;
use uuid::Uuid;
use crate::common::{TestDatabase, create_test_device};

fn token_config() -> TokenConfiguration {
    TokenConfiguration {
        heimdall_token_expiration_hours: 24,
        session_token_expiration_hours: 1,
        refresh_token_expiration_days: 30,
        proactive_renewal_minutes: 5,
    }
}

fn setup_rotation_manager(
    test_db: &TestDatabase,
    temp_dir: &TempDir,
    rotation_interval_secs: u64,
) -> TokenRotationManager {
    let keys_dir = temp_dir.path().to_path_buf();
    let (keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
    heimdall::keys::SecureKeyStorage::new(keys_dir.clone()).store_keypair("heimdall", &pkcs8).unwrap();
    let config = token_config();
    let validator = Arc::new(TokenValidator::new(keys_dir));
    let generator = TokenGenerator::new(std::sync::Arc::new(keypair), config);
    let revoke_mgr = Arc::new(TokenRevocationManager::new(
        Arc::new(TokenRepository::new(test_db.pool.clone())),
        None,
    ));
    TokenRotationManager::new(validator, generator, revoke_mgr, rotation_interval_secs)
}

#[tokio::test]
async fn test_should_rotate_when_interval_elapsed() {
    let test_db = TestDatabase::new().await.unwrap();
    let temp_dir = TempDir::new().unwrap();
    let mgr = setup_rotation_manager(&test_db, &temp_dir, 24 * 3600);

    let now = Utc::now().timestamp();
    let payload = TokenPayload {
        token_id: Uuid::new_v4().to_string(),
        device_id: "dev".to_string(),
        user_id: "user".to_string(),
        token_type: "heimdall".to_string(),
        issued_at: now - 25 * 3600, // 25 hours ago
        expires_at: now + 3600,
        permissions: vec!["read".to_string()],
    };
    assert!(mgr.should_rotate(&payload));
}

#[tokio::test]
async fn test_should_not_rotate_when_interval_not_elapsed() {
    let test_db = TestDatabase::new().await.unwrap();
    let temp_dir = TempDir::new().unwrap();
    let mgr = setup_rotation_manager(&test_db, &temp_dir, 24 * 3600);

    let now = Utc::now().timestamp();
    let payload = TokenPayload {
        token_id: Uuid::new_v4().to_string(),
        device_id: "dev".to_string(),
        user_id: "user".to_string(),
        token_type: "heimdall".to_string(),
        issued_at: now - 3600, // 1 hour ago
        expires_at: now + 23 * 3600,
        permissions: vec![],
    };
    assert!(!mgr.should_rotate(&payload));
}

#[tokio::test]
async fn test_rotate_heimdall_token_returns_new_token_and_revokes_old() {
    let test_db = TestDatabase::new().await.unwrap();
    let temp_dir = TempDir::new().unwrap();
    let keys_dir = temp_dir.path().to_path_buf();
    let (keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
    heimdall::keys::SecureKeyStorage::new(keys_dir.clone()).store_keypair("heimdall", &pkcs8).unwrap();

    let config = token_config();
    let generator = TokenGenerator::new(std::sync::Arc::new(keypair), config.clone());
    let (old_token_data, old_token_id, _) = generator
        .generate_heimdall_token("device-1", "user-1", vec!["read".to_string()])
        .unwrap();

    let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
    let user_id = Uuid::new_v4();
    let dev_id = Uuid::new_v4().to_string();
    let device = create_test_device(&test_db.pool, &dev_id, user_id).await.unwrap();
    token_repo
        .create(
            &old_token_id,
            device.id,
            user_id,
            "heimdall",
            &old_token_data,
            Utc::now() + chrono::Duration::hours(1),
        )
        .await
        .unwrap();

    let validator = Arc::new(TokenValidator::new(keys_dir));
    let revoke_mgr = Arc::new(TokenRevocationManager::new(token_repo.clone(), None));
    let mgr = TokenRotationManager::new(
        validator,
        generator,
        revoke_mgr,
        24 * 3600,
    );

    let (new_token, new_id, expires_at) = mgr
        .rotate_heimdall_token(&old_token_data)
        .await
        .unwrap();

    assert!(!new_token.is_empty());
    assert_ne!(new_id, old_token_id);
    assert!(expires_at > Utc::now().timestamp());

    let old = token_repo.get_by_token_id(&old_token_id).await.unwrap();
    assert!(old.is_revoked);
}

#[tokio::test]
async fn test_rotate_with_invalid_token_returns_error() {
    let test_db = TestDatabase::new().await.unwrap();
    let temp_dir = TempDir::new().unwrap();
    let keys_dir = temp_dir.path().to_path_buf();
    let (keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
    heimdall::keys::SecureKeyStorage::new(keys_dir.clone()).store_keypair("heimdall", &pkcs8).unwrap();

    let config = token_config();
    let validator = Arc::new(TokenValidator::new(keys_dir));
    let generator = TokenGenerator::new(std::sync::Arc::new(keypair), config);
    let revoke_mgr = Arc::new(TokenRevocationManager::new(
        Arc::new(TokenRepository::new(test_db.pool.clone())),
        None,
    ));
    let mgr = TokenRotationManager::new(validator, generator, revoke_mgr, 24 * 3600);

    let res = mgr.rotate_heimdall_token("invalid-token").await;
    assert!(res.is_err());
}
