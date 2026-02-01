//! Tests for Phase 4.2.1: TokenGenerator (Heimdall-style token, expiration, refresh token).

use bifrost::security::{KeyGenerator, TokenGenerator};
use std::time::Duration;

#[test]
fn generate_access_token_returns_token_with_expiration() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let token = gen
        .generate_access_token("user-1", "device-1", Duration::from_secs(3600))
        .unwrap();
    assert!(!token.token_string().is_empty());
    assert!(token.expires_at() > 0);
}

#[test]
fn generate_refresh_token_returns_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let token = gen
        .generate_refresh_token("user-1", "device-1", Duration::from_secs(86400))
        .unwrap();
    assert!(!token.token_string().is_empty());
    assert!(token.expires_at() > 0);
}

#[test]
fn access_token_expires_before_refresh_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let access = gen
        .generate_access_token("u", "d", Duration::from_secs(60))
        .unwrap();
    let refresh = gen
        .generate_refresh_token("u", "d", Duration::from_secs(3600))
        .unwrap();
    assert!(access.expires_at() < refresh.expires_at());
}

#[test]
fn token_generator_new_accepts_keys() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let _ = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
}

#[test]
fn generated_tokens_differ() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let t1 = gen.generate_access_token("u", "d", Duration::from_secs(60)).unwrap();
    let t2 = gen.generate_access_token("u", "d", Duration::from_secs(60)).unwrap();
    assert_ne!(t1.token_string(), t2.token_string());
}
