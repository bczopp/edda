//! Image Input Processor for handling image input from frontend

use uuid::Uuid;
use tracing::{info, warn, error};

/// Image information after processing
#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub message_id: String,
    pub format: String,
    pub width: i32,
    pub height: i32,
    pub size_bytes: usize,
    pub user_id: String,
    pub device_id: String,
    pub data: Vec<u8>,
}

/// Image Input Processor handles image input from frontend and forwards to Odin
pub struct ImageInputProcessor {
    max_size_bytes: usize,
    allowed_formats: Vec<String>,
}

impl ImageInputProcessor {
    pub fn new() -> Self {
        info!("Creating ImageInputProcessor");
        Self {
            max_size_bytes: 10 * 1024 * 1024, // Default: 10MB
            allowed_formats: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "webp".to_string(),
            ],
        }
    }
    
    pub fn with_max_size(max_size_bytes: usize) -> Self {
        info!("Creating ImageInputProcessor with max size: {} bytes", max_size_bytes);
        Self {
            max_size_bytes,
            allowed_formats: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "webp".to_string(),
            ],
        }
    }
    
    /// Process image input and create ImageInfo
    pub async fn process(
        &self,
        image_data: &[u8],
        format: &str,
        width: i32,
        height: i32,
        user_id: &str,
        device_id: &str,
    ) -> Result<ImageInfo, Box<dyn std::error::Error>> {
        if image_data.is_empty() {
            return Err("Image data cannot be empty".into());
        }
        
        if !self.validate_format(format) {
            return Err(format!("Unsupported image format: {}", format).into());
        }
        
        if !self.validate_size(image_data.len()) {
            return Err(format!(
                "Image size {} bytes exceeds maximum allowed size {} bytes",
                image_data.len(),
                self.max_size_bytes
            ).into());
        }
        
        if width <= 0 || height <= 0 {
            return Err("Image dimensions must be positive".into());
        }
        
        info!(
            "Processing image input: {} bytes, {}x{} {} from user {} on device {}",
            image_data.len(),
            width,
            height,
            format,
            user_id,
            device_id
        );
        
        let message_id = Uuid::new_v4().to_string();
        
        let image_info = ImageInfo {
            message_id,
            format: format.to_lowercase(),
            width,
            height,
            size_bytes: image_data.len(),
            user_id: user_id.to_string(),
            device_id: device_id.to_string(),
            data: image_data.to_vec(),
        };
        
        // TODO: Forward to Odin
        // self.forward_to_odin(&image_info).await?;
        
        Ok(image_info)
    }
    
    /// Validate image format
    pub fn validate_format(&self, format: &str) -> bool {
        let format_lower = format.to_lowercase();
        self.allowed_formats.contains(&format_lower)
    }
    
    /// Validate image size
    pub fn validate_size(&self, size_bytes: usize) -> bool {
        size_bytes > 0 && size_bytes <= self.max_size_bytes
    }
    
    /// Forward ImageInfo to Odin (TODO: Implement Odin client)
    async fn forward_to_odin(
        &self,
        _image_info: &ImageInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement Odin client integration
        warn!("Forwarding to Odin not yet implemented");
        Ok(())
    }
}

impl Default for ImageInputProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_format() {
        let processor = ImageInputProcessor::new();
        assert!(processor.validate_format("jpg"));
        assert!(processor.validate_format("PNG"));
        assert!(!processor.validate_format("gif"));
    }
    
    #[test]
    fn test_validate_size() {
        let processor = ImageInputProcessor::new();
        assert!(processor.validate_size(1024));
        assert!(processor.validate_size(10 * 1024 * 1024));
        assert!(!processor.validate_size(11 * 1024 * 1024));
    }
    
    #[tokio::test]
    async fn test_process_empty_data() {
        let processor = ImageInputProcessor::new();
        let result = processor.process(&[], "png", 100, 100, "user1", "device1").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_process_invalid_format() {
        let processor = ImageInputProcessor::new();
        let data = vec![0u8; 100];
        let result = processor.process(&data, "gif", 100, 100, "user1", "device1").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_process_invalid_dimensions() {
        let processor = ImageInputProcessor::new();
        let data = vec![0u8; 100];
        let result = processor.process(&data, "png", 0, 100, "user1", "device1").await;
        assert!(result.is_err());
    }
}
