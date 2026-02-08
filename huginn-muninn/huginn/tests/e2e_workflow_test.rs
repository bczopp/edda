//! E2E tests for Huginn workflows: STT (Audio → Text → RavenMessage) and Media (Bild/Video → Huginn → Odin)

use huginn::grpc::HuginnMediaServiceImpl;
use tonic::Request;

pub mod huginn {
    tonic::include_proto!("huginn");
}

/// E2E: STT-Workflow — Audio → Text → RavenMessage (Huginn returns transcription; Odin forwarding is TODO)
#[tokio::test]
async fn e2e_stt_workflow_audio_to_text_to_raven_message() {
    let service = HuginnMediaServiceImpl::new();
    let audio_data = vec![0u8; 32000]; // 1s silence @ 16kHz, 16-bit mono

    let request = Request::new(huginn::TranscribeAudioRequest {
        audio_data,
        audio_format: "wav".to_string(),
        sample_rate: 16000,
        channels: 1,
        language: "en-US".to_string(),
        user_id: "e2e_user".to_string(),
        device_id: "e2e_device".to_string(),
    });

    let response = service.transcribe_audio(request).await;
    assert!(response.is_ok(), "STT workflow should succeed");

    let inner = response.unwrap().into_inner();
    assert!(inner.success, "STT response success");
    let msg = inner.message.expect("RavenMessage present");
    assert!(!msg.message_id.is_empty(), "RavenMessage ID present");
    assert!(!msg.content.is_empty(), "Transcription (content) present");
}

/// E2E: Media-Workflow — Text-Input → Huginn → (Odin)
#[tokio::test]
async fn e2e_media_workflow_text_forward() {
    let service = HuginnMediaServiceImpl::new();

    let request = Request::new(huginn::ForwardTextRequest {
        text: "E2E text input".to_string(),
        user_id: "e2e_user".to_string(),
        device_id: "e2e_device".to_string(),
        metadata: std::collections::HashMap::new(),
    });

    let response = service.forward_text(request).await;
    assert!(response.is_ok(), "Text forward workflow should succeed");

    let inner = response.unwrap().into_inner();
    assert!(inner.success);
    assert!(!inner.message_id.is_empty());
}

/// E2E: Media-Workflow — Bild → Huginn → (Odin)
#[tokio::test]
async fn e2e_media_workflow_image_forward() {
    let service = HuginnMediaServiceImpl::new();
    let image_data = vec![0u8; 256];

    let request = Request::new(huginn::ForwardImageRequest {
        image_data,
        image_format: "png".to_string(),
        width: 16,
        height: 16,
        user_id: "e2e_user".to_string(),
        device_id: "e2e_device".to_string(),
        metadata: std::collections::HashMap::new(),
    });

    let response = service.forward_image(request).await;
    assert!(response.is_ok(), "Image forward workflow should succeed");

    let inner = response.unwrap().into_inner();
    assert!(inner.success);
    assert!(!inner.message_id.is_empty());
}

/// E2E: Media-Workflow — Video → Huginn → (Odin)
#[tokio::test]
async fn e2e_media_workflow_video_forward() {
    let service = HuginnMediaServiceImpl::new();
    let video_data = vec![0u8; 1024];

    let request = Request::new(huginn::ForwardVideoRequest {
        video_data,
        video_format: "mp4".to_string(),
        duration_ms: 5000,
        user_id: "e2e_user".to_string(),
        device_id: "e2e_device".to_string(),
        metadata: std::collections::HashMap::new(),
    });

    let response = service.forward_video(request).await;
    assert!(response.is_ok(), "Video forward workflow should succeed");

    let inner = response.unwrap().into_inner();
    assert!(inner.success);
    assert!(!inner.message_id.is_empty());
}
