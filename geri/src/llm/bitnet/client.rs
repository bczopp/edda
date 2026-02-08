use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitNetError {
    #[error("Failed to load BitNet model: {0}")]
    ModelLoadFailed(String),
    #[error("Text generation failed: {0}")]
    GenerationFailed(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitNetConfig {
    /// Path to the BitNet model file
    pub model_path: String,
    /// Context size (number of tokens)
    pub n_ctx: u32,
    /// Number of CPU threads to use
    pub n_threads: u32,
    /// Enable extreme efficiency mode (1-bit quantization optimizations)
    pub use_extreme_efficiency: bool,
}

impl BitNetConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), BitNetError> {
        if self.model_path.is_empty() {
            return Err(BitNetError::InvalidConfig(
                "model_path cannot be empty".to_string(),
            ));
        }
        
        if self.n_ctx == 0 {
            return Err(BitNetError::InvalidConfig(
                "n_ctx must be greater than 0".to_string(),
            ));
        }
        
        if self.n_threads == 0 {
            return Err(BitNetError::InvalidConfig(
                "n_threads must be greater than 0".to_string(),
            ));
        }
        
        Ok(())
    }
}

/// Client for BitNet.cpp integration
/// 
/// BitNet.cpp provides extreme efficiency through 1-bit quantization, achieving:
/// - 90% reduction in memory usage compared to full precision models
/// - 5-10x faster inference speed
/// - Maintained quality through specialized training
/// 
/// This client provides a Rust interface to BitNet.cpp for running ultra-efficient
/// local LLM inference on resource-constrained devices.
pub struct BitNetClient {
    config: BitNetConfig,
    model_name: String,
}

impl BitNetClient {
    /// Create a new BitNet.cpp client
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration for the BitNet model
    /// 
    /// # Errors
    /// 
    /// Returns an error if the configuration is invalid
    pub fn new(config: BitNetConfig) -> Result<Self, BitNetError> {
        config.validate()?;
        
        // Extract model name from path (last path component without extension)
        let model_name = config
            .model_path
            .split(['/', '\\'])
            .last()
            .unwrap_or("unknown")
            .trim_end_matches(".bitnet")
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
    /// This is currently a stub implementation. Real FFI integration with BitNet.cpp
    /// will be added in the next step. The implementation will leverage 1-bit
    /// quantization for extreme efficiency.
    pub async fn generate(&self, prompt: &str, max_tokens: u32) -> Result<String, BitNetError> {
        // TODO: Implement actual BitNet.cpp FFI bindings
        // For now, return a placeholder that indicates the integration point
        
        // Simulate faster processing due to 1-bit efficiency
        let processing_time = if self.config.use_extreme_efficiency {
            5 // 5ms for extreme efficiency mode
        } else {
            10 // 10ms for normal mode
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(processing_time)).await;
        
        let efficiency_marker = if self.config.use_extreme_efficiency {
            "[EXTREME-EFFICIENCY]"
        } else {
            "[NORMAL]"
        };
        
        Ok(format!(
            "[BitNet.cpp stub {} 1-bit] Generated {} tokens for prompt: '{}'",
            efficiency_marker,
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
    
    /// Check if extreme efficiency mode is enabled
    pub fn is_extreme_efficiency(&self) -> bool {
        self.config.use_extreme_efficiency
    }
    
    /// Get the bit depth (always 1 for BitNet)
    pub fn bit_depth(&self) -> u8 {
        1
    }
    
    /// Estimate memory usage in MB
    /// 
    /// BitNet models use approximately 90% less memory than full precision models.
    /// For a 3B parameter model:
    /// - Full precision (FP16): ~6GB
    /// - BitNet (1-bit): ~400MB
    pub fn estimated_memory_usage_mb(&self) -> u32 {
        // Extract parameter count from model name (e.g., "bitnet-3b" -> 3)
        let param_count_billions = self
            .model_name
            .split('-')
            .find(|part| part.ends_with('b'))
            .and_then(|part| part.trim_end_matches('b').parse::<u32>().ok())
            .unwrap_or(3); // Default to 3B if not found
        
        // 1-bit models use ~130MB per billion parameters (vs ~2GB for FP16)
        param_count_billions * 130
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_validation_valid() {
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_validation_empty_path() {
        let config = BitNetConfig {
            model_path: "".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_validation_zero_ctx() {
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 0,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_validation_zero_threads() {
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 0,
            use_extreme_efficiency: true,
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_client_creation() {
        let config = BitNetConfig {
            model_path: "/path/to/bitnet-3b-1bit.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client = BitNetClient::new(config).expect("client creation failed");
        assert_eq!(client.model_name(), "bitnet-3b-1bit");
        assert_eq!(client.context_size(), 2048);
        assert_eq!(client.num_threads(), 4);
        assert!(client.is_extreme_efficiency());
        assert_eq!(client.bit_depth(), 1);
    }
    
    #[tokio::test]
    async fn test_generate_stub() {
        let config = BitNetConfig {
            model_path: "/path/to/model.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client = BitNetClient::new(config).expect("client creation failed");
        let result = client.generate("Hello, world!", 100).await;
        
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(!text.is_empty());
        assert!(text.contains("BitNet.cpp stub"));
        assert!(text.contains("1-bit"));
    }
    
    #[test]
    fn test_memory_estimation() {
        let config_3b = BitNetConfig {
            model_path: "/path/to/bitnet-3b.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client_3b = BitNetClient::new(config_3b).expect("client creation failed");
        let memory_3b = client_3b.estimated_memory_usage_mb();
        
        // 3B model should use ~390MB (3 * 130)
        assert_eq!(memory_3b, 390);
        
        let config_7b = BitNetConfig {
            model_path: "/path/to/bitnet-7b.bitnet".to_string(),
            n_ctx: 2048,
            n_threads: 4,
            use_extreme_efficiency: true,
        };
        
        let client_7b = BitNetClient::new(config_7b).expect("client creation failed");
        let memory_7b = client_7b.estimated_memory_usage_mb();
        
        // 7B model should use ~910MB (7 * 130)
        assert_eq!(memory_7b, 910);
        
        // Verify BitNet is much more efficient than full precision
        // (A 3B FP16 model would be ~6000MB)
        assert!(memory_3b < 1000);
        assert!(memory_7b < 2000);
    }
}
