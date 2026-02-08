//! Video Stream Processor for handling video streams from frontend

use uuid::Uuid;
use tracing::{info, warn, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Stream chunk information
#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub chunk_index: i32,
    pub data: Vec<u8>,
    pub is_last: bool,
    pub format: String,
}

/// Stream statistics
#[derive(Debug, Clone)]
pub struct StreamStats {
    pub stream_id: String,
    pub chunk_count: usize,
    pub total_bytes: usize,
    pub format: String,
    pub user_id: String,
    pub device_id: String,
}

/// Stream state
#[derive(Debug, Clone, PartialEq)]
enum StreamState {
    Active,
    Interrupted,
    Completed,
}

/// Stream session information
#[derive(Debug, Clone)]
struct StreamSession {
    stream_id: String,
    state: StreamState,
    stats: StreamStats,
    last_chunk_index: i32,
}

/// Video Stream Processor handles video streams from frontend and forwards to Odin
pub struct VideoStreamProcessor {
    max_chunk_size_bytes: usize,
    allowed_formats: Vec<String>,
    sessions: Arc<RwLock<HashMap<String, StreamSession>>>, // Key: "{user_id}:{device_id}"
}

impl VideoStreamProcessor {
    pub fn new() -> Self {
        info!("Creating VideoStreamProcessor");
        Self {
            max_chunk_size_bytes: 10 * 1024 * 1024, // Default: 10MB per chunk
            allowed_formats: vec![
                "mp4".to_string(),
                "webm".to_string(),
                "rtsp".to_string(),
                "webrtc".to_string(),
            ],
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn with_max_chunk_size(max_chunk_size_bytes: usize) -> Self {
        info!("Creating VideoStreamProcessor with max chunk size: {} bytes", max_chunk_size_bytes);
        Self {
            max_chunk_size_bytes,
            allowed_formats: vec![
                "mp4".to_string(),
                "webm".to_string(),
                "rtsp".to_string(),
                "webrtc".to_string(),
            ],
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Process a stream chunk
    pub async fn process_chunk(
        &self,
        chunk: StreamChunk,
        user_id: &str,
        device_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if chunk.data.is_empty() {
            return Err("Chunk data cannot be empty".into());
        }
        
        if !self.validate_format(&chunk.format) {
            return Err(format!("Unsupported stream format: {}", chunk.format).into());
        }
        
        if chunk.data.len() > self.max_chunk_size_bytes {
            return Err(format!(
                "Chunk size {} bytes exceeds maximum allowed size {} bytes",
                chunk.data.len(),
                self.max_chunk_size_bytes
            ).into());
        }
        
        let session_key = format!("{}:{}", user_id, device_id);
        let mut sessions = self.sessions.write().await;
        
        // Get or create session
        let session = sessions.entry(session_key.clone()).or_insert_with(|| {
            let stream_id = Uuid::new_v4().to_string();
            StreamSession {
                stream_id: stream_id.clone(),
                state: StreamState::Active,
                stats: StreamStats {
                    stream_id,
                    chunk_count: 0,
                    total_bytes: 0,
                    format: chunk.format.clone(),
                    user_id: user_id.to_string(),
                    device_id: device_id.to_string(),
                },
                last_chunk_index: -1,
            }
        });
        
        // Check for stream interruption (missing chunks)
        if chunk.chunk_index > session.last_chunk_index + 1 {
            warn!(
                "Stream interruption detected: expected chunk {}, got chunk {}",
                session.last_chunk_index + 1,
                chunk.chunk_index
            );
            session.state = StreamState::Interrupted;
        }
        
        // Update session
        session.last_chunk_index = chunk.chunk_index;
        session.stats.chunk_count += 1;
        session.stats.total_bytes += chunk.data.len();
        
        if chunk.is_last {
            session.state = StreamState::Completed;
            info!("Stream completed: {} chunks, {} bytes", session.stats.chunk_count, session.stats.total_bytes);
        }
        
        info!(
            "Processing stream chunk {}: {} bytes, format {} from user {} on device {}",
            chunk.chunk_index,
            chunk.data.len(),
            chunk.format,
            user_id,
            device_id
        );
        
        // TODO: Forward chunk to Odin
        // self.forward_chunk_to_odin(&chunk, &session.stats).await?;
        
        Ok(())
    }
    
    /// Handle stream interruption
    pub async fn handle_interruption(
        &self,
        user_id: &str,
        device_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session_key = format!("{}:{}", user_id, device_id);
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(&session_key) {
            session.state = StreamState::Interrupted;
            warn!("Stream interruption handled for user {} on device {}", user_id, device_id);
        }
        
        Ok(())
    }
    
    /// Get stream statistics
    pub async fn get_stream_stats(
        &self,
        user_id: &str,
        device_id: &str,
    ) -> Option<StreamStats> {
        let session_key = format!("{}:{}", user_id, device_id);
        let sessions = self.sessions.read().await;
        
        sessions.get(&session_key).map(|session| session.stats.clone())
    }
    
    /// Validate stream format
    pub fn validate_format(&self, format: &str) -> bool {
        let format_lower = format.to_lowercase();
        self.allowed_formats.contains(&format_lower)
    }
    
    /// Forward chunk to Odin (TODO: Implement Odin client)
    async fn forward_chunk_to_odin(
        &self,
        _chunk: &StreamChunk,
        _stats: &StreamStats,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement Odin client integration
        warn!("Forwarding chunk to Odin not yet implemented");
        Ok(())
    }
    
    /// Clean up completed sessions
    pub async fn cleanup_completed_sessions(&self) {
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, session| session.state != StreamState::Completed);
    }
}

impl Default for VideoStreamProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_format() {
        let processor = VideoStreamProcessor::new();
        assert!(processor.validate_format("mp4"));
        assert!(processor.validate_format("RTSP"));
        assert!(!processor.validate_format("mov"));
    }
    
    #[tokio::test]
    async fn test_process_chunk_empty_data() {
        let processor = VideoStreamProcessor::new();
        let chunk = StreamChunk {
            chunk_index: 0,
            data: vec![],
            is_last: false,
            format: "mp4".to_string(),
        };
        let result = processor.process_chunk(chunk, "user1", "device1").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_process_chunk_invalid_format() {
        let processor = VideoStreamProcessor::new();
        let chunk = StreamChunk {
            chunk_index: 0,
            data: vec![0u8; 100],
            is_last: false,
            format: "mov".to_string(),
        };
        let result = processor.process_chunk(chunk, "user1", "device1").await;
        assert!(result.is_err());
    }
}
