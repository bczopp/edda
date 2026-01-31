use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Protocol handling failed: {0}")]
    HandlingFailed(String),
}

pub struct ProtocolHandler;

impl ProtocolHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle_message(&self, message: &[u8]) -> Result<Vec<u8>, ProtocolError> {
        // TODO: Handle protocol messages
        Ok(message.to_vec())
    }
}
