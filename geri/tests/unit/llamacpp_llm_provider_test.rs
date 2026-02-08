use geri::llm::llamacpp::{LlamaCppClient, LlamaCppConfig};
use geri::llm::provider::{LLMProvider, PromptRequest};

#[tokio::test]
async fn test_llamacpp_provider_creation() {
    let config = LlamaCppConfig {
        model_path: "/path/to/llama-3-8b.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    let client = LlamaCppClient::new(config).expect("client creation failed");
    let provider = geri::llm::llamacpp::LlamaCppLLMProvider::new(client);
    
    assert_eq!(provider.model_name(), "llama-3-8b");
}

#[tokio::test]
async fn test_llamacpp_provider_process_prompt() {
    let config = LlamaCppConfig {
        model_path: "/path/to/llama-3-8b.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    let client = LlamaCppClient::new(config).expect("client creation failed");
    let provider = geri::llm::llamacpp::LlamaCppLLMProvider::new(client);
    
    let request = PromptRequest {
        prompt: "Hello, world!".to_string(),
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
async fn test_llamacpp_provider_with_context() {
    let config = LlamaCppConfig {
        model_path: "/path/to/llama-3-8b.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    let client = LlamaCppClient::new(config).expect("client creation failed");
    let provider = geri::llm::llamacpp::LlamaCppLLMProvider::new(client);
    
    let request = PromptRequest {
        prompt: "What is the capital?".to_string(),
        context: Some("France is a country in Europe.".to_string()),
        max_tokens: Some(50),
    };
    
    let result = provider.process_prompt(request).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(!response.text.is_empty());
}

#[tokio::test]
async fn test_llamacpp_provider_max_tokens() {
    let config = LlamaCppConfig {
        model_path: "/path/to/llama-3-8b.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    let client = LlamaCppClient::new(config).expect("client creation failed");
    let provider = geri::llm::llamacpp::LlamaCppLLMProvider::new(client);
    
    // Test with max_tokens specified
    let request_with_limit = PromptRequest {
        prompt: "Test prompt".to_string(),
        context: None,
        max_tokens: Some(10),
    };
    
    let result_with_limit = provider.process_prompt(request_with_limit).await;
    assert!(result_with_limit.is_ok());
    
    // Test without max_tokens (should use default)
    let request_no_limit = PromptRequest {
        prompt: "Test prompt".to_string(),
        context: None,
        max_tokens: None,
    };
    
    let result_no_limit = provider.process_prompt(request_no_limit).await;
    assert!(result_no_limit.is_ok());
}

#[tokio::test]
async fn test_llamacpp_provider_trait_implementation() {
    let config = LlamaCppConfig {
        model_path: "/path/to/model.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    let client = LlamaCppClient::new(config).expect("client creation failed");
    let provider = geri::llm::llamacpp::LlamaCppLLMProvider::new(client);
    
    // Test that it implements LLMProvider trait
    let _provider_ref: &dyn LLMProvider = &provider;
    
    // Test model_name from trait
    assert!(!provider.model_name().is_empty());
}
