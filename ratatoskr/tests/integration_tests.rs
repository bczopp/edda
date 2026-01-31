use ratatoskr::protocol::*;
use ratatoskr::messages::*;
use ratatoskr::proto::ratatoskr::*;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

#[tokio::test]
async fn test_full_protocol_flow() {
    // Setup
    let serializer = MessageSerializer::new();
    let validator = MessageValidator::new();
    let nonce_manager = NonceManager::new();
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let signer = MessageSigner::new(signing_key);
    let connection_protocol = ConnectionProtocol::new();

    // 1. Create connection request
    let mut request = RatatoskrRequest::new_connection_request(
        "req-123".to_string(),
        "device-456".to_string(),
        "user-789".to_string(),
        "device-identity".to_string(),
        "auth-token".to_string(),
        "1.0.0".to_string(),
    );

    // 2. Generate and set nonce
    request.nonce = nonce_manager.generate_nonce();

    // 3. Sign the request
    signer.sign_request(&mut request).unwrap();

    // 4. Validate the request
    validator.validate_request(&request).unwrap();

    // 5. Serialize the request
    let serialized = serializer.serialize_request(&request).unwrap();

    // 6. Deserialize the request
    let deserialized = serializer.deserialize_request(&serialized).unwrap();

    // 7. Verify signature on deserialized request
    signer.verify_request(&deserialized, &verifying_key).unwrap();

    // 8. Validate nonce (prevent replay)
    nonce_manager.validate_and_record_nonce(&deserialized.nonce).unwrap();

    // 9. Parse connection request payload
    let payload = connection_protocol.parse_connection_request_payload(&deserialized).unwrap();
    assert_eq!(payload.device_identity, "device-identity");

    // 10. Create connection response
    let response = connection_protocol.create_connection_response(
        &deserialized,
        true,
        "session-123".to_string(),
        1234567890,
        "1.0.0".to_string(),
    ).unwrap();

    // 11. Validate response
    validator.validate_response(&response).unwrap();

    // 12. Serialize response
    let serialized_response = serializer.serialize_response(&response).unwrap();

    // 13. Deserialize response
    let deserialized_response = serializer.deserialize_response(&serialized_response).unwrap();

    assert_eq!(deserialized_response.success, true);
    assert_eq!(deserialized_response.request_id, "req-123");
}
