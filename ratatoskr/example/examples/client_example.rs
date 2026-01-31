//! Example: Ratatoskr Client Usage
//!
//! This example demonstrates how to use the Ratatoskr protocol as a client
//! to establish a connection and send business requests.

use ratatoskr_example::messages::*;
use ratatoskr_example::protocol::*;
use ratatoskr_example::proto::ratatoskr::*;
use ed25519_dalek::SigningKey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize components
    let serializer = MessageSerializer::new();
    let validator = MessageValidator::new();
    let nonce_manager = NonceManager::new();
    let signing_key = SigningKey::generate(&mut rand::thread_rng());
    let signer = MessageSigner::new(signing_key.clone());
    let verifying_key = signer.verifying_key();

    // Step 1: Create connection request
    let mut connection_request = RatatoskrRequest::new_connection_request(
        "req-001".to_string(),
        "device-123".to_string(),
        "user-456".to_string(),
        "device-identity-123".to_string(),
        "auth-token-789".to_string(),
        "1.0.0".to_string(),
    );

    // Step 2: Add nonce
    connection_request.nonce = nonce_manager.generate_nonce();

    // Step 3: Sign the request
    signer.sign_request(&mut connection_request)?;

    // Step 4: Validate the request
    validator.validate_request(&connection_request)?;

    // Step 5: Serialize the request
    let serialized = serializer.serialize_request(&connection_request)?;
    println!("Serialized connection request: {} bytes", serialized.len());

    // Step 6: Deserialize (simulating server receiving)
    let deserialized = serializer.deserialize_request(&serialized)?;
    println!("Deserialized request ID: {}", deserialized.request_id);

    // Step 7: Verify signature
    signer.verify_request(&deserialized, &verifying_key)?;
    println!("Signature verified successfully!");

    // Step 8: Create a business request
    let mut business_request = RatatoskrRequest::new_business_request(
        "req-002".to_string(),
        "device-123".to_string(),
        "user-456".to_string(),
        b"business payload".to_vec(),
    );

    business_request.nonce = nonce_manager.generate_nonce();
    signer.sign_request(&mut business_request)?;
    validator.validate_request(&business_request)?;

    let serialized_business = serializer.serialize_request(&business_request)?;
    println!("Serialized business request: {} bytes", serialized_business.len());

    Ok(())
}
