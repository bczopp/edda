//! Unit tests for Google LLM Provider

use geri::llm::google::{GoogleLLMProvider, GoogleConfig};
use geri::llm::{LLMProvider, PromptRequest};

#[test]
fn test_google_llm_provider_new() {
    let config = GoogleConfig::new("test-key".to_string());
    let provider = GoogleLLMProvider::new(config, "gemini-2.5-flash".to_string());
    
    assert_eq!(provider.model_name(), "gemini-2.5-flash");
}

#[test]
fn test_google_llm_provider_model_name() {
    let config = GoogleConfig::new("test-key".to_string());
    let provider = GoogleLLMProvider::new(config, "gemini-pro".to_string());
    
    assert_eq!(provider.model_name(), "gemini-pro");
}

#[tokio::test]
#[ignore] // Requires valid API key and network
async fn test_google_llm_provider_process_prompt() {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        println!("Skipping integration test - no GEMINI_API_KEY");
        return;
    }
    
    let config = GoogleConfig::new(api_key);
    let provider = GoogleLLMProvider::new(config, "gemini-2.5-flash".to_string());
    
    let request = PromptRequest {
        prompt: "Say hello in one word".to_string(),
        context: None,
        max_tokens: Some(10),
    };
    
    let response = provider.process_prompt(request).await;
    assert!(response.is_ok());
    
    let result = response.unwrap();
    assert!(!result.text.is_empty());
    assert!(result.tokens_used > 0);
}

#[tokio::test]
#[ignore] // Requires valid API key and network
async fn test_google_llm_provider_with_context() {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        println!("Skipping integration test - no GEMINI_API_KEY");
        return;
    }
    
    let config = GoogleConfig::new(api_key);
    let provider = GoogleLLMProvider::new(config, "gemini-2.5-flash".to_string());
    
    let request = PromptRequest {
        prompt: "What is the capital?".to_string(),
        context: Some("The country is France.".to_string()),
        max_tokens: Some(20),
    };
    
    let response = provider.process_prompt(request).await;
    assert!(response.is_ok());
    
    let result = response.unwrap();
    assert!(!result.text.is_empty());
    // Response should mention Paris
    assert!(result.text.to_lowercase().contains("paris"));
}
