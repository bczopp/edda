//! E2E tests for Muninn TTS workflow: RavenMessage/Text → Audio → Output

use muninn::grpc::MuninnTtsServiceImpl;
use tonic::Request;

pub mod muninn {
    tonic::include_proto!("muninn");
}

/// E2E: TTS-Workflow — Text → Audio → Output (Muninn returns audio; output device is downstream)
#[tokio::test]
async fn e2e_tts_workflow_text_to_audio_output() {
    let service = MuninnTtsServiceImpl::new();

    let request = Request::new(muninn::TtsRequest {
        text: "E2E TTS workflow test".to_string(),
        language: "en-US".to_string(),
        voice: muninn::TtsVoice::Female as i32,
        settings: Some(muninn::TtsSettings {
            speed: 1.0,
            pitch: 0.0,
            volume: 1.0,
            audio_format: "wav".to_string(),
            sample_rate: 16000,
        }),
        user_id: "e2e_user".to_string(),
        device_id: "e2e_device".to_string(),
    });

    let response = service.generate_speech(request).await;
    assert!(response.is_ok(), "TTS workflow should succeed");

    let inner = response.unwrap().into_inner();
    assert!(inner.success, "TTS response success");
    assert!(!inner.audio_data.is_empty(), "Audio data present");
    assert!(inner.duration_ms >= 0, "Duration reported");
}

/// E2E: TTS-Workflow — empty text must fail
#[tokio::test]
async fn e2e_tts_workflow_empty_text_fails() {
    let service = MuninnTtsServiceImpl::new();

    let request = Request::new(muninn::TtsRequest {
        text: String::new(),
        language: "en-US".to_string(),
        voice: muninn::TtsVoice::Male as i32,
        settings: None,
        user_id: "e2e_user".to_string(),
        device_id: "e2e_device".to_string(),
    });

    let response = service.generate_speech(request).await;
    assert!(response.is_err(), "Empty text should yield error");
}
