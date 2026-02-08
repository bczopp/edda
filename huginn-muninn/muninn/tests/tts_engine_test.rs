//! Tests for TTS Engine

use muninn::tts::{TtsEngine, TtsConfig, TtsVoice};
use shared::{AudioBuffer, AudioFormat};

#[test]
fn test_tts_engine_creation() {
    let config = TtsConfig {
        language: "en-US".to_string(),
        voice: TtsVoice::Male,
        model_path: None,
    };
    
    let engine = TtsEngine::new(config);
    assert_eq!(engine.language(), "en-US");
}

#[tokio::test]
async fn test_tts_engine_synthesize_empty() {
    let config = TtsConfig::default();
    let engine = TtsEngine::new(config);
    
    let result = engine.synthesize("").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_tts_engine_synthesize_valid_text() {
    let config = TtsConfig::default();
    let engine = TtsEngine::new(config);
    
    let result = engine.synthesize("Hello world").await;
    assert!(result.is_ok());
    
    let buffer = result.unwrap();
    assert!(!buffer.is_empty());
}

#[test]
fn test_tts_config_default() {
    let config = TtsConfig::default();
    assert_eq!(config.language, "en-US");
    assert_eq!(config.voice, TtsVoice::Female);
    assert!(config.model_path.is_none());
}

#[test]
fn test_tts_voice_variants() {
    let male = TtsVoice::Male;
    let female = TtsVoice::Female;
    let neutral = TtsVoice::Neutral;
    
    assert_ne!(male, female);
    assert_ne!(female, neutral);
    assert_ne!(male, neutral);
}
