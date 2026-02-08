//! Google Gemini API Client

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub struct GoogleConfig {
    pub api_key: String,
    pub base_url: String,
}

impl GoogleConfig {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum GoogleError {
    HttpError(String),
    ParseError(String),
    ApiError { code: u16, message: String },
}

impl fmt::Display for GoogleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoogleError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            GoogleError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            GoogleError::ApiError { code, message } => {
                write!(f, "API error {}: {}", code, message)
            }
        }
    }
}

impl std::error::Error for GoogleError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<InlineData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineData {
    pub mime_type: String,
    pub data: String, // base64 encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenerateContentResponse {
    pub candidates: Vec<Candidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: u32,
    pub candidates_token_count: u32,
    pub total_token_count: u32,
}

pub struct GoogleClient {
    config: GoogleConfig,
    client: reqwest::Client,
}

impl GoogleClient {
    pub fn new(config: GoogleConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub fn config(&self) -> &GoogleConfig {
        &self.config
    }

    pub fn build_generate_content_request(
        &self,
        _model: &str,
        prompt: &str,
        context: Option<&str>,
        max_tokens: Option<u32>,
    ) -> GenerateContentRequest {
        let text = if let Some(ctx) = context {
            format!("Context: {}\n\nPrompt: {}", ctx, prompt)
        } else {
            prompt.to_string()
        };

        GenerateContentRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text,
                    inline_data: None,
                }],
                role: Some("user".to_string()),
            }],
            generation_config: Some(GenerationConfig {
                max_output_tokens: max_tokens,
                temperature: None,
            }),
        }
    }

    pub fn build_vision_request(
        &self,
        _model: &str,
        image_data: &[u8],
        prompt: Option<&str>,
    ) -> GenerateContentRequest {
        let base64_image = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            image_data,
        );

        let mut parts = vec![];
        
        if let Some(p) = prompt {
            parts.push(Part {
                text: p.to_string(),
                inline_data: None,
            });
        }

        parts.push(Part {
            text: String::new(),
            inline_data: Some(InlineData {
                mime_type: "image/jpeg".to_string(),
                data: base64_image,
            }),
        });

        GenerateContentRequest {
            contents: vec![Content {
                parts,
                role: Some("user".to_string()),
            }],
            generation_config: Some(GenerationConfig {
                max_output_tokens: Some(300),
                temperature: None,
            }),
        }
    }

    pub async fn generate_content(
        &self,
        request: GenerateContentRequest,
    ) -> Result<GenerateContentResponse, GoogleError> {
        // Note: Model is typically specified in the request, but we use a default
        let model = "gemini-2.5-flash"; // Could be parameterized
        let url = format!(
            "{}/models/{}:generateContent",
            self.config.base_url, model
        );

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.config.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| GoogleError::HttpError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GoogleError::ApiError {
                code: status.as_u16(),
                message: error_text,
            });
        }

        let response_body: GenerateContentResponse = response
            .json()
            .await
            .map_err(|e| GoogleError::ParseError(e.to_string()))?;

        Ok(response_body)
    }
}
