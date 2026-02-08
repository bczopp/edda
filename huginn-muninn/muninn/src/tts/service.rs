//! TTS Service Trait for abstracting TTS implementations

use shared::{AudioBuffer, AudioFormat};
use async_trait::async_trait;
use tokio::sync::mpsc;

/// Result of TTS synthesis
#[derive(Debug, Clone)]
pub struct TtsResult {
    pub audio_buffer: AudioBuffer,
    pub duration_ms: u64,
}

/// Streaming chunk for TTS synthesis
#[derive(Debug, Clone)]
pub struct TtsStreamChunk {
    pub audio_data: Vec<u8>,
    pub is_last: bool,
}

/// TTS Service Trait - Abstract interface for TTS implementations
#[async_trait]
pub trait TTSService: Send + Sync {
    /// Synthesize text to speech
    /// 
    /// # Arguments
    /// * `text` - Text to synthesize
    /// * `language` - Language code (e.g., "en-US", "de-DE")
    /// * `voice` - Voice identifier (e.g., "male", "female", "neutral")
    /// 
    /// # Returns
    /// * `TtsResult` - Audio buffer and duration
    async fn synthesize(
        &self,
        text: &str,
        language: &str,
        voice: &str,
    ) -> Result<TtsResult, Box<dyn std::error::Error>>;
    
    /// Synthesize text to speech with streaming support
    /// 
    /// # Arguments
    /// * `text` - Text to synthesize
    /// * `language` - Language code (e.g., "en-US", "de-DE")
    /// * `voice` - Voice identifier (e.g., "male", "female", "neutral")
    /// 
    /// # Returns
    /// * `mpsc::Receiver<TtsStreamChunk>` - Stream of audio chunks
    async fn synthesize_stream(
        &self,
        text: &str,
        language: &str,
        voice: &str,
    ) -> Result<mpsc::Receiver<TtsStreamChunk>, Box<dyn std::error::Error>>;
    
    /// Get list of supported voices
    /// 
    /// # Returns
    /// * `Vec<String>` - List of supported voice identifiers
    fn get_supported_voices(&self) -> Vec<String>;
    
    /// Get list of supported languages
    /// 
    /// # Returns
    /// * `Vec<String>` - List of supported language codes
    fn get_supported_languages(&self) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test helper: Mock implementation
    struct MockTTSService;
    
    #[async_trait]
    impl TTSService for MockTTSService {
        async fn synthesize(
            &self,
            text: &str,
            _language: &str,
            _voice: &str,
        ) -> Result<TtsResult, Box<dyn std::error::Error>> {
            if text.is_empty() {
                return Err("Text cannot be empty".into());
            }
            
            let audio_buffer = AudioBuffer::new(
                vec![0i16; 16000],
                AudioFormat::Pcm16kHz16BitMono,
            );
            
            Ok(TtsResult {
                audio_buffer,
                duration_ms: 1000,
            })
        }
        
        async fn synthesize_stream(
            &self,
            _text: &str,
            _language: &str,
            _voice: &str,
        ) -> Result<mpsc::Receiver<TtsStreamChunk>, Box<dyn std::error::Error>> {
            let (tx, rx) = mpsc::channel(10);
            tokio::spawn(async move {
                let _ = tx.send(TtsStreamChunk {
                    audio_data: vec![0u8; 32000],
                    is_last: true,
                }).await;
            });
            Ok(rx)
        }
        
        fn get_supported_voices(&self) -> Vec<String> {
            vec!["male".to_string(), "female".to_string()]
        }
        
        fn get_supported_languages(&self) -> Vec<String> {
            vec!["en-US".to_string()]
        }
    }
    
    #[tokio::test]
    async fn test_trait_synthesize() {
        let service = MockTTSService;
        let result = service.synthesize("test", "en-US", "female").await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_trait_synthesize_stream() {
        let service = MockTTSService;
        let mut rx = service.synthesize_stream("test", "en-US", "female").await.unwrap();
        let chunk = rx.recv().await;
        assert!(chunk.is_some());
    }
    
    #[test]
    fn test_trait_get_supported_voices() {
        let service = MockTTSService;
        let voices = service.get_supported_voices();
        assert!(!voices.is_empty());
    }
    
    #[test]
    fn test_trait_get_supported_languages() {
        let service = MockTTSService;
        let languages = service.get_supported_languages();
        assert!(!languages.is_empty());
    }
}
