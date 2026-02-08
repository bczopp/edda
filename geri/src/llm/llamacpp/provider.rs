use async_trait::async_trait;
use crate::llm::provider::{LLMProvider, LLMError, PromptRequest, PromptResponse};
use super::client::LlamaCppClient;

/// LLM Provider implementation for llama.cpp
/// 
/// This provider integrates llama.cpp with the LLMProvider trait,
/// enabling local LLM inference with GGUF models.
pub struct LlamaCppLLMProvider {
    client: LlamaCppClient,
}

impl LlamaCppLLMProvider {
    /// Create a new llama.cpp LLM provider
    /// 
    /// # Arguments
    /// 
    /// * `client` - The configured llama.cpp client
    pub fn new(client: LlamaCppClient) -> Self {
        Self { client }
    }
    
    /// Get the underlying client (for advanced usage)
    pub fn client(&self) -> &LlamaCppClient {
        &self.client
    }
}

#[async_trait]
impl LLMProvider for LlamaCppLLMProvider {
    fn model_name(&self) -> &str {
        self.client.model_name()
    }
    
    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError> {
        // Use PromptFormatter for consistent prompting
        let formatter = crate::prompt::PromptFormatter::default();
        let full_prompt = formatter.format(
            request.system_prompt.as_deref().unwrap_or(""),
            &request.prompt,
            request.context.as_deref(),
        );
        
        // Use max_tokens from request or default to 512
        let max_tokens = request.max_tokens.unwrap_or(512);
        
        // Generate text using llama.cpp
        let generated_text = self.client
            .generate(&full_prompt, max_tokens)
            .await
            .map_err(|e| LLMError::ProcessingFailed(e.to_string()))?;
        
        // Estimate token usage (simplified - would use actual tokenizer in production)
        // Count input tokens
        let input_tokens = full_prompt.split_whitespace().count() as u32;
        // Count output tokens
        let output_tokens = generated_text.split_whitespace().count() as u32;
        let total_tokens = input_tokens + output_tokens;
        
        Ok(PromptResponse {
            text: generated_text,
            tokens_used: total_tokens,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::llamacpp::LlamaCppConfig;
    
    #[tokio::test]
    async fn test_provider_creation() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        
        let client = LlamaCppClient::new(config).expect("client creation failed");
        let provider = LlamaCppLLMProvider::new(client);
        
        assert!(!provider.model_name().is_empty());
    }
    
    #[tokio::test]
    async fn test_process_prompt_simple() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        
        let client = LlamaCppClient::new(config).expect("client creation failed");
        let provider = LlamaCppLLMProvider::new(client);
        
        let request = PromptRequest {
            prompt: "Hello".to_string(),
            context: None,
            max_tokens: Some(100),
        };
        
        let result = provider.process_prompt(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(!response.text.is_empty());
        assert!(response.tokens_used > 0);
    }
    
    #[tokio::test]
    async fn test_process_prompt_with_context() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        
        let client = LlamaCppClient::new(config).expect("client creation failed");
        let provider = LlamaCppLLMProvider::new(client);
        
        let request = PromptRequest {
            prompt: "What is this?".to_string(),
            context: Some("This is a test context.".to_string()),
            max_tokens: Some(50),
        };
        
        let result = provider.process_prompt(request).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_default_max_tokens() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        
        let client = LlamaCppClient::new(config).expect("client creation failed");
        let provider = LlamaCppLLMProvider::new(client);
        
        let request = PromptRequest {
            prompt: "Test".to_string(),
            context: None,
            max_tokens: None, // Should use default of 512
        };
        
        let result = provider.process_prompt(request).await;
        assert!(result.is_ok());
    }
}
