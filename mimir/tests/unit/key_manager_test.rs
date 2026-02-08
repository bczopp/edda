//! Unit tests for Key Manager

use mimir::encryption::KeyManager;
use std::time::Duration;

#[tokio::test]
async fn test_key_manager_current_key() {
    let manager = KeyManager::new_with_random_key();
    let key1 = manager.get_current_key().await;
    let key2 = manager.get_current_key().await;
    
    // Same key should be returned
    assert_eq!(key1.len(), 32);
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_key_manager_key_rotation() {
    let mut manager = KeyManager::new_with_random_key();
    let key1 = manager.get_current_key().await;
    
    // Rotate key
    manager.rotate_key().await.expect("rotate_key");
    let key2 = manager.get_current_key().await;
    
    // Keys should be different
    assert_ne!(key1, key2);
    assert_eq!(key2.len(), 32);
}

#[tokio::test]
async fn test_key_manager_historical_keys() {
    let mut manager = KeyManager::new_with_random_key();
    let key1 = manager.get_current_key().await.clone();
    
    manager.rotate_key().await.expect("rotate_key");
    let key2 = manager.get_current_key().await.clone();
    
    // Both keys should be accessible
    let historical = manager.get_historical_keys().await;
    assert!(historical.contains(&key1));
    assert_eq!(historical.len(), 1); // Only previous keys
    
    // Current key should still be key2
    assert_eq!(manager.get_current_key().await, &key2);
}

#[tokio::test]
async fn test_key_manager_max_historical_keys() {
    let mut manager = KeyManager::new_with_random_key();
    
    // Rotate multiple times
    for _ in 0..5 {
        manager.rotate_key().await.expect("rotate_key");
    }
    
    let historical = manager.get_historical_keys().await;
    
    // Should keep only max_historical_keys (default: 3)
    assert!(historical.len() <= 3);
}

#[tokio::test]
async fn test_key_manager_get_key_by_version() {
    let mut manager = KeyManager::new_with_random_key();
    let version1 = manager.get_current_version().await;
    let key1 = manager.get_current_key().await.clone();
    
    manager.rotate_key().await.expect("rotate_key");
    let version2 = manager.get_current_version().await;
    let key2 = manager.get_current_key().await.clone();
    
    // Should be able to retrieve keys by version
    let retrieved_key1 = manager.get_key_by_version(version1).await;
    let retrieved_key2 = manager.get_key_by_version(version2).await;
    
    assert_eq!(retrieved_key1, Some(&key1));
    assert_eq!(retrieved_key2, Some(&key2));
}

#[tokio::test]
async fn test_key_manager_automatic_rotation() {
    let mut manager = KeyManager::new_with_random_key();
    manager.enable_automatic_rotation(Duration::from_millis(100));
    
    let key1 = manager.get_current_key().await.clone();
    
    // Wait for automatic rotation
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    let key2 = manager.get_current_key().await;
    
    // Key should have been rotated automatically
    assert_ne!(&key1, key2);
    
    manager.disable_automatic_rotation();
}
