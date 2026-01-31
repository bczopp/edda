#[cfg(test)]
mod tests {
    use ratatoskr_example::messages::*;
    use ratatoskr_example::protocol::security::{MessageSigner, NonceManager};
    use ratatoskr_example::proto::ratatoskr::*;
    use ed25519_dalek::{SigningKey, VerifyingKey};

    #[test]
    fn test_generate_nonce() {
        // Test: Generate a nonce
        let nonce_manager = NonceManager::new();
        let nonce = nonce_manager.generate_nonce();

        assert!(!nonce.is_empty());
        assert!(nonce.len() >= 8);
    }

    #[test]
    fn test_generate_unique_nonces() {
        // Test: Generate multiple nonces - they should be unique
        let nonce_manager = NonceManager::new();
        let nonce1 = nonce_manager.generate_nonce();
        let nonce2 = nonce_manager.generate_nonce();

        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_validate_nonce_not_seen() {
        // Test: Validate a nonce that hasn't been seen before
        let nonce_manager = NonceManager::new();
        let nonce = nonce_manager.generate_nonce();

        let result = nonce_manager.validate_and_record_nonce(&nonce);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_nonce_replay_attack() {
        // Test: Validate a nonce that was already used (replay attack)
        let nonce_manager = NonceManager::new();
        let nonce = nonce_manager.generate_nonce();

        // First use - should succeed
        let result1 = nonce_manager.validate_and_record_nonce(&nonce);
        assert!(result1.is_ok());

        // Second use - should fail (replay attack)
        let result2 = nonce_manager.validate_and_record_nonce(&nonce);
        assert!(result2.is_err());
    }

    #[test]
    fn test_sign_message() {
        // Test: Sign a message
        let mut request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        let signer = MessageSigner::new(signing_key);

        let result = signer.sign_request(&mut request);
        assert!(result.is_ok());
        assert!(!request.signature.is_empty());
    }

    #[test]
    fn test_verify_message_signature() {
        // Test: Verify a message signature
        let mut request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        let verifying_key = signing_key.verifying_key();
        let signer = MessageSigner::new(signing_key);

        // Sign the message
        signer.sign_request(&mut request).unwrap();

        // Verify the signature
        let result = signer.verify_request(&request, &verifying_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_invalid_signature() {
        // Test: Verify a message with invalid signature
        let mut request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        let wrong_key = SigningKey::generate(&mut rand::thread_rng());
        let wrong_verifying_key = wrong_key.verifying_key();
        let signer = MessageSigner::new(signing_key);

        // Sign the message
        signer.sign_request(&mut request).unwrap();

        // Verify with wrong key - should fail
        let result = signer.verify_request(&request, &wrong_verifying_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_sign_and_verify_roundtrip() {
        // Test: Full sign and verify roundtrip
        let mut request = RatatoskrRequest::new_connection_request(
            "req-123".to_string(),
            "device-456".to_string(),
            "user-789".to_string(),
            "device-identity-123".to_string(),
            "token-456".to_string(),
            "1.0.0".to_string(),
        );

        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        let verifying_key = signing_key.verifying_key();
        let signer = MessageSigner::new(signing_key);

        // Add nonce
        let nonce_manager = NonceManager::new();
        request.nonce = nonce_manager.generate_nonce();

        // Sign
        signer.sign_request(&mut request).unwrap();

        // Verify
        let result = signer.verify_request(&request, &verifying_key);
        assert!(result.is_ok());
    }
}
