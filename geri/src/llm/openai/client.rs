//! OpenAI API Client for LLM and Vision requests

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Error)]
pub enum OpenAIError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    APIError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionRequest {
    pub model: String,
    pub messages: Vec<VisionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionMessage {
    pub role: String,
    pub content: Vec<VisionContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VisionContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
}

pub struct OpenAIClient {
    config: OpenAIConfig,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(config: OpenAIConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to build HTTP client");
        
        Self { config, client }
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    pub async fn validate(&self) -> Result<(), OpenAIError> {
        if self.config.api_key.is_empty() {
            return Err(OpenAIError::InvalidConfig("API key is empty".to_string()));
        }
        if !self.config.api_key.starts_with("sk-") {
            return Err(OpenAIError::InvalidConfig("API key must start with 'sk-'".to_string()));
        }
        Ok(())
    }

    pub fn build_chat_request(
        &self,
        model: &str,
        prompt: &str,
        system_prompt: Option<&str>,
        context: Option<&str>,
        max_tokens: Option<u32>,
    ) -> ChatRequest {
        let mut messages = Vec::new();

        // Use PromptFormatter indirectly to preserve separate system/user roles if possible
        // but here we just manage the system message content
        let mut system_content = system_prompt.unwrap_or("").to_string();
        if let Some(ctx) = context {
            if !system_content.is_empty() {
                system_content.push_str("\n\nContext:\n");
            } else {
                system_content.push_str("Context:\n");
            }
            system_content.push_str(ctx);
        }

        if !system_content.is_empty() {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: system_content,
            });
        }
        
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        });
        
        ChatRequest {
            model: model.to_string(),
            messages,
            max_tokens,
            temperature: None,
        }
    }

    pub fn build_vision_request(
        &self,
        model: &str,
        image_data: &[u8],
        prompt: Option<&str>,
    ) -> VisionRequest {
        // Encode image as base64 data URL
        let base64_image = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_data);
        let data_url = format!("data:image/jpeg;base64,{}", base64_image);
        
        let mut content = vec![
            VisionContent::ImageUrl {
                image_url: ImageUrl { url: data_url },
            },
        ];
        
        if let Some(p) = prompt {
            content.insert(0, VisionContent::Text {
                text: p.to_string(),
            });
        }
        
        VisionRequest {
            model: model.to_string(),
            messages: vec![VisionMessage {
                role: "user".to_string(),
                content,
            }],
            max_tokens: Some(300),
        }
    }

    pub async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse, OpenAIError> {
        let url = format!("{}/chat/completions", self.config.base_url);
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::APIError(format!("HTTP {}: {}", status, error_text)));
        }
        
        response
            .json()
            .await
            .map_err(|e| OpenAIError::RequestFailed(format!("Failed to parse response: {}", e)))
    }

    pub async fn vision_completion(&self, request: VisionRequest) -> Result<ChatResponse, OpenAIError> {
        let url = format!("{}/chat/completions", self.config.base_url);
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::APIError(format!("HTTP {}: {}", status, error_text)));
        }
        
        response
            .json()
            .await
            .map_err(|e| OpenAIError::RequestFailed(format!("Failed to parse response: {}", e)))
    }
}
