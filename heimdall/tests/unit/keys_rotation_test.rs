//! Tests for KeyRotationManager (Phase 3.3.1): rotation, grace period, cleanup.

use heimdall::keys::*;
use ring::signature::KeyPair;
use std::time::Duration;
use tempfile::TempDir;

fn rotation_manager_with_short_grace(
    temp_dir: &TempDir,
    rotation_interval_secs: u64,
    grace_period_secs: u64,
) -> KeyRotationManager {
    let keys_dir = temp_dir.path().to_path_buf();
    let storage = SecureKeyStorage::new(keys_dir.clone());
    KeyRotationManager::new(
        KeyGenerator::new(),
        storage,
        keys_dir,
        Duration::from_secs(rotation_interval_secs),
        Duration::from_secs(grace_period_secs),
    )
}

#[test]
fn test_rotate_key_generates_new_key() {
    let temp_dir = TempDir::new().unwrap();
    let keys_dir = temp_dir.path().to_path_buf();
    let storage = SecureKeyStorage::new(keys_dir.clone());
    let gen = KeyGenerator::new();
    let (initial_key, initial_pkcs8) = gen.generate_ed25519_keypair().unwrap();
    storage.store_keypair("sig", &initial_pkcs8).unwrap();

    let mgr = KeyRotationManager::new(
        gen,
        storage,
        keys_dir,
        Duration::from_secs(90 * 24 * 3600),
        Duration::from_secs(7 * 24 * 3600),
    );

    mgr.rotate_key("sig").unwrap();
    let current = mgr.get_current_keypair("sig").unwrap().unwrap();
    assert_ne!(
        initial_key.public_key().as_ref(),
        current.public_key().as_ref(),
        "after rotation current key should differ from initial"
    );
}

#[test]
fn test_after_rotation_old_key_is_deprecated() {
    let temp_dir = TempDir::new().unwrap();
    let keys_dir = temp_dir.path().to_path_buf();
    let storage = SecureKeyStorage::new(keys_dir.clone());
    let gen = KeyGenerator::new();
    let (first_key, first_pkcs8) = gen.generate_ed25519_keypair().unwrap();
    storage.store_keypair("sig", &first_pkcs8).unwrap();

    let mgr = KeyRotationManager::new(
        gen,
        storage,
        keys_dir,
        Duration::from_secs(90 * 24 * 3600),
        Duration::from_secs(7 * 24 * 3600),
    );
    mgr.rotate_key("sig").unwrap();

    let current = mgr.get_current_keypair("sig").unwrap().unwrap();
    let deprecated = mgr.get_deprecated_keypair("sig").unwrap().unwrap();
    assert_ne!(current.public_key().as_ref(), deprecated.public_key().as_ref());
    assert_eq!(first_key.public_key().as_ref(), deprecated.public_key().as_ref());
}

#[test]
fn test_should_rotate_when_no_key() {
    let temp_dir = TempDir::new().unwrap();
    let mgr = rotation_manager_with_short_grace(&temp_dir, 3600, 60);

    assert!(mgr.should_rotate("sig").unwrap());
}

#[test]
fn test_should_rotate_after_interval() {
    let temp_dir = TempDir::new().unwrap();
    let keys_dir = temp_dir.path().to_path_buf();
    let storage = SecureKeyStorage::new(keys_dir.clone());
    let gen = KeyGenerator::new();
    let (_kp, kp_pkcs8) = gen.generate_ed25519_keypair().unwrap();
    storage.store_keypair("sig", &kp_pkcs8).unwrap();

    let mgr = KeyRotationManager::new(
        gen,
        storage,
        keys_dir.clone(),
        Duration::from_secs(1),
        Duration::from_secs(10),
    );
    let rotated_at_path = keys_dir.join(".rotated_at.sig");
    let old_ts = 0u64;
    std::fs::write(&rotated_at_path, old_ts.to_string()).unwrap();

    assert!(mgr.should_rotate("sig").unwrap());
}

#[test]
fn test_cleanup_deprecated_removes_after_grace() {
    let temp_dir = TempDir::new().unwrap();
    let keys_dir = temp_dir.path().to_path_buf();
    let storage = SecureKeyStorage::new(keys_dir.clone());
    let gen = KeyGenerator::new();
    let (_kp, kp_pkcs8) = gen.generate_ed25519_keypair().unwrap();
    storage.store_keypair("sig", &kp_pkcs8).unwrap();

    let mgr = KeyRotationManager::new(
        gen,
        storage,
        keys_dir.clone(),
        Duration::from_secs(3600),
        Duration::from_secs(1),
    );
    mgr.rotate_key("sig").unwrap();
    assert!(mgr.get_deprecated_keypair("sig").unwrap().is_some());

    std::thread::sleep(Duration::from_secs(2));
    mgr.cleanup_deprecated("sig").unwrap();
    assert!(mgr.get_deprecated_keypair("sig").unwrap().is_none());
}
