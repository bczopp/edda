# Ratatoskr Protocol Documentation

## Overview

Ratatoskr is a WebSocket-based business protocol for secure communication between user devices and Yggdrasil. It is specifically designed for business logic operations such as Marketplace transactions, Payments, and Provider Registration.

## Protocol Features

- **WebSocket-based**: Persistent connections for efficient communication
- **TLS 1.3 Encryption**: Transport-layer security
- **Message Signing**: All messages are digitally signed using Ed25519
- **Nonce-based Replay Protection**: Prevents replay attacks
- **Binary Protocol**: Efficient Protobuf-based message format
- **Connection Management**: Handshake protocol for connection establishment

## Message Types

### Connection Management

- `CONNECTION_REQUEST`: Client requests connection to server
- `CONNECTION_RESPONSE`: Server responds to connection request
- `HEARTBEAT`: Keep-alive messages
- `DISCONNECT`: Graceful connection termination
- `ERROR`: Error messages

### Business Operations

- `BUSINESS_REQUEST`: Generic business request - payload is interpreted by the receiving service (Nornen, Heidrun, etc.). Ratatoskr only handles transport, not business logic.

## Message Format

### RatatoskrRequest

```protobuf
message RatatoskrRequest {
    MessageType message_type = 1;
    string request_id = 2;
    string device_id = 3;
    string user_id = 4;
    int64 timestamp = 5;
    bytes nonce = 6;
    bytes signature = 7;
    bytes payload = 8;
    map<string, string> metadata = 9;
}
```

### RatatoskrResponse

```protobuf
message RatatoskrResponse {
    MessageType message_type = 1;
    string request_id = 2;
    int64 timestamp = 3;
    bool success = 4;
    string error_code = 5;
    string error_message = 6;
    bytes payload = 7;
    map<string, string> metadata = 8;
}
```

## Connection Flow

### 1. Connection Establishment

1. **Client sends CONNECTION_REQUEST**
   - Includes device identity and authentication token
   - Generates and includes nonce
   - Signs the request

2. **Server validates request**
   - Validates schema (required fields)
   - Validates nonce (check for replay attacks)
   - Verifies signature
   - Validates timestamp
   - Authenticates device/user

3. **Server sends CONNECTION_RESPONSE**
   - Accepts or rejects connection
   - If accepted, includes session ID and expiration
   - Signed response

4. **Connection established**
   - Client and server can now exchange business messages
   - Heartbeat messages maintain connection

### 2. Business Message Flow

1. **Client creates business request**
   - Sets message type to `BUSINESS_REQUEST`
   - Includes generic payload with business data (format determined by receiving service)
   - Generates nonce
   - Signs message

2. **Client sends request**
   - Serializes request to binary format
   - Sends over WebSocket connection

3. **Server receives and validates**
   - Deserializes request
   - Validates schema, nonce, signature, timestamp (Ratatoskr responsibility)
   - Forwards payload to appropriate service (Nornen, Heidrun, etc.) for business logic processing

4. **Server sends response**
   - Receives result from business service
   - Creates response with result
   - Serializes and sends over WebSocket

## Security

### Message Signing

All messages are signed using Ed25519 digital signatures:

1. Create message hash: SHA256(request_id + device_id + user_id + timestamp + nonce + payload)
2. Sign hash with private key
3. Include signature in message
4. Verify signature on receiving side

### Nonce Management

- Each request includes a unique nonce (minimum 8 bytes, recommended 16 bytes)
- Server tracks used nonces to prevent replay attacks
- Nonces must be unique per request

### Timestamp Validation

- Timestamps must be within acceptable range (default: 5 minutes)
- Prevents replay of old messages
- Rejects messages with future timestamps

## Usage Examples

See `examples/client_example.rs` and `examples/server_example.rs` for complete usage examples.

## Error Handling

All protocol operations return `Result` types for proper error handling:

- `SerializationError`: Message serialization/deserialization failures
- `ValidationError`: Schema, nonce, signature, or timestamp validation failures
- `ConnectionError`: Connection protocol errors
- `SecurityError`: Security-related errors (signature verification, replay attacks)

## Best Practices

1. **Always validate messages**: Use `MessageValidator` before processing
2. **Check nonces**: Prevent replay attacks by validating nonces
3. **Verify signatures**: Always verify message signatures
4. **Handle errors gracefully**: Implement proper error handling and logging
5. **Use secure key management**: Store signing keys securely
6. **Monitor connections**: Implement heartbeat and connection monitoring
