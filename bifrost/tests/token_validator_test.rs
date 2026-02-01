//! Tests for Phase 4.2.2: TokenValidator (signature, expiration, revocation).

use bifrost::security::{KeyGenerator, TokenGenerator, TokenValidator};
use std::collections::HashSet;
use std::time::Duration;

#[test]
fn validate_accepts_valid_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let token = gen
        .generate_access_token("user-1", "dev-1", Duration::from_secs(3600))
        .unwrap();
    let validator = TokenValidator::new(kp.public_key().clone());
    let result = validator.validate(token.token_string(), None);
    assert!(result.is_ok());
    let validated = result.unwrap();
    assert_eq!(validated.sub(), "user-1");
    assert_eq!(validated.device_id(), "dev-1");
    assert_eq!(validated.token_type(), "access");
}

#[test]
fn validate_rejects_expired_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let token = gen
        .generate_access_token("u", "d", Duration::from_secs(0))
        .unwrap();
    let validator = TokenValidator::new(kp.public_key().clone());
    let result = validator.validate(token.token_string(), None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().to_lowercase().contains("expir"));
}

#[test]
fn validate_rejects_invalid_signature() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let other_kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let token = gen
        .generate_access_token("u", "d", Duration::from_secs(3600))
        .unwrap();
    let validator = TokenValidator::new(other_kp.public_key().clone());
    let result = validator.validate(token.token_string(), None);
    assert!(result.is_err());
}

#[test]
fn validate_rejects_revoked_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let gen = TokenGenerator::new(kp.public_key().clone(), kp.secret_key().clone());
    let token = gen
        .generate_access_token("u", "d", Duration::from_secs(3600))
        .unwrap();
    let validator = TokenValidator::new(kp.public_key().clone());
    let validated = validator.validate(token.token_string(), None).unwrap();
    let mut revoked = HashSet::new();
    revoked.insert(validated.jti().to_string());
    let result = validator.validate(token.token_string(), Some(&revoked));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().to_lowercase().contains("revok"));
}

#[test]
fn validate_rejects_malformed_token() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let validator = TokenValidator::new(kp.public_key().clone());
    let result = validator.validate("not-a-valid-token", None);
    assert!(result.is_err());
}

#[test]
fn validator_new_accepts_public_key() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let _ = TokenValidator::new(kp.public_key().clone());
}
