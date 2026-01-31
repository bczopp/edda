use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Token counting failed: {0}")]
    CountingFailed(String),
}

pub struct TokenCounter;

impl TokenCounter {
    pub fn new() -> Self {
        Self
    }

    pub async fn count_tokens(&self, text: &str) -> Result<u64, TokenError> {
        // TODO: Implement proper token counting (tiktoken, etc.)
        // Simple approximation: ~4 characters per token
        Ok((text.len() as f64 / 4.0).ceil() as u64)
    }
}
