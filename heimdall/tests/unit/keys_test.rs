#[cfg(test)]
mod tests {
    use heimdall::keys::*;
    use ring::signature::KeyPair;
    use tempfile::TempDir;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_generate_ed25519_keypair() {
        let generator = KeyGenerator::new();
        let (keypair, _) = generator.generate_ed25519_keypair().unwrap();
        
        // Verify keypair has public key
        let public_key = keypair.public_key();
        assert_eq!(public_key.as_ref().len(), 32); // Ed25519 public key is 32 bytes
    }

    #[test]
    fn test_generate_random_bytes() {
        let generator = KeyGenerator::new();
        let bytes1 = generator.generate_random_bytes(32).unwrap();
        let bytes2 = generator.generate_random_bytes(32).unwrap();
        
        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        // Should be different (very high probability)
        assert_ne!(bytes1, bytes2);
    }

    #[tokio::test]
    async fn test_store_and_load_keypair() {
        let temp_dir = TempDir::new().unwrap();
        let keys_dir = temp_dir.path().to_path_buf();
        let storage = SecureKeyStorage::new(keys_dir.clone());
        
        let generator = KeyGenerator::new();
        let (keypair, pkcs8) = generator.generate_ed25519_keypair().unwrap();
        
        // Store keypair
        storage.store_keypair("test_key", &pkcs8).unwrap();
        
        // Load keypair
        let loaded_keypair = storage.load_keypair("test_key").unwrap();
        
        // Verify public keys match
        assert_eq!(
            keypair.public_key().as_ref(),
            loaded_keypair.public_key().as_ref()
        );
    }

    #[tokio::test]
    async fn test_encrypted_key_storage() {
        let temp_dir = TempDir::new().unwrap();
        let keys_dir = temp_dir.path().to_path_buf();
        let storage = SecureKeyStorage::new(keys_dir.clone());
        
        let generator = KeyGenerator::new();
        let (keypair, pkcs8) = generator.generate_ed25519_keypair().unwrap();
        
        // Store keypair (should be encrypted)
        storage.store_keypair("encrypted_key", &pkcs8).unwrap();
        
        // Verify the stored file is not plaintext PKCS8
        let key_file = keys_dir.join("encrypted_key.key");
        let encrypted_data = std::fs::read_to_string(&key_file).unwrap();
        let encrypted_bytes = general_purpose::STANDARD
            .decode(encrypted_data.trim())
            .unwrap();
        
        // Should have salt (16) + encrypted data + tag (16) = at least 32 bytes
        assert!(encrypted_bytes.len() >= 32);
        
        // Try to parse as PKCS8 should fail (it's encrypted)
        assert!(ring::signature::Ed25519KeyPair::from_pkcs8(&encrypted_bytes).is_err());
        
        // But loading through storage should work
        let loaded_keypair = storage.load_keypair("encrypted_key").unwrap();
        assert_eq!(
            keypair.public_key().as_ref(),
            loaded_keypair.public_key().as_ref()
        );
    }

    #[tokio::test]
    async fn test_load_public_key() {
        let temp_dir = TempDir::new().unwrap();
        let keys_dir = temp_dir.path().to_path_buf();
        let storage = SecureKeyStorage::new(keys_dir.clone());
        
        let generator = KeyGenerator::new();
        let (keypair, pkcs8) = generator.generate_ed25519_keypair().unwrap();
        
        // Store keypair
        storage.store_keypair("test_key", &pkcs8).unwrap();
        
        // Load public key
        let public_key_bytes = storage.load_public_key("test_key").unwrap();
        
        // Verify public key matches
        assert_eq!(keypair.public_key().as_ref(), public_key_bytes.as_slice());
    }

    #[test]
    fn test_sign_and_verify() {
        let generator = KeyGenerator::new();
        let (keypair, _) = generator.generate_ed25519_keypair().unwrap();
        
        let message = b"test message";
        
        // Sign message
        let signature = SignatureManager::sign(&keypair, message).unwrap();
        assert_eq!(signature.len(), 64); // Ed25519 signature is 64 bytes
        
        // Verify signature
        let public_key = keypair.public_key();
        SignatureManager::verify(public_key.as_ref(), message, &signature).unwrap();
        
        // Verify with wrong message should fail
        let wrong_message = b"wrong message";
        assert!(SignatureManager::verify(public_key.as_ref(), wrong_message, &signature).is_err());
    }
}
