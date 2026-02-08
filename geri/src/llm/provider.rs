use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub context: Option<String>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResponse {
    pub text: String,
    pub tokens_used: u32,
}

#[derive(Debug, Error)]
pub enum LLMError {
    #[error("LLM processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
}

#[async_trait]
pub trait LLMProvider: Send + Sync {
    fn model_name(&self) -> &str;

    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError>;
}

pub struct LocalLLMProvider {
    model_name: String,
}

impl LocalLLMProvider {
    pub fn new(model_name: String) -> Self {
        Self { model_name }
    }
}

#[async_trait]
impl LLMProvider for LocalLLMProvider {
    fn model_name(&self) -> &str {
        &self.model_name
    }

    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError> {
        // Integrate llama.cpp or BitNet.cpp
        // In a real implementation, this would:
        // 1. Load model if not already loaded
        // 2. Tokenize prompt (and context if provided)
        // 3. Run inference through llama.cpp/BitNet.cpp
        // 4. Return generated text
        
        // Use PromptFormatter for consistent prompting
        let formatter = crate::prompt::PromptFormatter::default();
        let full_prompt = formatter.format(
            request.system_prompt.as_deref().unwrap_or(""),
            &request.prompt,
            request.context.as_deref(),
        );
        
        // Estimate token count (simplified - would use actual tokenizer)
        let estimated_tokens = full_prompt.split_whitespace().count() as u32;
        
        // In production, this would call llama.cpp or BitNet.cpp
        // For now, return a placeholder that indicates the integration point
        Ok(PromptResponse {
            text: format!("[LLM Response from {} for: {}]", self.model_name, request.prompt),
            tokens_used: estimated_tokens,
        })
    }
}
