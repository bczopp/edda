#[cfg(test)]
mod tests {
    use mimir::encryption::manager::{EncryptionManager, EncryptionError};
    use ring::rand::{SecureRandom, SystemRandom};

    fn generate_key() -> Vec<u8> {
        let mut key = vec![0u8; 32]; // AES-256 requires 32 bytes
        let rng = SystemRandom::new();
        rng.fill(&mut key).unwrap();
        key
    }

    #[test]
    fn test_encryption_manager_creation() {
        let key = generate_key();
        let manager = EncryptionManager::new(&key);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_encryption_decryption() {
        let key = generate_key();
        let manager = EncryptionManager::new(&key).unwrap();
        
        let original_data = b"Hello, Mimir! This is test data.";
        let encrypted = manager.encrypt(original_data).unwrap();
        
        // Encrypted data should be different from original
        assert_ne!(encrypted, original_data);
        
        // Decrypt should recover original
        let decrypted = manager.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original_data);
    }

    #[test]
    fn test_encryption_different_data() {
        let key = generate_key();
        let manager = EncryptionManager::new(&key).unwrap();
        
        let data1 = b"First piece of data";
        let data2 = b"Second piece of data";
        
        let encrypted1 = manager.encrypt(data1).unwrap();
        let encrypted2 = manager.encrypt(data2).unwrap();
        
        // Different data should produce different encrypted output
        assert_ne!(encrypted1, encrypted2);
        
        // Both should decrypt correctly
        assert_eq!(manager.decrypt(&encrypted1).unwrap(), data1);
        assert_eq!(manager.decrypt(&encrypted2).unwrap(), data2);
    }

    #[test]
    fn test_encryption_wrong_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        
        let manager1 = EncryptionManager::new(&key1).unwrap();
        let manager2 = EncryptionManager::new(&key2).unwrap();
        
        let data = b"Secret data";
        let encrypted = manager1.encrypt(data).unwrap();
        
        // Decrypting with wrong key should fail
        let result = manager2.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_encryption_empty_data() {
        let key = generate_key();
        let manager = EncryptionManager::new(&key).unwrap();
        
        let empty_data = b"";
        let encrypted = manager.encrypt(empty_data).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, empty_data);
    }

    #[test]
    fn test_encryption_large_data() {
        let key = generate_key();
        let manager = EncryptionManager::new(&key).unwrap();
        
        let large_data = vec![0u8; 10000];
        let encrypted = manager.encrypt(&large_data).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, large_data);
    }
}
