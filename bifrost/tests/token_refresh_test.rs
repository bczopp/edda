//! Tests for Phase 4.2.3: TokenRefreshManager (validate refresh, new tokens, proactive renewal).

use bifrost::security::{KeyGenerator, TokenGenerator, TokenRefreshManager, TokenValidator};
use std::collections::HashSet;
use std::time::Duration;

#[test]
fn refresh_valid_refresh_token_returns_new_access_and_refresh() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let refresh = gen
        .generate_refresh_token("user-1", "dev-1", Duration::from_secs(86400))
        .unwrap();
    let manager = TokenRefreshManager::new(
        TokenValidator::new(kp.public_key().clone()),
        TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone()),
        Duration::from_secs(3600),
        Duration::from_secs(86400),
    );
    let result = manager.refresh(refresh.token_string(), None);
    assert!(result.is_ok());
    let (access, new_refresh) = result.unwrap();
    assert!(!access.token_string().is_empty());
    assert!(!new_refresh.token_string().is_empty());
    assert_ne!(access.token_string(), refresh.token_string());
}

#[test]
fn refresh_rejects_access_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let access = gen
        .generate_access_token("u", "d", Duration::from_secs(3600))
        .unwrap();
    let manager = TokenRefreshManager::new(
        TokenValidator::new(kp.public_key().clone()),
        TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone()),
        Duration::from_secs(3600),
        Duration::from_secs(86400),
    );
    let result = manager.refresh(access.token_string(), None);
    assert!(result.is_err());
}

#[test]
fn refresh_rejects_revoked_refresh_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let refresh = gen
        .generate_refresh_token("u", "d", Duration::from_secs(86400))
        .unwrap();
    let validator = TokenValidator::new(kp.public_key().clone());
    let validated = validator.validate(refresh.token_string(), None).unwrap();
    let mut revoked = HashSet::new();
    revoked.insert(validated.jti().to_string());
    let manager = TokenRefreshManager::new(
        validator,
        TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone()),
        Duration::from_secs(3600),
        Duration::from_secs(86400),
    );
    let result = manager.refresh(refresh.token_string(), Some(&revoked));
    assert!(result.is_err());
}

#[test]
fn should_renew_proactively_returns_true_when_near_expiry() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let manager = TokenRefreshManager::new(
        TokenValidator::new(kp.public_key().clone()),
        TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone()),
        Duration::from_secs(3600),
        Duration::from_secs(86400),
    );
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let expires_at = now_secs + 30;
    let should = manager.should_renew_proactively(expires_at, Duration::from_secs(60));
    assert!(should);
}

#[test]
fn should_renew_proactively_returns_false_when_far_from_expiry() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let manager = TokenRefreshManager::new(
        TokenValidator::new(kp.public_key().clone()),
        TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone()),
        Duration::from_secs(3600),
        Duration::from_secs(86400),
    );
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let expires_at = now_secs + 3600;
    let should = manager.should_renew_proactively(expires_at, Duration::from_secs(60));
    assert!(!should);
}

#[test]
fn manager_new_accepts_validator_and_generator() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let _ = TokenRefreshManager::new(
        TokenValidator::new(kp.public_key().clone()),
        TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone()),
        Duration::from_secs(3600),
        Duration::from_secs(86400),
    );
}
