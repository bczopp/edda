use ratatoskr::protocol::{NonceManager, MessageSigner};
use ratatoskr::messages::*;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[tokio::test]
async fn test_nonce_generation() {
    let manager = NonceManager::new();
    let nonce1 = manager.generate_nonce();
    let nonce2 = manager.generate_nonce();
    
    assert_eq!(nonce1.len(), 16);
    assert_eq!(nonce2.len(), 16);
    assert_ne!(nonce1, nonce2);
}

#[tokio::test]
async fn test_nonce_replay_detection() {
    let manager = NonceManager::new();
    let nonce = manager.generate_nonce();
    
    // First use should succeed
    let result1 = manager.validate_and_record_nonce(&nonce);
    assert!(result1.is_ok());
    
    // Second use should fail (replay attack)
    let result2 = manager.validate_and_record_nonce(&nonce);
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("replay"));
}

#[tokio::test]
async fn test_message_signing_and_verification() {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let signer = MessageSigner::new(signing_key);
    
    let mut request = RatatoskrRequest::new_business_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        b"payload".to_vec(),
    );
    
    // Generate nonce
    let nonce_manager = NonceManager::new();
    request.nonce = nonce_manager.generate_nonce();
    
    // Sign the request
    signer.sign_request(&mut request).unwrap();
    
    // Verify the signature
    let result = signer.verify_request(&request, &verifying_key);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_message_signing_fails_with_wrong_key() {
    let signing_key1 = SigningKey::generate(&mut OsRng);
    let signing_key2 = SigningKey::generate(&mut OsRng);
    let verifying_key2 = signing_key2.verifying_key();
    let signer = MessageSigner::new(signing_key1);
    
    let mut request = RatatoskrRequest::new_business_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        b"payload".to_vec(),
    );
    
    // Generate nonce
    let nonce_manager = NonceManager::new();
    request.nonce = nonce_manager.generate_nonce();
    
    // Sign with key1
    signer.sign_request(&mut request).unwrap();
    
    // Try to verify with key2 (should fail)
    let result = signer.verify_request(&request, &verifying_key2);
    assert!(result.is_err());
}
