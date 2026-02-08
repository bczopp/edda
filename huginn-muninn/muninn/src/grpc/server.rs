//! Muninn gRPC Service Implementation

use tonic::{Request, Response, Status};
use tracing::{info, warn};

// Include generated proto code
pub mod muninn {
    tonic::include_proto!("muninn");
}

pub mod raven {
    tonic::include_proto!("raven");
}

use muninn::muninn_tts_service_server::MuninnTtsService;
use muninn::*;
use crate::tts::{TtsEngine, TtsConfig, TtsVoice as TtsVoiceEnum};
use crate::cache::TTSCacheManager;
use std::sync::Arc;
use std::time::Duration;

pub struct MuninnTtsServiceImpl {
    tts_engine: Arc<TtsEngine>,
    cache: Arc<TTSCacheManager>,
}

impl MuninnTtsServiceImpl {
    pub fn new() -> Self {
        info!("Creating MuninnTtsServiceImpl");
        Self {
            tts_engine: Arc::new(TtsEngine::new(TtsConfig::default())),
            cache: Arc::new(TTSCacheManager::new(100, Duration::from_secs(3600))),
        }
    }
    
    pub fn with_tts_engine(tts_engine: Arc<TtsEngine>) -> Self {
        info!("Creating MuninnTtsServiceImpl with custom TTS engine");
        Self {
            tts_engine,
            cache: Arc::new(TTSCacheManager::new(100, Duration::from_secs(3600))),
        }
    }
    
    pub fn is_ready(&self) -> bool {
        true
    }
}

#[tonic::async_trait]
impl MuninnTtsService for MuninnTtsServiceImpl {
    async fn generate_speech(
        &self,
        request: Request<TtsRequest>,
    ) -> Result<Response<TtsResponse>, Status> {
        let req = request.into_inner();
        
        if req.text.is_empty() {
            return Err(Status::invalid_argument("Text cannot be empty"));
        }
        
        info!(
            "Generating speech: {} chars, language {}, voice {:?} for user {}",
            req.text.len(),
            req.language,
            TtsVoice::try_from(req.voice).unwrap_or(TtsVoice::Female),
            req.user_id
        );
        
        let settings = req.settings.unwrap_or_else(|| TtsSettings {
            speed: 1.0,
            pitch: 0.0,
            volume: 1.0,
            audio_format: "wav".to_string(),
            sample_rate: 16000,
        });
        
        // Use TTS engine to generate speech
        let tts_voice = match TtsVoice::try_from(req.voice) {
            Ok(TtsVoice::Male) => TtsVoiceEnum::Male,
            Ok(TtsVoice::Female) => TtsVoiceEnum::Female,
            Ok(TtsVoice::Neutral) => TtsVoiceEnum::Neutral,
            _ => TtsVoiceEnum::Female, // Default
        };
        
        let mut tts_config = TtsConfig {
            language: req.language.clone(),
            voice: tts_voice,
            model_path: None,
        };
        
        // Check cache first
        let cache_key = self.cache.generate_key(&req.text, &req.language, &format!("{:?}", tts_voice));
        let audio_buffer = if let Some(cached) = self.cache.get(&cache_key).await {
            info!("Cache hit for text: {} chars", req.text.len());
            cached
        } else {
            info!("Cache miss for text: {} chars, synthesizing...", req.text.len());
            let tts_engine = TtsEngine::new(tts_config);
            let audio_buffer = tts_engine.synthesize(&req.text).await
                .map_err(|e| Status::internal(format!("TTS synthesis failed: {}", e)))?;
            
            // Cache the result
            self.cache.set(&cache_key, audio_buffer.clone()).await;
            audio_buffer
        };
        
        // Convert AudioBuffer to bytes
        let audio_data: Vec<u8> = audio_buffer.samples.iter()
            .flat_map(|&sample| sample.to_le_bytes().to_vec())
            .collect();
        
        let duration_ms = audio_buffer.duration_ms as i32;
        
        Ok(Response::new(TtsResponse {
            audio_data,
            audio_format: settings.audio_format,
            sample_rate: settings.sample_rate,
            channels: channels as i32,
            duration_ms,
            success: true,
            error_message: String::new(),
        }))
    }
    
    async fn generate_speech_stream(
        &self,
        request: Request<TtsRequest>,
    ) -> Result<Response<Self::GenerateSpeechStreamStream>, Status> {
        let req = request.into_inner();
        
        if req.text.is_empty() {
            return Err(Status::invalid_argument("Text cannot be empty"));
        }
        
        info!(
            "Generating speech stream: {} chars for user {}",
            req.text.len(),
            req.user_id
        );
        
        // TODO: Implement streaming TTS
        // For now, return error as not implemented
        Err(Status::unimplemented("Streaming TTS not yet implemented"))
    }
    
    type GenerateSpeechStreamStream = std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<TtsStreamChunk, Status>> + Send + 'static>
    >;
}

impl Default for MuninnTtsServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
