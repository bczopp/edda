//! StreamingHandler trait (Phase 9.1.1, TDD).

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StreamingError {
    #[error("Streaming not supported")]
    NotSupported,
    #[error("IO error: {0}")]
    Io(String),
}

/// Interface for video/audio streaming (preparation for plugins).
#[async_trait]
pub trait StreamingHandler: Send + Sync {
    async fn send_video_stream(&self, data: &[u8]) -> Result<(), StreamingError>;
    async fn send_audio_stream(&self, data: &[u8]) -> Result<(), StreamingError>;
    async fn receive_video_stream(&self) -> Result<Vec<u8>, StreamingError>;
    async fn receive_audio_stream(&self) -> Result<Vec<u8>, StreamingError>;
}
