//! Unit tests for Anthropic LLM Provider

use geri::llm::{LLMProvider, PromptRequest};
use geri::llm::anthropic::{AnthropicLLMProvider, AnthropicConfig};

#[tokio::test]
async fn test_anthropic_llm_provider_new() {
    let config = AnthropicConfig {
        api_key: "sk-ant-test123".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let provider = AnthropicLLMProvider::new(config, "claude-3-opus-20240229".to_string());
    assert_eq!(provider.model_name(), "claude-3-opus-20240229");
}

#[tokio::test]
async fn test_anthropic_llm_provider_model_name() {
    let config = AnthropicConfig {
        api_key: "sk-ant-test".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let provider = AnthropicLLMProvider::new(config, "claude-3-sonnet-20240229".to_string());
    assert_eq!(provider.model_name(), "claude-3-sonnet-20240229");
}

// Note: Real API tests would require a mock server or feature-gated integration tests
// For now, we test the structure and configuration
#[tokio::test]
async fn test_anthropic_llm_provider_constructs_valid_request() {
    let config = AnthropicConfig {
        api_key: "sk-ant-test123".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let provider = AnthropicLLMProvider::new(config, "claude-3-haiku-20240307".to_string());
    
    let request = PromptRequest {
        prompt: "Hello, Claude!".to_string(),
        context: None,
        max_tokens: Some(100),
    };
    
    // In a real scenario, we would mock the HTTP client
    // For now, we verify the provider can be constructed and configured
    assert_eq!(provider.model_name(), "claude-3-haiku-20240307");
}
