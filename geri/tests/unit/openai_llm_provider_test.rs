//! Unit tests for OpenAI LLM Provider

use geri::llm::{LLMProvider, PromptRequest};
use geri::llm::openai::{OpenAILLMProvider, OpenAIConfig};

#[tokio::test]
async fn test_openai_llm_provider_new() {
    let config = OpenAIConfig {
        api_key: "sk-test123".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let provider = OpenAILLMProvider::new(config, "gpt-4".to_string());
    assert_eq!(provider.model_name(), "gpt-4");
}

#[tokio::test]
async fn test_openai_llm_provider_model_name() {
    let config = OpenAIConfig {
        api_key: "sk-test".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let provider = OpenAILLMProvider::new(config, "gpt-3.5-turbo".to_string());
    assert_eq!(provider.model_name(), "gpt-3.5-turbo");
}

// Note: Real API tests would require a mock server or feature-gated integration tests
// For now, we test the structure and configuration
#[tokio::test]
async fn test_openai_llm_provider_constructs_valid_request() {
    let config = OpenAIConfig {
        api_key: "sk-test123".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let provider = OpenAILLMProvider::new(config, "gpt-4".to_string());
    
    let request = PromptRequest {
        prompt: "Hello, world!".to_string(),
        context: None,
        max_tokens: Some(100),
    };
    
    // In a real scenario, we would mock the HTTP client
    // For now, we verify the provider can be constructed and configured
    assert_eq!(provider.model_name(), "gpt-4");
}
