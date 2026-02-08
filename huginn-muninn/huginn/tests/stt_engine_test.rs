//! Tests for STT Engine

use huginn::stt::{SttEngine, SttConfig, SttResult};
use shared::{AudioBuffer, AudioFormat};

#[test]
fn test_stt_engine_creation() {
    let config = SttConfig {
        language: "en-US".to_string(),
        model_path: None,
    };
    
    let engine = SttEngine::new(config);
    assert_eq!(engine.language(), "en-US");
}

#[tokio::test]
async fn test_stt_engine_transcribe_empty() {
    let config = SttConfig {
        language: "en-US".to_string(),
        model_path: None,
    };
    
    let engine = SttEngine::new(config);
    let buffer = AudioBuffer::new(vec![], AudioFormat::Pcm16kHz16BitMono);
    
    let result = engine.transcribe(buffer).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stt_engine_transcribe_valid_audio() {
    let config = SttConfig {
        language: "en-US".to_string(),
        model_path: None,
    };
    
    let engine = SttEngine::new(config);
    
    // Create sample audio buffer (1 second of silence)
    let samples = vec![0i16; 16000];
    let buffer = AudioBuffer::new(samples, AudioFormat::Pcm16kHz16BitMono);
    
    // For now, this will return a mock result until we integrate actual STT
    let result = engine.transcribe(buffer).await;
    assert!(result.is_ok());
}

#[test]
fn test_stt_config_default() {
    let config = SttConfig::default();
    assert_eq!(config.language, "en-US");
    assert!(config.model_path.is_none());
}

#[tokio::test]
async fn test_stt_result_properties() {
    let result = SttResult {
        text: "Hello world".to_string(),
        confidence: 0.95,
        duration_ms: 1000,
    };
    
    assert_eq!(result.text, "Hello world");
    assert_eq!(result.confidence, 0.95);
    assert_eq!(result.duration_ms, 1000);
}
