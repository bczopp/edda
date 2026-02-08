//! Anthropic API Client for Claude LLM and Vision requests

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_secs: u64,
    pub anthropic_version: String,
}

#[derive(Debug, Error)]
pub enum AnthropicError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    APIError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String, // "text"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub id: String,
    pub content: Vec<ResponseContent>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

// Vision-specific content blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VisionContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: ImageSource },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String, // "base64"
    pub media_type: String,  // "image/jpeg", "image/png", etc.
    pub data: String,        // base64-encoded image
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionMessage {
    pub role: String,
    pub content: Vec<VisionContentBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionRequest {
    pub model: String,
    pub messages: Vec<VisionMessage>,
    pub max_tokens: u32,
}

pub struct AnthropicClient {
    config: AnthropicConfig,
    client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(config: AnthropicConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to build HTTP client");
        
        Self { config, client }
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    pub async fn validate(&self) -> Result<(), AnthropicError> {
        if self.config.api_key.is_empty() {
            return Err(AnthropicError::InvalidConfig("API key is empty".to_string()));
        }
        if !self.config.api_key.starts_with("sk-ant-") {
            return Err(AnthropicError::InvalidConfig("API key must start with 'sk-ant-'".to_string()));
        }
        Ok(())
    }

    pub fn build_messages_request(
        &self,
        model: &str,
        prompt: &str,
        system_prompt: Option<&str>,
        context: Option<&str>,
        max_tokens: Option<u32>,
    ) -> MessagesRequest {
        let messages = vec![Message {
            role: "user".to_string(),
            content: vec![ContentBlock {
                content_type: "text".to_string(),
                text: prompt.to_string(),
            }],
        }];
        
        let mut system_content = system_prompt.unwrap_or("").to_string();
        if let Some(ctx) = context {
            if !system_content.is_empty() {
                system_content.push_str("\n\nContext:\n");
            } else {
                system_content.push_str("Context:\n");
            }
            system_content.push_str(ctx);
        }

        let system = if system_content.is_empty() {
            None
        } else {
            Some(system_content)
        };

        MessagesRequest {
            model: model.to_string(),
            messages,
            max_tokens: max_tokens.unwrap_or(1024),
            system,
            temperature: None,
        }
    }

    pub fn build_vision_request(
        &self,
        model: &str,
        image_data: &[u8],
        prompt: Option<&str>,
    ) -> VisionRequest {
        // Encode image as base64
        let base64_image = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_data);
        
        let mut content = Vec::new();
        
        if let Some(p) = prompt {
            content.push(VisionContentBlock::Text {
                text: p.to_string(),
            });
        }
        
        content.push(VisionContentBlock::Image {
            source: ImageSource {
                source_type: "base64".to_string(),
                media_type: "image/jpeg".to_string(),
                data: base64_image,
            },
        });
        
        VisionRequest {
            model: model.to_string(),
            messages: vec![VisionMessage {
                role: "user".to_string(),
                content,
            }],
            max_tokens: 1024,
        }
    }

    pub async fn messages(&self, request: MessagesRequest) -> Result<MessagesResponse, AnthropicError> {
        let url = format!("{}/messages", self.config.base_url);
        
        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", &self.config.anthropic_version)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AnthropicError::RequestFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AnthropicError::APIError(format!("HTTP {}: {}", status, error_text)));
        }
        
        response
            .json()
            .await
            .map_err(|e| AnthropicError::RequestFailed(format!("Failed to parse response: {}", e)))
    }

    pub async fn vision_messages(&self, request: VisionRequest) -> Result<MessagesResponse, AnthropicError> {
        let url = format!("{}/messages", self.config.base_url);
        
        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", &self.config.anthropic_version)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AnthropicError::RequestFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AnthropicError::APIError(format!("HTTP {}: {}", status, error_text)));
        }
        
        response
            .json()
            .await
            .map_err(|e| AnthropicError::RequestFailed(format!("Failed to parse response: {}", e)))
    }
}
