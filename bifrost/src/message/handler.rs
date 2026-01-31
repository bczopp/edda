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
}

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

pub struct MessageHandler;

impl MessageHandler {
    pub fn parse_message(data: &str) -> Result<BifrostMessage, Box<dyn std::error::Error + Send + Sync>> {
        let message: BifrostMessage = serde_json::from_str(data)?;
        Ok(message)
    }

    pub fn serialize_message(message: &BifrostMessage) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string(message)?;
        Ok(json)
    }
}
