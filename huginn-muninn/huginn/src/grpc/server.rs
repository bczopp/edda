//! Huginn gRPC Service Implementation

use tonic::{Request, Response, Status};
use tracing::{info, warn};
use uuid::Uuid;
use chrono;

// Include generated proto code
pub mod huginn {
    tonic::include_proto!("huginn");
}

pub mod raven {
    tonic::include_proto!("raven");
}

use huginn::huginn_media_service_server::HuginnMediaService;
use huginn::*;
use crate::stt::{SttEngine, SttConfig};
use crate::text_input::TextInputProcessor;
use crate::image_input::ImageInputProcessor;
use crate::video_input::VideoInputProcessor;
use std::sync::Arc;

pub struct HuginnMediaServiceImpl {
    stt_engine: Arc<SttEngine>,
    text_processor: Arc<TextInputProcessor>,
    image_processor: Arc<ImageInputProcessor>,
    video_processor: Arc<VideoInputProcessor>,
    video_stream_processor: Arc<VideoStreamProcessor>,
    // Future: Add Odin client, etc.
}

impl HuginnMediaServiceImpl {
    pub fn new() -> Self {
        info!("Creating HuginnMediaServiceImpl");
        Self {
            stt_engine: Arc::new(SttEngine::new(SttConfig::default())),
            text_processor: Arc::new(TextInputProcessor::new()),
            image_processor: Arc::new(ImageInputProcessor::new()),
            video_processor: Arc::new(VideoInputProcessor::new()),
            video_stream_processor: Arc::new(VideoStreamProcessor::new()),
        }
    }
    
    pub fn with_stt_engine(stt_engine: Arc<SttEngine>) -> Self {
        info!("Creating HuginnMediaServiceImpl with custom STT engine");
        Self {
            stt_engine,
            text_processor: Arc::new(TextInputProcessor::new()),
            image_processor: Arc::new(ImageInputProcessor::new()),
            video_processor: Arc::new(VideoInputProcessor::new()),
            video_stream_processor: Arc::new(VideoStreamProcessor::new()),
        }
    }
    
    pub fn is_ready(&self) -> bool {
        true
    }
}

#[tonic::async_trait]
impl HuginnMediaService for HuginnMediaServiceImpl {
    async fn forward_text(
        &self,
        request: Request<ForwardTextRequest>,
    ) -> Result<Response<MediaForwardResponse>, Status> {
        let req = request.into_inner();
        
        if req.text.is_empty() {
            return Err(Status::invalid_argument("Text cannot be empty"));
        }
        
        info!("Forwarding text: {} chars from user {}", req.text.len(), req.user_id);
        
        // Process text input and create RavenMessage
        let raven_message = self.text_processor.process(
            &req.text,
            &req.user_id,
            &req.device_id,
        ).await
        .map_err(|e| Status::internal(format!("Text processing failed: {}", e)))?;
        
        // TODO: Forward to Odin via gRPC
        let message_id = raven_message.message_id.clone();
        
        Ok(Response::new(MediaForwardResponse {
            success: true,
            message_id,
            error_message: String::new(),
        }))
    }
    
    async fn forward_image(
        &self,
        request: Request<ForwardImageRequest>,
    ) -> Result<Response<MediaForwardResponse>, Status> {
        let req = request.into_inner();
        
        // Process image input and create ImageInfo
        let image_info = self.image_processor.process(
            &req.image_data,
            &req.image_format,
            req.width,
            req.height,
            &req.user_id,
            &req.device_id,
        ).await
        .map_err(|e| Status::invalid_argument(format!("Image processing failed: {}", e)))?;
        
        info!(
            "Forwarding image: {} bytes, {}x{} {} from user {} on device {}",
            image_info.size_bytes,
            image_info.width,
            image_info.height,
            image_info.format,
            image_info.user_id,
            image_info.device_id
        );
        
        // TODO: Forward to Odin via gRPC
        let message_id = image_info.message_id.clone();
        
        Ok(Response::new(MediaForwardResponse {
            success: true,
            message_id,
            error_message: String::new(),
        }))
    }
    
    async fn forward_video(
        &self,
        request: Request<ForwardVideoRequest>,
    ) -> Result<Response<MediaForwardResponse>, Status> {
        let req = request.into_inner();
        
        // Process video input and create VideoInfo
        let video_info = self.video_processor.process(
            &req.video_data,
            &req.video_format,
            req.duration_ms,
            &req.user_id,
            &req.device_id,
        ).await
        .map_err(|e| Status::invalid_argument(format!("Video processing failed: {}", e)))?;
        
        info!(
            "Forwarding video: {} bytes, {} ms {} from user {} on device {}",
            video_info.size_bytes,
            video_info.duration_ms,
            video_info.format,
            video_info.user_id,
            video_info.device_id
        );
        
        // TODO: Forward to Odin via gRPC
        let message_id = video_info.message_id.clone();
        
        Ok(Response::new(MediaForwardResponse {
            success: true,
            message_id,
            error_message: String::new(),
        }))
    }
    
    async fn forward_video_stream(
        &self,
        request: Request<tonic::Streaming<ForwardVideoStreamChunk>>,
    ) -> Result<Response<MediaForwardResponse>, Status> {
        let mut stream = request.into_inner();
        
        info!("Forwarding video stream");
        
        let mut chunk_count = 0;
        let mut total_bytes = 0;
        
        while let Some(chunk) = stream.message().await? {
            chunk_count += 1;
            total_bytes += chunk.chunk_data.len();
            
            if chunk.is_last {
                info!("Video stream complete: {} chunks, {} bytes", chunk_count, total_bytes);
                break;
            }
        }
        
        // TODO: Forward to Odin
        let message_id = Uuid::new_v4().to_string();
        
        Ok(Response::new(MediaForwardResponse {
            success: true,
            message_id,
            error_message: String::new(),
        }))
    }
    
    async fn transcribe_audio(
        &self,
        request: Request<TranscribeAudioRequest>,
    ) -> Result<Response<TranscribeAudioResponse>, Status> {
        let req = request.into_inner();
        
        if req.audio_data.is_empty() {
            return Err(Status::invalid_argument("Audio data cannot be empty"));
        }
        
        info!(
            "Transcribing audio: {} bytes, {} Hz, {} channels, {} from user {}",
            req.audio_data.len(),
            req.sample_rate,
            req.channels,
            req.language,
            req.user_id
        );
        
        // Convert audio data to AudioBuffer
        use shared::{AudioBuffer, AudioFormat};
        let samples: Vec<i16> = req.audio_data.chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    i16::from_le_bytes([chunk[0], chunk[1]])
                } else {
                    0i16
                }
            })
            .collect();
        
        let format = match req.audio_format.as_str() {
            "wav" => AudioFormat::Wav,
            "mp3" => AudioFormat::Mp3,
            "opus" => AudioFormat::Opus,
            _ => AudioFormat::Pcm16kHz16BitMono, // Default
        };
        
        let audio_buffer = AudioBuffer::new(samples, format);
        
        // Use STT engine to transcribe
        let stt_result = self.stt_engine.transcribe(audio_buffer).await
            .map_err(|e| Status::internal(format!("STT transcription failed: {}", e)))?;
        
        let message_id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp();
        
        let raven_message = raven::RavenMessage {
            message_id: message_id.clone(),
            direction: raven::MessageDirection::Incoming as i32,
            content: stt_result.text,
            metadata: Some(raven::MessageMetadata {
                user_id: req.user_id,
                device_id: req.device_id,
                language: req.language,
                confidence: stt_result.confidence,
                duration_ms: stt_result.duration_ms as i32,
                custom: std::collections::HashMap::new(),
            }),
            timestamp,
        };
        
        Ok(Response::new(TranscribeAudioResponse {
            message: Some(raven_message),
            success: true,
            error_message: String::new(),
        }))
    }
}

impl Default for HuginnMediaServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
