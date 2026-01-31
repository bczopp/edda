//! Tests for TokenRenewalManager (Phase 6.3.1): proactive renewal, renewal via refresh token.

use heimdall::token::{TokenGenerator, TokenPayload, TokenRenewalManager, TokenValidator};
use heimdall::utils::config::TokenConfiguration;
use heimdall::keys::KeyGenerator;
use std::sync::Arc;
use tempfile::TempDir;

fn token_config() -> TokenConfiguration {
    TokenConfiguration {
        heimdall_token_expiration_hours: 24,
        session_token_expiration_hours: 1,
        refresh_token_expiration_days: 30,
        proactive_renewal_minutes: 5,
    }
}

fn setup_renewal_manager(temp_dir: &TempDir) -> TokenRenewalManager {
    let keys_dir = temp_dir.path().to_path_buf();
    let (keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
    let storage = heimdall::keys::SecureKeyStorage::new(keys_dir.clone());
    storage.store_keypair("heimdall", &pkcs8).unwrap();

    let config = token_config();
    let validator = Arc::new(TokenValidator::new(keys_dir.clone()));
    let generator = TokenGenerator::new(Arc::new(keypair), config.clone());
    TokenRenewalManager::new(
        validator,
        generator,
        config.proactive_renewal_minutes * 60,
    )
}

#[test]
fn test_should_renew_when_expires_within_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let mgr = setup_renewal_manager(&temp_dir);
    let now = chrono::Utc::now().timestamp();
    let payload = TokenPayload {
        token_id: "id".to_string(),
        device_id: "dev".to_string(),
        user_id: "user".to_string(),
        token_type: "session".to_string(),
        issued_at: now - 3600,
        expires_at: now + 120, // 2 minutes from now
        permissions: vec![],
    };
    assert!(mgr.should_renew(&payload));
}

#[test]
fn test_should_not_renew_when_expires_later_than_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let mgr = setup_renewal_manager(&temp_dir);
    let now = chrono::Utc::now().timestamp();
    let payload = TokenPayload {
        token_id: "id".to_string(),
        device_id: "dev".to_string(),
        user_id: "user".to_string(),
        token_type: "session".to_string(),
        issued_at: now - 3600,
        expires_at: now + 600, // 10 minutes from now
        permissions: vec![],
    };
    assert!(!mgr.should_renew(&payload));
}

#[tokio::test]
async fn test_renew_heimdall_with_refresh_token_returns_new_token() {
    let temp_dir = TempDir::new().unwrap();
    let keys_dir = temp_dir.path().to_path_buf();
    let (keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
    let storage = heimdall::keys::SecureKeyStorage::new(keys_dir.clone());
    storage.store_keypair("heimdall", &pkcs8).unwrap();

    let config = token_config();
    let generator = TokenGenerator::new(std::sync::Arc::new(keypair), config.clone());
    let refresh_token_data = generator
        .generate_refresh_token("device-1", "user-1")
        .unwrap();
    let refresh_token = refresh_token_data.0;

    let validator = Arc::new(TokenValidator::new(keys_dir));
    let mgr = TokenRenewalManager::new(
        validator,
        generator,
        config.proactive_renewal_minutes * 60,
    );

    let (new_token, token_id, expires_at) = mgr
        .renew_heimdall_with_refresh_token(&refresh_token)
        .await
        .unwrap();

    assert!(!new_token.is_empty());
    assert!(!token_id.is_empty());
    assert!(expires_at > chrono::Utc::now().timestamp());
    assert!(new_token.contains('.'));
}

#[tokio::test]
async fn test_renew_with_invalid_refresh_token_returns_error() {
    let temp_dir = TempDir::new().unwrap();
    let mgr = setup_renewal_manager(&temp_dir);

    let res = mgr.renew_heimdall_with_refresh_token("invalid-token").await;
    assert!(res.is_err());
}
