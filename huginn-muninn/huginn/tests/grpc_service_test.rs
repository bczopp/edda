//! Tests for Huginn gRPC Service

use huginn::grpc::HuginnMediaServiceImpl;
use tonic::Request;

// Include generated proto code
pub mod huginn {
    tonic::include_proto!("huginn");
}

pub mod raven {
    tonic::include_proto!("raven");
}

#[tokio::test]
async fn test_huginn_service_creation() {
    let service = HuginnMediaServiceImpl::new();
    assert!(service.is_ready());
}

#[tokio::test]
async fn test_forward_text_request() {
    let service = HuginnMediaServiceImpl::new();
    
    let request = Request::new(huginn::ForwardTextRequest {
        text: "Hello Odin".to_string(),
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
        metadata: std::collections::HashMap::new(),
    });
    
    let response = service.forward_text(request).await;
    assert!(response.is_ok());
    
    let response = response.unwrap().into_inner();
    assert!(response.success);
}

#[tokio::test]
async fn test_forward_text_empty() {
    let service = HuginnMediaServiceImpl::new();
    
    let request = Request::new(huginn::ForwardTextRequest {
        text: String::new(),
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
        metadata: std::collections::HashMap::new(),
    });
    
    let response = service.forward_text(request).await;
    // Should fail for empty text
    assert!(response.is_err() || !response.unwrap().into_inner().success);
}

#[tokio::test]
async fn test_transcribe_audio_request() {
    let service = HuginnMediaServiceImpl::new();
    
    // Create sample audio data (1 second of silence at 16kHz)
    let audio_data = vec![0u8; 16000 * 2]; // 16-bit samples
    
    let request = Request::new(huginn::TranscribeAudioRequest {
        audio_data,
        audio_format: "wav".to_string(),
        sample_rate: 16000,
        channels: 1,
        language: "en-US".to_string(),
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
    });
    
    let response = service.transcribe_audio(request).await;
    assert!(response.is_ok());
    
    let response = response.unwrap().into_inner();
    assert!(response.success);
}

#[tokio::test]
async fn test_forward_image_request() {
    let service = HuginnMediaServiceImpl::new();
    
    // Create minimal 1x1 PNG image data
    let image_data = vec![0u8; 100];
    
    let request = Request::new(huginn::ForwardImageRequest {
        image_data,
        image_format: "png".to_string(),
        width: 1,
        height: 1,
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
        metadata: std::collections::HashMap::new(),
    });
    
    let response = service.forward_image(request).await;
    assert!(response.is_ok());
    
    let response = response.unwrap().into_inner();
    assert!(response.success);
}

#[tokio::test]
async fn test_forward_video_request() {
    let service = HuginnMediaServiceImpl::new();
    
    // Create minimal video data
    let video_data = vec![0u8; 1000];
    
    let request = Request::new(huginn::ForwardVideoRequest {
        video_data,
        video_format: "mp4".to_string(),
        duration_ms: 1000,
        user_id: "test_user".to_string(),
        device_id: "test_device".to_string(),
        metadata: std::collections::HashMap::new(),
    });
    
    let response = service.forward_video(request).await;
    assert!(response.is_ok());
    
    let response = response.unwrap().into_inner();
    assert!(response.success);
}
