use geri::llm::bitnet::{BitNetClient, BitNetConfig};
use geri::llm::provider::{LLMProvider, PromptRequest};

#[tokio::test]
async fn test_bitnet_provider_creation() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    let provider = geri::llm::bitnet::BitNetLLMProvider::new(client);
    
    assert_eq!(provider.model_name(), "bitnet-3b");
}

#[tokio::test]
async fn test_bitnet_provider_process_prompt() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    let provider = geri::llm::bitnet::BitNetLLMProvider::new(client);
    
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
async fn test_bitnet_provider_extreme_efficiency() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    let provider = geri::llm::bitnet::BitNetLLMProvider::new(client);
    
    assert!(provider.is_extreme_efficiency());
    
    let request = PromptRequest {
        prompt: "Test".to_string(),
        context: None,
        max_tokens: Some(50),
    };
    
    let start = std::time::Instant::now();
    let result = provider.process_prompt(request).await;
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    // With extreme efficiency, processing should be faster (stub simulates this)
    assert!(duration.as_millis() < 100);
}

#[tokio::test]
async fn test_bitnet_provider_with_context() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    let provider = geri::llm::bitnet::BitNetLLMProvider::new(client);
    
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
async fn test_bitnet_provider_memory_efficiency() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    let provider = geri::llm::bitnet::BitNetLLMProvider::new(client);
    
    // Verify memory efficiency metadata
    let memory_usage = provider.estimated_memory_mb();
    
    // 3B BitNet model should use < 500MB
    assert!(memory_usage < 500);
}

#[tokio::test]
async fn test_bitnet_provider_trait_implementation() {
    let config = BitNetConfig {
        model_path: "/path/to/model.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: false,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    let provider = geri::llm::bitnet::BitNetLLMProvider::new(client);
    
    // Test that it implements LLMProvider trait
    let _provider_ref: &dyn LLMProvider = &provider;
    
    // Test model_name from trait
    assert!(!provider.model_name().is_empty());
}

#[tokio::test]
async fn test_bitnet_vs_llamacpp_comparison() {
    // This test demonstrates the trade-off between BitNet and llama.cpp
    // BitNet: Lower memory, faster inference, optimized for resource-constrained devices
    // llama.cpp: Higher precision, potentially better quality
    
    let bitnet_config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let bitnet_client = BitNetClient::new(bitnet_config).expect("bitnet client creation failed");
    let bitnet_provider = geri::llm::bitnet::BitNetLLMProvider::new(bitnet_client);
    
    // BitNet should have significantly lower memory footprint
    let bitnet_memory = bitnet_provider.estimated_memory_mb();
    
    // For comparison: A 3B llama.cpp GGUF Q4 model would be ~2GB
    // BitNet 3B 1-bit model is ~390MB
    assert!(bitnet_memory < 500); // ~90% reduction
}
