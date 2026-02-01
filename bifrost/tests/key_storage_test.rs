//! Tests for Phase 3.3.2: KeyStorage (save/load public, encrypted private, key loading).

use bifrost::security::key_storage::KeyStorage;
use std::path::PathBuf;

#[tokio::test]
async fn save_and_load_public_key() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().to_path_buf();
    let storage = KeyStorage::new(path.clone());
    let key_id = "device-1";
    let public = [1u8; 32];

    storage.save_public_key(key_id, &public).await.unwrap();
    let loaded = storage.load_public_key(key_id).await.unwrap();
    assert_eq!(loaded.as_slice(), &public);
}

#[tokio::test]
async fn load_public_key_missing_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let storage = KeyStorage::new(dir.path().to_path_buf());
    let result = storage.load_public_key("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn save_and_load_private_key_encrypted() {
    let dir = tempfile::tempdir().unwrap();
    let storage = KeyStorage::new(dir.path().to_path_buf());
    let key_id = "device-1";
    let secret = [2u8; 32];
    let passphrase = "secret-passphrase";

    storage
        .save_private_key(key_id, &secret, passphrase)
        .await
        .unwrap();
    let loaded = storage.load_private_key(key_id, passphrase).await.unwrap();
    assert_eq!(loaded.as_slice(), &secret);
}

#[tokio::test]
async fn load_private_key_wrong_passphrase_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let storage = KeyStorage::new(dir.path().to_path_buf());
    let key_id = "device-1";
    let secret = [2u8; 32];
    storage
        .save_private_key(key_id, &secret, "correct")
        .await
        .unwrap();

    let result = storage.load_private_key(key_id, "wrong").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn load_private_key_missing_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let storage = KeyStorage::new(dir.path().to_path_buf());
    let result = storage.load_private_key("nonexistent", "pass").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn key_storage_new_accepts_path() {
    let _ = KeyStorage::new(PathBuf::from("/tmp/keys"));
}
