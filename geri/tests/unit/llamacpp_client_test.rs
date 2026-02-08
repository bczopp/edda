use geri::llm::llamacpp::{LlamaCppClient, LlamaCppConfig, LlamaCppError};

#[tokio::test]
async fn test_llamacpp_client_creation() {
    let config = LlamaCppConfig {
        model_path: "/path/to/model.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    // Client creation should not fail
    let result = LlamaCppClient::new(config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_llamacpp_config_validation() {
    // Valid config
    let valid_config = LlamaCppConfig {
        model_path: "/valid/path.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    assert!(valid_config.validate().is_ok());
    
    // Empty model path should fail
    let invalid_config = LlamaCppConfig {
        model_path: "".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    assert!(invalid_config.validate().is_err());
    
    // Invalid context size (0) should fail
    let invalid_ctx = LlamaCppConfig {
        model_path: "/path/to/model.gguf".to_string(),
        n_ctx: 0,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    assert!(invalid_ctx.validate().is_err());
    
    // Invalid thread count (0) should fail
    let invalid_threads = LlamaCppConfig {
        model_path: "/path/to/model.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 0,
        n_gpu_layers: 0,
    };
    assert!(invalid_threads.validate().is_err());
}

#[tokio::test]
async fn test_llamacpp_generate_stub() {
    let config = LlamaCppConfig {
        model_path: "/path/to/model.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    let client = LlamaCppClient::new(config).expect("client creation failed");
    
    // Test generation (stub implementation for now)
    let prompt = "Hello, world!";
    let max_tokens = 100;
    
    let result = client.generate(prompt, max_tokens).await;
    
    // Stub should return Ok with placeholder text
    assert!(result.is_ok());
    let generated_text = result.unwrap();
    assert!(!generated_text.is_empty());
}

#[tokio::test]
async fn test_llamacpp_error_types() {
    // Test that error types are properly defined
    let _model_load_error = LlamaCppError::ModelLoadFailed("test error".to_string());
    let _generation_error = LlamaCppError::GenerationFailed("test error".to_string());
    let _config_error = LlamaCppError::InvalidConfig("test error".to_string());
}

#[tokio::test]
async fn test_llamacpp_model_info() {
    let config = LlamaCppConfig {
        model_path: "/path/to/llama-3-8b.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };
    
    let client = LlamaCppClient::new(config).expect("client creation failed");
    
    // Get model information
    let model_name = client.model_name();
    assert!(!model_name.is_empty());
    
    let context_size = client.context_size();
    assert_eq!(context_size, 2048);
}
