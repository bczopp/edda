//! Tests for Phase 3.3.1: KeyGenerator (Ed25519 key-pair generation).

use bifrost::security::key_generator::KeyGenerator;

#[test]
fn generate_ed25519_returns_keypair() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    assert_eq!(kp.public_key().len(), 32);
    assert_eq!(kp.secret_key().len(), 32);
}

#[test]
fn generate_ed25519_produces_different_keys_each_time() {
    let kp1 = KeyGenerator::generate_ed25519().unwrap();
    let kp2 = KeyGenerator::generate_ed25519().unwrap();
    assert_ne!(kp1.public_key(), kp2.public_key());
    assert_ne!(kp1.secret_key(), kp2.secret_key());
}

#[test]
fn ed25519_keypair_exposes_public_and_secret() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let public = kp.public_key();
    let secret = kp.secret_key();
    assert_eq!(public.len(), 32);
    assert_eq!(secret.len(), 32);
}

#[test]
fn key_generator_new_exists() {
    let _ = KeyGenerator::new();
}
