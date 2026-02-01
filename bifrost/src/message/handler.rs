use serde::{Deserialize, Serialize};

/// Bifrost protocol message types (Phase 2.1.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    ConnectionRequest,
    ConnectionResponse,
    Message,
    Heartbeat,
    Disconnect,
    Error,
    VersionNegotiation,
    ChallengeRequest,
    ChallengeResponse,
    ChallengeProof,
    AuthenticationResult,
    /// gRPC request tunnel (Phase 17.1.1).
    GrpcRequest,
    /// gRPC response tunnel (Phase 17.1.1).
    GrpcResponse,
}

/// Bifrost protocol message: JSON over WebSocket with message_type, source/target device IDs, payload.
///
/// # Example
///
/// ```
/// use bifrost::message::{BifrostMessage, MessageType};
///
/// let msg = BifrostMessage {
///     message_id: "id-1".to_string(),
///     message_type: MessageType::Heartbeat,
///     source_device_id: "dev-a".to_string(),
///     target_device_id: "server".to_string(),
///     payload: serde_json::json!({}),
///     timestamp: 1700000000,
///     protocol_version: Some(1),
/// };
/// let json = bifrost::message::MessageHandler::serialize_message(&msg).unwrap();
/// assert!(json.contains("HEARTBEAT"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BifrostMessage {
    pub message_id: String,
    pub message_type: MessageType,
    pub source_device_id: String,
    pub target_device_id: String,
    pub payload: serde_json::Value,
    pub timestamp: i64,
    /// Protocol version (Major); None = omit or legacy.
    #[serde(default)]
    pub protocol_version: Option<u32>,
}

/// Parses and serializes Bifrost protocol messages (JSON).
pub struct MessageHandler;

impl MessageHandler {
    /// Parses a JSON string into a [`BifrostMessage`].
    ///
    /// # Example
    ///
    /// ```
    /// use bifrost::message::{BifrostMessage, MessageType, MessageHandler};
    ///
    /// let json = r#"{"message_id":"x","message_type":"HEARTBEAT","source_device_id":"a",
    ///     "target_device_id":"b","payload":{},"timestamp":0}"#;
    /// let msg = MessageHandler::parse_message(json).unwrap();
    /// assert_eq!(msg.message_type, MessageType::Heartbeat);
    /// ```
    pub fn parse_message(data: &str) -> Result<BifrostMessage, Box<dyn std::error::Error + Send + Sync>> {
        let message: BifrostMessage = serde_json::from_str(data)?;
        Ok(message)
    }

    /// Serializes a [`BifrostMessage`] to JSON string.
    ///
    /// # Example
    ///
    /// ```
    /// use bifrost::message::{BifrostMessage, MessageType, MessageHandler};
    ///
    /// let msg = BifrostMessage {
    ///     message_id: "id-1".to_string(),
    ///     message_type: MessageType::Message,
    ///     source_device_id: "a".to_string(),
    ///     target_device_id: "b".to_string(),
    ///     payload: serde_json::json!({"body":"Hi"}),
    ///     timestamp: 0,
    ///     protocol_version: None,
    /// };
    /// let json = MessageHandler::serialize_message(&msg).unwrap();
    /// assert!(json.contains("MESSAGE"));
    /// ```
    pub fn serialize_message(message: &BifrostMessage) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string(message)?;
        Ok(json)
    }
}
