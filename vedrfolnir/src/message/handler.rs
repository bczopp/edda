use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Message handling failed: {0}")]
    HandlingFailed(String),
}

pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, message: &[u8]) -> Result<Vec<u8>, MessageError> {
        // TODO: Handle messages
        Ok(message.to_vec())
    }
}
