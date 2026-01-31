//! Tests for MessageValidator (Phase 9.5.1): signature verification, reject invalid messages.

use heimdall::bifrost::MessageValidator;
use heimdall::keys::{KeyGenerator, SignatureManager};
use ring::signature::KeyPair;

#[test]
fn test_verify_message_valid_signature() {
    let validator = MessageValidator;
    let gen = KeyGenerator::new();
    let (keypair, _) = gen.generate_ed25519_keypair().unwrap();
    let message = b"hello bifrost";
    let signature = SignatureManager::sign(&keypair, message).unwrap();
    let public_key = keypair.public_key().as_ref();
    assert!(validator.verify_message(public_key, message, &signature).unwrap());
}

#[test]
fn test_verify_message_invalid_signature_returns_false() {
    let validator = MessageValidator;
    let gen = KeyGenerator::new();
    let (keypair, _) = gen.generate_ed25519_keypair().unwrap();
    let public_key = keypair.public_key().as_ref();
    assert!(!validator.verify_message(public_key, b"message", &[0u8; 64]).unwrap());
}

#[test]
fn test_verify_message_wrong_message_returns_false() {
    let validator = MessageValidator;
    let gen = KeyGenerator::new();
    let (keypair, _) = gen.generate_ed25519_keypair().unwrap();
    let signature = SignatureManager::sign(&keypair, b"correct").unwrap();
    let public_key = keypair.public_key().as_ref();
    assert!(!validator.verify_message(public_key, b"wrong", &signature).unwrap());
}

#[test]
fn test_verify_message_invalid_public_key_returns_error() {
    let validator = MessageValidator;
    let invalid_key = [0u8; 31];
    let res = validator.verify_message(&invalid_key, b"msg", &[0u8; 64]);
    assert!(res.is_err());
}
