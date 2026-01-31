//! Tests for DeviceIdentityVerifier (Phase 5.2.1): public key validation, identity verification.

use heimdall::auth::DeviceIdentityVerifier;
use heimdall::keys::{KeyGenerator, SignatureManager};
use base64::{Engine as _, engine::general_purpose};
use ring::signature::KeyPair;

const ED25519_PUBLIC_KEY_LEN: usize = 32;

#[test]
fn test_validate_public_key_accepts_32_bytes() {
    let verifier = DeviceIdentityVerifier;
    let key = [1u8; ED25519_PUBLIC_KEY_LEN]; // non-zero; all-zero is rejected as invalid
    assert!(verifier.validate_public_key(&key).is_ok());
}

#[test]
fn test_validate_public_key_rejects_wrong_length() {
    let verifier = DeviceIdentityVerifier;
    assert!(verifier.validate_public_key(&[]).is_err());
    assert!(verifier.validate_public_key(&[0u8; 31]).is_err());
    assert!(verifier.validate_public_key(&[0u8; 33]).is_err());
}

#[test]
fn test_validate_public_key_base64_accepts_valid() {
    let verifier = DeviceIdentityVerifier;
    let key = [1u8; ED25519_PUBLIC_KEY_LEN];
    let b64 = general_purpose::STANDARD.encode(&key);
    assert!(verifier.validate_public_key_base64(&b64).is_ok());
}

#[test]
fn test_validate_public_key_base64_rejects_invalid() {
    let verifier = DeviceIdentityVerifier;
    assert!(verifier.validate_public_key_base64("").is_err());
    assert!(verifier.validate_public_key_base64("not-base64!!!").is_err());
    let short = general_purpose::STANDARD.encode(&[0u8; 16]);
    assert!(verifier.validate_public_key_base64(&short).is_err());
}

#[test]
fn test_verify_identity_success() {
    let verifier = DeviceIdentityVerifier;
    let gen = KeyGenerator::new();
    let (keypair, _) = gen.generate_ed25519_keypair().unwrap();
    let message = b"hello device";
    let signature = SignatureManager::sign(&keypair, message).unwrap();

    let public_key = keypair.public_key().as_ref();
    assert!(verifier.verify_identity(public_key, message, &signature).unwrap());
}

#[test]
fn test_verify_identity_fails_wrong_message() {
    let verifier = DeviceIdentityVerifier;
    let gen = KeyGenerator::new();
    let (keypair, _) = gen.generate_ed25519_keypair().unwrap();
    let message = b"hello device";
    let signature = SignatureManager::sign(&keypair, message).unwrap();

    let public_key = keypair.public_key().as_ref();
    assert!(!verifier.verify_identity(public_key, b"wrong message", &signature).unwrap());
}

#[test]
fn test_verify_identity_fails_invalid_public_key() {
    let verifier = DeviceIdentityVerifier;
    let invalid_key = [0u8; ED25519_PUBLIC_KEY_LEN];
    assert!(verifier.verify_identity(&invalid_key, b"msg", &[0u8; 64]).is_err());
}
