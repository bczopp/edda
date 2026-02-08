use async_trait::async_trait;
use crate::llm::provider::{LLMProvider, LLMError, PromptRequest, PromptResponse};
use super::client::BitNetClient;

/// LLM Provider implementation for BitNet.cpp
/// 
/// This provider integrates BitNet.cpp with the LLMProvider trait,
/// enabling ultra-efficient local LLM inference with 1-bit quantized models.
/// 
/// BitNet.cpp advantages:
/// - 90% reduction in memory usage vs full precision models
/// - 5-10x faster inference speed
/// - Ideal for resource-constrained devices (mobile, edge computing)
/// - Maintained quality through specialized 1-bit training
pub struct BitNetLLMProvider {
    client: BitNetClient,
}

impl BitNetLLMProvider {
    /// Create a new BitNet.cpp LLM provider
    /// 
    /// # Arguments
    /// 
    /// * `client` - The configured BitNet.cpp client
    pub fn new(client: BitNetClient) -> Self {
        Self { client }
    }
    
    /// Get the underlying client (for advanced usage)
    pub fn client(&self) -> &BitNetClient {
        &self.client
    }
    
    /// Check if extreme efficiency mode is enabled
    pub fn is_extreme_efficiency(&self) -> bool {
        self.client.is_extreme_efficiency()
    }
    
    /// Get estimated memory usage in MB
    /// 
    /// BitNet models use ~90% less memory than full precision models
    pub fn estimated_memory_mb(&self) -> u32 {
        self.client.estimated_memory_usage_mb()
    }
    
    /// Get the bit depth (always 1 for BitNet)
    pub fn bit_depth(&self) -> u8 {
        self.client.bit_depth()
    }
}

#[async_trait]
impl LLMProvider for BitNetLLMProvider {
    fn model_name(&self) -> &str {
        self.client.model_name()
    }
    
    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError> {
        // Build the full prompt with context if provided
        let full_prompt = if let Some(context) = &request.context {
            format!("Context: {}\n\nPrompt: {}", context, request.prompt)
        } else {
            request.prompt.clone()
        };
        
        // Use max_tokens from request or default to 512
        let max_tokens = request.max_tokens.unwrap_or(512);
        
        // Generate text using BitNet.cpp
        // BitNet will automatically use extreme efficiency optimizations if enabled
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
    use crate::llm::bitnet::BitNetConfig;
    
    #[tokio::test]
    async fn test_provider_creation() {
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client = BitNetClient::new(config).expect("client creation failed");
        let provider = BitNetLLMProvider::new(client);
        
        assert!(!provider.model_name().is_empty());
        assert!(provider.is_extreme_efficiency());
        assert_eq!(provider.bit_depth(), 1);
    }
    
    #[tokio::test]
    async fn test_process_prompt_simple() {
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client = BitNetClient::new(config).expect("client creation failed");
        let provider = BitNetLLMProvider::new(client);
        
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
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client = BitNetClient::new(config).expect("client creation failed");
        let provider = BitNetLLMProvider::new(client);
        
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
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client = BitNetClient::new(config).expect("client creation failed");
        let provider = BitNetLLMProvider::new(client);
        
        let request = PromptRequest {
            prompt: "Test".to_string(),
            context: None,
            max_tokens: None, // Should use default of 512
        };
        
        let result = provider.process_prompt(request).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_memory_efficiency() {
        let config = BitNetConfig {
            model_path: "/path/to/bitnet-3b.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client = BitNetClient::new(config).expect("client creation failed");
        let provider = BitNetLLMProvider::new(client);
        
        let memory = provider.estimated_memory_mb();
        
        // 3B model should be ~390MB
        assert_eq!(memory, 390);
        
        // Much more efficient than full precision models
        assert!(memory < 1000);
    }
}
