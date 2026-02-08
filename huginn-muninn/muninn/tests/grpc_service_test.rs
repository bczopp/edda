//! Tests for Muninn gRPC Service

use muninn::grpc::MuninnTtsServiceImpl;
use tonic::Request;

// Include generated proto code
pub mod muninn {
    tonic::include_proto!("muninn");
}

pub mod raven {
    tonic::include_proto!("raven");
}

#[tokio::test]
async fn test_muninn_service_creation() {
    let service = MuninnTtsServiceImpl::new();
    assert!(service.is_ready());
}

#[tokio::test]
async fn test_generate_speech_request() {
    let service = MuninnTtsServiceImpl::new();
    
    let request = Request::new(muninn::TtsRequest {
        text: "Hello world".to_string(),
        language: "en-US".to_string(),
        voice: muninn::TtsVoice::Female as i32,
        settings: Some(muninn::TtsSettings {
            speed: 1.0,
            pitch: 0.0,
            volume: 1.0,
            audio_format: "wav".to_string(),
            sample_rate: 16000,
        }),
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
    });
    
    let response = service.generate_speech(request).await;
    assert!(response.is_ok());
    
    let response = response.unwrap().into_inner();
    assert!(response.success);
    assert!(!response.audio_data.is_empty());
}

#[tokio::test]
async fn test_generate_speech_empty_text() {
    let service = MuninnTtsServiceImpl::new();
    
    let request = Request::new(muninn::TtsRequest {
        text: String::new(),
        language: "en-US".to_string(),
        voice: muninn::TtsVoice::Female as i32,
        settings: Some(muninn::TtsSettings {
            speed: 1.0,
            pitch: 0.0,
            volume: 1.0,
            audio_format: "wav".to_string(),
            sample_rate: 16000,
        }),
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
    });
    
    let response = service.generate_speech(request).await;
    // Should fail for empty text
    assert!(response.is_err());
}

#[tokio::test]
async fn test_generate_speech_different_voices() {
    let service = MuninnTtsServiceImpl::new();
    
    for voice in &[muninn::TtsVoice::Male, muninn::TtsVoice::Female, muninn::TtsVoice::Neutral] {
        let request = Request::new(muninn::TtsRequest {
            text: "Test".to_string(),
            language: "en-US".to_string(),
            voice: *voice as i32,
            settings: Some(muninn::TtsSettings {
                speed: 1.0,
                pitch: 0.0,
                volume: 1.0,
                audio_format: "wav".to_string(),
                sample_rate: 16000,
            }),
            user_id: "test_user".to_string(),
            device_id: "test_device".to_string(),
        });
        
        let response = service.generate_speech(request).await;
        assert!(response.is_ok());
    }
}

#[tokio::test]
async fn test_generate_speech_custom_settings() {
    let service = MuninnTtsServiceImpl::new();
    
    let request = Request::new(muninn::TtsRequest {
        text: "Fast speech".to_string(),
        language: "en-US".to_string(),
        voice: muninn::TtsVoice::Female as i32,
        settings: Some(muninn::TtsSettings {
            speed: 1.5,  // Faster
            pitch: 5.0,  // Higher pitch
            volume: 0.8, // Quieter
            audio_format: "mp3".to_string(),
            sample_rate: 44100,
        }),
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
    });
    
    let response = service.generate_speech(request).await;
    assert!(response.is_ok());
    
    let response = response.unwrap().into_inner();
    assert_eq!(response.audio_format, "mp3");
    assert_eq!(response.sample_rate, 44100);
}
