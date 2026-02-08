use heimdall::security::e2e::{E2EEncryptionManager, E2EEncryptionError};

#[tokio::test]
async fn test_e2e_manager_new() {
    let manager = E2EEncryptionManager::new();
    assert!(true);
}

#[tokio::test]
async fn test_generate_session_key() {
    let manager = E2EEncryptionManager::new();
    
    // Generate session key
    let result = manager.generate_session_key().await;
    assert!(result.is_ok());
    
    let session_key = result.unwrap();
    
    // Session key should be 32 bytes (AES-256)
    assert_eq!(session_key.len(), 32);
    
    // Generate another key and verify they are different (randomness)
    let session_key2 = manager.generate_session_key().await.unwrap();
    assert_ne!(session_key, session_key2, "Session keys should be unique");
}

#[tokio::test]
async fn test_perform_key_exchange() {
    let manager_alice = E2EEncryptionManager::new();
    let manager_bob = E2EEncryptionManager::new();
    
    // Generate keypairs for Alice and Bob
    let alice_keypair = manager_alice.generate_keypair().await.unwrap();
    let bob_keypair = manager_bob.generate_keypair().await.unwrap();
    
    // Perform key exchange: Alice with Bob's public key
    let alice_shared_secret = manager_alice.perform_key_exchange(
        &alice_keypair.private_key,
        &bob_keypair.public_key
    ).await;
    assert!(alice_shared_secret.is_ok());
    
    // Perform key exchange: Bob with Alice's public key
    let bob_shared_secret = manager_bob.perform_key_exchange(
        &bob_keypair.private_key,
        &alice_keypair.public_key
    ).await;
    assert!(bob_shared_secret.is_ok());
    
    // Both should derive the same shared secret (ECDH property)
    assert_eq!(alice_shared_secret.unwrap(), bob_shared_secret.unwrap());
}

#[tokio::test]
async fn test_encrypt_message() {
    let manager = E2EEncryptionManager::new();
    
    // Generate session key
    let session_key = manager.generate_session_key().await.unwrap();
    
    // Encrypt message
    let plaintext = b"Hello, World!";
    let result = manager.encrypt_message(&session_key, plaintext).await;
    assert!(result.is_ok());
    
    let (ciphertext, nonce) = result.unwrap();
    
    // Ciphertext should be different from plaintext
    assert_ne!(ciphertext.as_slice(), plaintext);
    
    // Nonce should be 12 bytes (GCM standard)
    assert_eq!(nonce.len(), 12);
}

#[tokio::test]
async fn test_decrypt_message() {
    let manager = E2EEncryptionManager::new();
    
    // Generate session key
    let session_key = manager.generate_session_key().await.unwrap();
    
    // Encrypt message
    let plaintext = b"Hello, World!";
    let (ciphertext, nonce) = manager.encrypt_message(&session_key, plaintext).await.unwrap();
    
    // Decrypt message
    let result = manager.decrypt_message(&session_key, &ciphertext, &nonce).await;
    assert!(result.is_ok());
    
    let decrypted = result.unwrap();
    
    // Decrypted should match original plaintext
    assert_eq!(decrypted.as_slice(), plaintext);
}

#[tokio::test]
async fn test_encrypt_decrypt_roundtrip() {
    let manager = E2EEncryptionManager::new();
    
    // Generate session key
    let session_key = manager.generate_session_key().await.unwrap();
    
    // Test various message sizes
    let test_messages = vec![
        b"Short".to_vec(),
        b"Medium length message for testing encryption".to_vec(),
        vec![0u8; 1024], // 1KB
        vec![0xAA; 4096], // 4KB with specific byte pattern
    ];
    
    for plaintext in test_messages {
        // Encrypt
        let (ciphertext, nonce) = manager.encrypt_message(&session_key, &plaintext).await.unwrap();
        
        // Decrypt
        let decrypted = manager.decrypt_message(&session_key, &ciphertext, &nonce).await.unwrap();
        
        // Verify
        assert_eq!(decrypted, plaintext, "Roundtrip failed for message");
    }
}

#[tokio::test]
async fn test_decrypt_with_wrong_key() {
    let manager = E2EEncryptionManager::new();
    
    // Generate two different session keys
    let session_key1 = manager.generate_session_key().await.unwrap();
    let session_key2 = manager.generate_session_key().await.unwrap();
    
    // Encrypt with key1
    let plaintext = b"Secret message";
    let (ciphertext, nonce) = manager.encrypt_message(&session_key1, plaintext).await.unwrap();
    
    // Try to decrypt with key2 (wrong key)
    let result = manager.decrypt_message(&session_key2, &ciphertext, &nonce).await;
    
    // Should fail
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), E2EEncryptionError::DecryptionFailed(_)));
}

#[tokio::test]
async fn test_decrypt_with_wrong_nonce() {
    let manager = E2EEncryptionManager::new();
    
    // Generate session key
    let session_key = manager.generate_session_key().await.unwrap();
    
    // Encrypt message
    let plaintext = b"Secret message";
    let (ciphertext, _nonce) = manager.encrypt_message(&session_key, plaintext).await.unwrap();
    
    // Generate wrong nonce
    let wrong_nonce = vec![0u8; 12];
    
    // Try to decrypt with wrong nonce
    let result = manager.decrypt_message(&session_key, &ciphertext, &wrong_nonce).await;
    
    // Should fail
    assert!(result.is_err());
}

#[tokio::test]
async fn test_decrypt_with_tampered_ciphertext() {
    let manager = E2EEncryptionManager::new();
    
    // Generate session key
    let session_key = manager.generate_session_key().await.unwrap();
    
    // Encrypt message
    let plaintext = b"Secret message";
    let (mut ciphertext, nonce) = manager.encrypt_message(&session_key, plaintext).await.unwrap();
    
    // Tamper with ciphertext
    if !ciphertext.is_empty() {
        ciphertext[0] ^= 0xFF; // Flip bits
    }
    
    // Try to decrypt tampered ciphertext
    let result = manager.decrypt_message(&session_key, &ciphertext, &nonce).await;
    
    // Should fail (GCM authentication will detect tampering)
    assert!(result.is_err());
}

#[tokio::test]
async fn test_generate_keypair() {
    let manager = E2EEncryptionManager::new();
    
    // Generate keypair
    let result = manager.generate_keypair().await;
    assert!(result.is_ok());
    
    let keypair = result.unwrap();
    
    // Private key should be 32 bytes (Ed25519)
    assert_eq!(keypair.private_key.len(), 32);
    
    // Public key should be 32 bytes (Ed25519)
    assert_eq!(keypair.public_key.len(), 32);
    
    // Keys should be different
    assert_ne!(keypair.private_key, keypair.public_key);
}

#[tokio::test]
async fn test_multiple_sessions_with_different_keys() {
    let manager = E2EEncryptionManager::new();
    
    // Generate multiple session keys
    let key1 = manager.generate_session_key().await.unwrap();
    let key2 = manager.generate_session_key().await.unwrap();
    let key3 = manager.generate_session_key().await.unwrap();
    
    // All keys should be unique
    assert_ne!(key1, key2);
    assert_ne!(key2, key3);
    assert_ne!(key1, key3);
    
    // Each key should work independently
    let plaintext = b"Test message";
    
    let (ciphertext1, nonce1) = manager.encrypt_message(&key1, plaintext).await.unwrap();
    let (ciphertext2, nonce2) = manager.encrypt_message(&key2, plaintext).await.unwrap();
    
    // Decrypt with correct keys
    let decrypted1 = manager.decrypt_message(&key1, &ciphertext1, &nonce1).await.unwrap();
    let decrypted2 = manager.decrypt_message(&key2, &ciphertext2, &nonce2).await.unwrap();
    
    assert_eq!(decrypted1.as_slice(), plaintext);
    assert_eq!(decrypted2.as_slice(), plaintext);
    
    // Cross-decryption should fail
    assert!(manager.decrypt_message(&key1, &ciphertext2, &nonce2).await.is_err());
    assert!(manager.decrypt_message(&key2, &ciphertext1, &nonce1).await.is_err());
}

#[tokio::test]
async fn test_empty_message_encryption() {
    let manager = E2EEncryptionManager::new();
    
    // Generate session key
    let session_key = manager.generate_session_key().await.unwrap();
    
    // Encrypt empty message
    let plaintext = b"";
    let result = manager.encrypt_message(&session_key, plaintext).await;
    assert!(result.is_ok());
    
    let (ciphertext, nonce) = result.unwrap();
    
    // Decrypt
    let decrypted = manager.decrypt_message(&session_key, &ciphertext, &nonce).await.unwrap();
    
    // Should decrypt to empty message
    assert_eq!(decrypted.len(), 0);
}

#[tokio::test]
async fn test_perfect_forward_secrecy() {
    let manager_alice = E2EEncryptionManager::new();
    let manager_bob = E2EEncryptionManager::new();
    
    // Session 1: Generate keypairs and exchange
    let alice_keypair1 = manager_alice.generate_keypair().await.unwrap();
    let bob_keypair1 = manager_bob.generate_keypair().await.unwrap();
    
    let session1_key = manager_alice.perform_key_exchange(
        &alice_keypair1.private_key,
        &bob_keypair1.public_key
    ).await.unwrap();
    
    // Session 2: Generate NEW keypairs and exchange (Perfect Forward Secrecy)
    let alice_keypair2 = manager_alice.generate_keypair().await.unwrap();
    let bob_keypair2 = manager_bob.generate_keypair().await.unwrap();
    
    let session2_key = manager_alice.perform_key_exchange(
        &alice_keypair2.private_key,
        &bob_keypair2.public_key
    ).await.unwrap();
    
    // Session keys should be different (Perfect Forward Secrecy)
    assert_ne!(session1_key, session2_key, "Session keys should be unique for each session (PFS)");
    
    // Keypairs should be different
    assert_ne!(alice_keypair1.private_key, alice_keypair2.private_key);
    assert_ne!(bob_keypair1.private_key, bob_keypair2.private_key);
}
