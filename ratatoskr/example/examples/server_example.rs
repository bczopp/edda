//! Example: Ratatoskr Server Usage
//!
//! This example demonstrates how to use the Ratatoskr protocol as a server
//! to handle connection requests and business requests.

use ratatoskr_example::messages::*;
use ratatoskr_example::protocol::*;
use ratatoskr_example::proto::ratatoskr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize components
    let serializer = MessageSerializer::new();
    let validator = MessageValidator::new();
    let connection_protocol = ConnectionProtocol::new();
    let nonce_manager = NonceManager::new();

    // Simulate receiving a connection request
    let connection_request = RatatoskrRequest::new_connection_request(
        "req-001".to_string(),
        "device-123".to_string(),
        "user-456".to_string(),
        "device-identity-123".to_string(),
        "auth-token-789".to_string(),
        "1.0.0".to_string(),
    );

    // Step 1: Deserialize request (in real scenario, this comes from WebSocket)
    let serialized = serializer.serialize_request(&connection_request)?;
    let received_request = serializer.deserialize_request(&serialized)?;

    // Step 2: Validate request
    validator.validate_request(&received_request)?;
    println!("Request validated successfully");

    // Step 3: Check nonce (prevent replay attacks)
    nonce_manager.validate_and_record_nonce(&received_request.nonce)?;
    println!("Nonce validated - no replay attack detected");

    // Step 4: Parse connection request payload
    let payload = connection_protocol.parse_connection_request_payload(&received_request)?;
    println!("Device Identity: {}", payload.device_identity);
    println!("Version: {}", payload.version);

    // Step 5: Authenticate (simplified - in real scenario, verify token with Heimdall)
    let authenticated = payload.authentication_token == "auth-token-789";

    // Step 6: Create connection response
    let connection_response = if authenticated {
        connection_protocol.create_connection_response(
            &received_request,
            true,
            "session-12345".to_string(),
            1234567890,
            "1.0.0".to_string(),
        )?
    } else {
        connection_protocol.create_connection_response(
            &received_request,
            false,
            String::new(),
            0,
            "1.0.0".to_string(),
        )?
    };

    // Step 7: Validate response
    validator.validate_response(&connection_response)?;

    // Step 8: Serialize response
    let serialized_response = serializer.serialize_response(&connection_response)?;
    println!("Connection response: {} bytes", serialized_response.len());
    println!("Connection accepted: {}", connection_response.success);

    Ok(())
}
