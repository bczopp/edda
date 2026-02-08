use geri::llm::bitnet::{BitNetClient, BitNetConfig, BitNetError};

#[tokio::test]
async fn test_bitnet_client_creation() {
    let config = BitNetConfig {
        model_path: "/path/to/model.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    // Client creation should not fail
    let result = BitNetClient::new(config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bitnet_config_validation() {
    // Valid config
    let valid_config = BitNetConfig {
        model_path: "/valid/path.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    assert!(valid_config.validate().is_ok());
    
    // Empty model path should fail
    let invalid_config = BitNetConfig {
        model_path: "".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    assert!(invalid_config.validate().is_err());
    
    // Invalid context size (0) should fail
    let invalid_ctx = BitNetConfig {
        model_path: "/path/to/model.bitnet".to_string(),
        n_ctx: 0,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    assert!(invalid_ctx.validate().is_err());
    
    // Invalid thread count (0) should fail
    let invalid_threads = BitNetConfig {
        model_path: "/path/to/model.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 0,
        use_extreme_efficiency: true,
    };
    assert!(invalid_threads.validate().is_err());
}

#[tokio::test]
async fn test_bitnet_generate_stub() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    
    // Test generation (stub implementation for now)
    let prompt = "Hello, world!";
    let max_tokens = 100;
    
    let result = client.generate(prompt, max_tokens).await;
    
    // Stub should return Ok with placeholder text
    assert!(result.is_ok());
    let generated_text = result.unwrap();
    assert!(!generated_text.is_empty());
    assert!(generated_text.contains("BitNet"));
}

#[tokio::test]
async fn test_bitnet_extreme_efficiency_mode() {
    let config_efficient = BitNetConfig {
        model_path: "/path/to/model.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let config_normal = BitNetConfig {
        model_path: "/path/to/model.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: false,
    };
    
    let client_efficient = BitNetClient::new(config_efficient).expect("efficient client creation failed");
    let client_normal = BitNetClient::new(config_normal).expect("normal client creation failed");
    
    assert!(client_efficient.is_extreme_efficiency());
    assert!(!client_normal.is_extreme_efficiency());
}

#[tokio::test]
async fn test_bitnet_error_types() {
    // Test that error types are properly defined
    let _model_load_error = BitNetError::ModelLoadFailed("test error".to_string());
    let _generation_error = BitNetError::GenerationFailed("test error".to_string());
    let _config_error = BitNetError::InvalidConfig("test error".to_string());
}

#[tokio::test]
async fn test_bitnet_model_info() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b-1bit.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    
    // Get model information
    let model_name = client.model_name();
    assert!(!model_name.is_empty());
    
    let context_size = client.context_size();
    assert_eq!(context_size, 2048);
    
    let bit_depth = client.bit_depth();
    assert_eq!(bit_depth, 1); // 1-bit model
}

#[tokio::test]
async fn test_bitnet_memory_efficiency() {
    let config = BitNetConfig {
        model_path: "/path/to/bitnet-3b.bitnet".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        use_extreme_efficiency: true,
    };
    
    let client = BitNetClient::new(config).expect("client creation failed");
    
    // Estimated memory usage should be significantly lower than full precision models
    let estimated_memory_mb = client.estimated_memory_usage_mb();
    
    // 1-bit models should use ~90% less memory than full precision
    // For a 3B parameter model: ~400MB vs ~12GB for FP16
    assert!(estimated_memory_mb < 1000); // Should be under 1GB for 3B model
}
