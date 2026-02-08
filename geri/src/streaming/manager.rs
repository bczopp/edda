//! Streaming-Manager (Phase 12.1.1): Chunk-basiertes Streaming, Error-Handling.

use thiserror::Error;

/// Fehler beim Streaming (Provider, IO, etc.).
#[derive(Debug, Clone, Error)]
pub enum StreamingError {
    #[error("Stream error: {0}")]
    StreamError(String),
}

impl From<String> for StreamingError {
    fn from(s: String) -> Self {
        StreamingError::StreamError(s)
    }
}

impl StreamingError {
    /// Liefert eine nutzerfreundliche Fehlermeldung (z. B. für TTS/Log).
    pub fn user_message(&self) -> String {
        match self {
            StreamingError::StreamError(msg) => format!("Streaming fehlgeschlagen: {}", msg),
        }
    }
}

/// Sammelt Chunks zu einer Response, stoppt bei erstem Fehler (Error-Handling).
#[derive(Debug, Clone, Copy, Default)]
pub struct StreamingManager;

impl StreamingManager {
    /// Sammelt alle Chunks zu einem String; bei erstem `Err` wird dieser als `StreamingError` zurückgegeben.
    pub fn collect_chunks<E: Into<StreamingError>>(
        self,
        chunks: impl IntoIterator<Item = Result<String, E>>,
    ) -> Result<String, StreamingError> {
        let mut out = String::new();
        for chunk in chunks {
            match chunk {
                Ok(s) => out.push_str(&s),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(out)
    }
}
