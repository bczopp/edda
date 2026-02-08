//! Text Input Processor for handling text input from frontend

use crate::grpc::server::raven::{RavenMessage, MessageDirection, MessageMetadata};
use chrono::Utc;
use uuid::Uuid;
use tracing::{info, warn, error};

/// Text Input Processor handles text input from frontend and forwards to Odin
pub struct TextInputProcessor {
    // Future: Add Odin client
}

impl TextInputProcessor {
    pub fn new() -> Self {
        info!("Creating TextInputProcessor");
        Self {}
    }
    
    /// Process text input and create RavenMessage
    pub async fn process(
        &self,
        text: &str,
        user_id: &str,
        device_id: &str,
    ) -> Result<RavenMessage, Box<dyn std::error::Error>> {
        if text.is_empty() {
            return Err("Text cannot be empty".into());
        }
        
        info!("Processing text input: {} chars from user {} on device {}", 
            text.len(), user_id, device_id);
        
        let raven_message = self.create_raven_message(text, user_id, device_id)?;
        
        // TODO: Forward to Odin
        // self.forward_to_odin(&raven_message).await?;
        
        Ok(raven_message)
    }
    
    /// Create RavenMessage from text input
    pub fn create_raven_message(
        &self,
        text: &str,
        user_id: &str,
        device_id: &str,
    ) -> Result<RavenMessage, Box<dyn std::error::Error>> {
        let message_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();
        
        let metadata = MessageMetadata {
            user_id: user_id.to_string(),
            device_id: device_id.to_string(),
            language: String::new(), // TODO: Detect language or get from request
            confidence: 1.0, // Text input is always 100% confidence
            duration_ms: 0, // Not applicable for text
            custom: std::collections::HashMap::new(),
        };
        
        Ok(RavenMessage {
            message_id,
            direction: MessageDirection::Incoming as i32,
            content: text.to_string(),
            metadata: Some(metadata),
            timestamp,
        })
    }
    
    /// Forward RavenMessage to Odin (TODO: Implement Odin client)
    async fn forward_to_odin(
        &self,
        _message: &RavenMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement Odin client integration
        warn!("Forwarding to Odin not yet implemented");
        Ok(())
    }
}

impl Default for TextInputProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_raven_message() {
        let processor = TextInputProcessor::new();
        let message = processor.create_raven_message("Hello, World!", "user123", "device456").unwrap();
        
        assert_eq!(message.content, "Hello, World!");
        assert_eq!(message.direction, MessageDirection::Incoming as i32);
        assert!(message.metadata.is_some());
        let metadata = message.metadata.unwrap();
        assert_eq!(metadata.user_id, "user123");
        assert_eq!(metadata.device_id, "device456");
        assert_eq!(metadata.confidence, 1.0);
    }
    
    #[tokio::test]
    async fn test_process_empty_text() {
        let processor = TextInputProcessor::new();
        let result = processor.process("", "user123", "device456").await;
        assert!(result.is_err());
    }
}
