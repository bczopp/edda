//! STT Engine implementation

use shared::{AudioBuffer, AudioError, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttConfig {
    pub language: String,
    pub model_path: Option<String>,
}

impl Default for SttConfig {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            model_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttResult {
    pub text: String,
    pub confidence: f32,
    pub duration_ms: u32,
}

pub struct SttEngine {
    config: SttConfig,
}

impl SttEngine {
    pub fn new(config: SttConfig) -> Self {
        info!("Creating STT engine with language: {}", config.language);
        Self { config }
    }
    
    pub fn language(&self) -> &str {
        &self.config.language
    }
    
    pub async fn transcribe(&self, buffer: AudioBuffer) -> Result<SttResult> {
        if buffer.is_empty() {
            return Err(AudioError::InvalidData("Audio buffer is empty".to_string()));
        }
        
        info!(
            "Transcribing audio: {} frames, {} ms",
            buffer.frames(),
            buffer.duration_ms
        );
        
        // TODO: Integrate actual STT engine (Whisper.cpp)
        // For now, return mock result
        Ok(SttResult {
            text: "[Mock transcription]".to_string(),
            confidence: 0.0,
            duration_ms: buffer.duration_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::AudioFormat;
    
    #[test]
    fn test_stt_engine_new() {
        let config = SttConfig {
            language: "de-DE".to_string(),
            model_path: Some("/path/to/model".to_string()),
        };
        
        let engine = SttEngine::new(config.clone());
        assert_eq!(engine.language(), "de-DE");
    }
    
    #[tokio::test]
    async fn test_transcribe_empty_buffer() {
        let engine = SttEngine::new(SttConfig::default());
        let buffer = AudioBuffer::new(vec![], AudioFormat::Pcm16kHz16BitMono);
        
        let result = engine.transcribe(buffer).await;
        assert!(result.is_err());
    }
}
