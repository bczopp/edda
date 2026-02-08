use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlamaCppError {
    #[error("Failed to load model: {0}")]
    ModelLoadFailed(String),
    #[error("Text generation failed: {0}")]
    GenerationFailed(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaCppConfig {
    /// Path to the GGUF model file
    pub model_path: String,
    /// Context size (number of tokens)
    pub n_ctx: u32,
    /// Number of CPU threads to use
    pub n_threads: u32,
    /// Number of GPU layers to offload (0 = CPU only)
    pub n_gpu_layers: u32,
}

impl LlamaCppConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), LlamaCppError> {
        if self.model_path.is_empty() {
            return Err(LlamaCppError::InvalidConfig(
                "model_path cannot be empty".to_string(),
            ));
        }
        
        if self.n_ctx == 0 {
            return Err(LlamaCppError::InvalidConfig(
                "n_ctx must be greater than 0".to_string(),
            ));
        }
        
        if self.n_threads == 0 {
            return Err(LlamaCppError::InvalidConfig(
                "n_threads must be greater than 0".to_string(),
            ));
        }
        
        Ok(())
    }
}

/// Client for llama.cpp integration
/// 
/// This client provides a Rust interface to llama.cpp for running local LLM inference.
/// It supports GGUF format models and provides CPU and GPU acceleration options.
pub struct LlamaCppClient {
    config: LlamaCppConfig,
    model_name: String,
}

impl LlamaCppClient {
    /// Create a new llama.cpp client
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration for the llama.cpp model
    /// 
    /// # Errors
    /// 
    /// Returns an error if the configuration is invalid
    pub fn new(config: LlamaCppConfig) -> Result<Self, LlamaCppError> {
        config.validate()?;
        
        // Extract model name from path (last path component without extension)
        let model_name = config
            .model_path
            .split(['/', '\\'])
            .last()
            .unwrap_or("unknown")
            .trim_end_matches(".gguf")
            .to_string();
        
        Ok(Self {
            config,
            model_name,
        })
    }
    
    /// Generate text from a prompt
    /// 
    /// # Arguments
    /// 
    /// * `prompt` - The input prompt
    /// * `max_tokens` - Maximum number of tokens to generate
    /// 
    /// # Returns
    /// 
    /// The generated text
    /// 
    /// # Errors
    /// 
    /// Returns an error if generation fails
    /// 
    /// # Note
    /// 
    /// This is currently a stub implementation. Real FFI integration with llama.cpp
    /// will be added in the next step.
    pub async fn generate(&self, prompt: &str, max_tokens: u32) -> Result<String, LlamaCppError> {
        // TODO: Implement actual llama.cpp FFI bindings
        // For now, return a placeholder that indicates the integration point
        
        // Simulate some processing
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        Ok(format!(
            "[llama.cpp stub] Generated {} tokens for prompt: '{}'",
            max_tokens.min(50),
            prompt.chars().take(50).collect::<String>()
        ))
    }
    
    /// Get the model name
    pub fn model_name(&self) -> &str {
        &self.model_name
    }
    
    /// Get the context size
    pub fn context_size(&self) -> u32 {
        self.config.n_ctx
    }
    
    /// Get the number of threads
    pub fn num_threads(&self) -> u32 {
        self.config.n_threads
    }
    
    /// Get the number of GPU layers
    pub fn num_gpu_layers(&self) -> u32 {
        self.config.n_gpu_layers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_validation_valid() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_validation_empty_path() {
        let config = LlamaCppConfig {
            model_path: "".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_validation_zero_ctx() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 0,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_validation_zero_threads() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 0,
            n_gpu_layers: 0,
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_client_creation() {
        let config = LlamaCppConfig {
            model_path: "/path/to/llama-3-8b.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        
        let client = LlamaCppClient::new(config).expect("client creation failed");
        assert_eq!(client.model_name(), "llama-3-8b");
        assert_eq!(client.context_size(), 2048);
        assert_eq!(client.num_threads(), 4);
        assert_eq!(client.num_gpu_layers(), 0);
    }
    
    #[tokio::test]
    async fn test_generate_stub() {
        let config = LlamaCppConfig {
            model_path: "/path/to/model.gguf".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
        };
        
        let client = LlamaCppClient::new(config).expect("client creation failed");
        let result = client.generate("Hello, world!", 100).await;
        
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(!text.is_empty());
        assert!(text.contains("llama.cpp stub"));
    }
}
