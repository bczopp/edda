//! Tests for TTS Service Trait

use muninn::tts::service::{TTSService, TtsResult, TtsStreamChunk};
use shared::{AudioBuffer, AudioFormat};
use std::sync::Arc;
use tokio::sync::mpsc;

// Mock implementation for testing
struct MockTTSService;

#[async_trait::async_trait]
impl TTSService for MockTTSService {
    async fn synthesize(&self, text: &str, language: &str, voice: &str) -> Result<TtsResult, Box<dyn std::error::Error>> {
        if text.is_empty() {
            return Err("Text cannot be empty".into());
        }
        
        let audio_buffer = AudioBuffer::new(
            vec![0i16; 16000], // 1 second of silence
            AudioFormat::Pcm16kHz16BitMono,
        );
        
        Ok(TtsResult {
            audio_buffer,
            duration_ms: 1000,
        })
    }
    
    async fn synthesize_stream(
        &self,
        text: &str,
        language: &str,
        voice: &str,
    ) -> Result<mpsc::Receiver<TtsStreamChunk>, Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel(10);
        
        // Send a single chunk
        tokio::spawn(async move {
            let chunk = TtsStreamChunk {
                audio_data: vec![0u8; 32000], // 1 second of audio
                is_last: true,
            };
            let _ = tx.send(chunk).await;
        });
        
        Ok(rx)
    }
    
    fn get_supported_voices(&self) -> Vec<String> {
        vec!["male".to_string(), "female".to_string(), "neutral".to_string()]
    }
    
    fn get_supported_languages(&self) -> Vec<String> {
        vec!["en-US".to_string(), "de-DE".to_string(), "fr-FR".to_string()]
    }
}

#[tokio::test]
async fn test_tts_service_synthesize() {
    let service = MockTTSService;
    let result = service.synthesize("Hello, World!", "en-US", "female").await;
    assert!(result.is_ok());
    
    let tts_result = result.unwrap();
    assert_eq!(tts_result.duration_ms, 1000);
    assert_eq!(tts_result.audio_buffer.samples.len(), 16000);
}

#[tokio::test]
async fn test_tts_service_synthesize_empty_text() {
    let service = MockTTSService;
    let result = service.synthesize("", "en-US", "female").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_tts_service_synthesize_stream() {
    let service = MockTTSService;
    let mut rx = service.synthesize_stream("Hello, World!", "en-US", "female").await.unwrap();
    
    let chunk = rx.recv().await;
    assert!(chunk.is_some());
    let chunk = chunk.unwrap();
    assert!(chunk.is_last);
    assert_eq!(chunk.audio_data.len(), 32000);
}

#[test]
fn test_tts_service_get_supported_voices() {
    let service = MockTTSService;
    let voices = service.get_supported_voices();
    assert_eq!(voices.len(), 3);
    assert!(voices.contains(&"male".to_string()));
    assert!(voices.contains(&"female".to_string()));
    assert!(voices.contains(&"neutral".to_string()));
}

#[test]
fn test_tts_service_get_supported_languages() {
    let service = MockTTSService;
    let languages = service.get_supported_languages();
    assert_eq!(languages.len(), 3);
    assert!(languages.contains(&"en-US".to_string()));
    assert!(languages.contains(&"de-DE".to_string()));
    assert!(languages.contains(&"fr-FR".to_string()));
}
