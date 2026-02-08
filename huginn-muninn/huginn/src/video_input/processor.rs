//! Video Input Processor for handling video input from frontend

use uuid::Uuid;
use tracing::{info, warn, error};

/// Video information after processing
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub message_id: String,
    pub format: String,
    pub duration_ms: i32,
    pub size_bytes: usize,
    pub user_id: String,
    pub device_id: String,
    pub data: Vec<u8>,
}

/// Video Input Processor handles video input from frontend and forwards to Odin
pub struct VideoInputProcessor {
    max_size_bytes: usize,
    max_duration_ms: i32,
    allowed_formats: Vec<String>,
}

impl VideoInputProcessor {
    pub fn new() -> Self {
        info!("Creating VideoInputProcessor");
        Self {
            max_size_bytes: 100 * 1024 * 1024, // Default: 100MB
            max_duration_ms: 3600000, // Default: 1 hour
            allowed_formats: vec![
                "mp4".to_string(),
                "webm".to_string(),
                "avi".to_string(),
                "mkv".to_string(),
            ],
        }
    }
    
    pub fn with_limits(max_size_bytes: usize, max_duration_ms: i32) -> Self {
        info!(
            "Creating VideoInputProcessor with max size: {} bytes, max duration: {} ms",
            max_size_bytes,
            max_duration_ms
        );
        Self {
            max_size_bytes,
            max_duration_ms,
            allowed_formats: vec![
                "mp4".to_string(),
                "webm".to_string(),
                "avi".to_string(),
                "mkv".to_string(),
            ],
        }
    }
    
    /// Process video input and create VideoInfo
    pub async fn process(
        &self,
        video_data: &[u8],
        format: &str,
        duration_ms: i32,
        user_id: &str,
        device_id: &str,
    ) -> Result<VideoInfo, Box<dyn std::error::Error>> {
        if video_data.is_empty() {
            return Err("Video data cannot be empty".into());
        }
        
        if !self.validate_format(format) {
            return Err(format!("Unsupported video format: {}", format).into());
        }
        
        if !self.validate_size(video_data.len()) {
            return Err(format!(
                "Video size {} bytes exceeds maximum allowed size {} bytes",
                video_data.len(),
                self.max_size_bytes
            ).into());
        }
        
        if !self.validate_duration(duration_ms) {
            return Err(format!(
                "Video duration {} ms exceeds maximum allowed duration {} ms",
                duration_ms,
                self.max_duration_ms
            ).into());
        }
        
        info!(
            "Processing video input: {} bytes, {} ms {} from user {} on device {}",
            video_data.len(),
            duration_ms,
            format,
            user_id,
            device_id
        );
        
        let message_id = Uuid::new_v4().to_string();
        
        let video_info = VideoInfo {
            message_id,
            format: format.to_lowercase(),
            duration_ms,
            size_bytes: video_data.len(),
            user_id: user_id.to_string(),
            device_id: device_id.to_string(),
            data: video_data.to_vec(),
        };
        
        // TODO: Forward to Odin
        // self.forward_to_odin(&video_info).await?;
        
        Ok(video_info)
    }
    
    /// Validate video format
    pub fn validate_format(&self, format: &str) -> bool {
        let format_lower = format.to_lowercase();
        self.allowed_formats.contains(&format_lower)
    }
    
    /// Validate video size
    pub fn validate_size(&self, size_bytes: usize) -> bool {
        size_bytes > 0 && size_bytes <= self.max_size_bytes
    }
    
    /// Validate video duration
    pub fn validate_duration(&self, duration_ms: i32) -> bool {
        duration_ms > 0 && duration_ms <= self.max_duration_ms
    }
    
    /// Forward VideoInfo to Odin (TODO: Implement Odin client)
    async fn forward_to_odin(
        &self,
        _video_info: &VideoInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement Odin client integration
        warn!("Forwarding to Odin not yet implemented");
        Ok(())
    }
}

impl Default for VideoInputProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_format() {
        let processor = VideoInputProcessor::new();
        assert!(processor.validate_format("mp4"));
        assert!(processor.validate_format("WEBM"));
        assert!(!processor.validate_format("mov"));
    }
    
    #[test]
    fn test_validate_size() {
        let processor = VideoInputProcessor::new();
        assert!(processor.validate_size(1024));
        assert!(processor.validate_size(100 * 1024 * 1024));
        assert!(!processor.validate_size(101 * 1024 * 1024));
    }
    
    #[test]
    fn test_validate_duration() {
        let processor = VideoInputProcessor::new();
        assert!(processor.validate_duration(1000));
        assert!(processor.validate_duration(3600000));
        assert!(!processor.validate_duration(0));
        assert!(!processor.validate_duration(-1000));
    }
    
    #[tokio::test]
    async fn test_process_empty_data() {
        let processor = VideoInputProcessor::new();
        let result = processor.process(&[], "mp4", 1000, "user1", "device1").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_process_invalid_format() {
        let processor = VideoInputProcessor::new();
        let data = vec![0u8; 100];
        let result = processor.process(&data, "mov", 1000, "user1", "device1").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_process_invalid_duration() {
        let processor = VideoInputProcessor::new();
        let data = vec![0u8; 100];
        let result = processor.process(&data, "mp4", 0, "user1", "device1").await;
        assert!(result.is_err());
    }
}
