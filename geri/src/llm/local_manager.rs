use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::llm::llamacpp::{LlamaCppClient, LlamaCppConfig, LlamaCppLLMProvider};
use crate::llm::bitnet::{BitNetClient, BitNetConfig, BitNetLLMProvider};
use crate::llm::provider::LLMProvider;

#[derive(Debug, Error)]
pub enum LocalManagerError {
    #[error("Hardware detection failed: {0}")]
    HardwareDetectionFailed(String),
    #[error("Provider creation failed: {0}")]
    ProviderCreationFailed(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Hardware profile for automatic provider selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// Total system memory in MB
    pub total_memory_mb: u32,
    /// Currently available memory in MB
    pub available_memory_mb: u32,
    /// Number of CPU cores
    pub cpu_cores: u32,
    /// Whether a GPU is available
    pub has_gpu: bool,
    /// GPU memory in MB (0 if no GPU)
    pub gpu_memory_mb: u32,
}

/// Provider selection result
#[derive(Debug, Clone)]
pub struct ProviderSelection {
    /// Provider type: "llamacpp" or "bitnet"
    pub provider_type: String,
    /// Recommended model size: "3b", "7b", "8b", etc.
    pub model_size: String,
    /// Rationale for selection
    pub rationale: String,
}

/// Configuration for local model
#[derive(Debug, Clone)]
pub struct LocalModelConfig {
    pub model_path: String,
    pub n_ctx: u32,
    pub n_threads: u32,
    pub n_gpu_layers: u32,
}

/// Local LLM Manager for automatic provider selection
/// 
/// This manager detects hardware capabilities and automatically selects
/// the best local LLM provider (llama.cpp vs. BitNet.cpp) based on:
/// - Available memory
/// - CPU capabilities
/// - GPU availability
/// - Battery constraints (mobile)
/// 
/// It also integrates with ModelDownloader to ensure models are available.
pub struct LocalLLMManager {
    downloader: crate::llm::model_downloader::ModelDownloader,
}

impl LocalLLMManager {
    /// Create a new LocalLLMManager
    /// 
    /// # Arguments
    /// 
    /// * `models_dir` - Optional models directory (defaults to "./models")
    pub fn new() -> Result<Self, LocalManagerError> {
        Self::with_models_dir("./models".to_string())
    }
    
    /// Create a new LocalLLMManager with custom models directory
    pub fn with_models_dir(models_dir: String) -> Result<Self, LocalManagerError> {
        let downloader = crate::llm::model_downloader::ModelDownloader::new(models_dir)
            .map_err(|e| LocalManagerError::HardwareDetectionFailed(e.to_string()))?;
        
        Ok(Self {
            downloader,
        })
    }
    
    /// Detect hardware profile
    /// 
    /// Uses sysinfo crate to detect system capabilities
    pub fn detect_hardware(&self) -> HardwareProfile {
        use sysinfo::{System, SystemExt, CpuExt};
        
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // Get memory info
        let total_memory_mb = (sys.total_memory() / 1024 / 1024) as u32;
        let available_memory_mb = (sys.available_memory() / 1024 / 1024) as u32;
        
        // Get CPU info
        let cpu_cores = sys.cpus().len() as u32;
        
        // GPU detection - basic check for now
        // TODO: Integrate proper GPU detection library (e.g., vulkano, wgpu)
        // For now, we'll check for common GPU indicators
        let (has_gpu, gpu_memory_mb) = self.detect_gpu();
        
        tracing::debug!(
            total_memory_mb = total_memory_mb,
            available_memory_mb = available_memory_mb,
            cpu_cores = cpu_cores,
            has_gpu = has_gpu,
            gpu_memory_mb = gpu_memory_mb,
            "Hardware profile detected"
        );
        
        HardwareProfile {
            total_memory_mb,
            available_memory_mb,
            cpu_cores,
            has_gpu,
            gpu_memory_mb,
        }
    }
    
    /// Detect GPU availability and memory
    /// 
    /// Basic GPU detection. Returns (has_gpu, gpu_memory_mb).
    /// Future: Integrate proper GPU detection library.
    fn detect_gpu(&self) -> (bool, u32) {
        // Placeholder: Check environment variables or system info
        // For now, return no GPU (conservative approach)
        // TODO: Integrate vulkano/wgpu for proper GPU detection
        
        // Check for common GPU environment variables
        if std::env::var("CUDA_VISIBLE_DEVICES").is_ok() {
            // CUDA GPU available, estimate memory (conservative 8GB)
            tracing::info!("CUDA GPU detected via environment variable");
            return (true, 8000);
        }
        
        if std::env::var("HIP_VISIBLE_DEVICES").is_ok() {
            // AMD GPU available, estimate memory (conservative 8GB)
            tracing::info!("AMD GPU detected via environment variable");
            return (true, 8000);
        }
        
        // No GPU detected
        (false, 0)
    }
    
    /// Select the best provider based on hardware profile
    pub fn select_provider(&self, profile: &HardwareProfile) -> ProviderSelection {
        // Decision logic:
        // 1. Mobile/Low memory (< 4GB available) -> BitNet for efficiency
        // 2. Medium memory (4-8GB) -> BitNet for balance
        // 3. High memory (> 8GB) -> llama.cpp for quality
        
        if profile.available_memory_mb < 4000 {
            // Low memory: Use BitNet for extreme efficiency
            ProviderSelection {
                provider_type: "bitnet".to_string(),
                model_size: self.recommend_model_size(profile),
                rationale: format!(
                    "BitNet selected: Low memory system ({}MB available). BitNet provides 90% memory reduction.",
                    profile.available_memory_mb
                ),
            }
        } else if profile.available_memory_mb < 8000 {
            // Medium memory: Still prefer BitNet for safety
            ProviderSelection {
                provider_type: "bitnet".to_string(),
                model_size: self.recommend_model_size(profile),
                rationale: format!(
                    "BitNet selected: Medium memory system ({}MB available). BitNet balances quality and efficiency.",
                    profile.available_memory_mb
                ),
            }
        } else {
            // High memory: Use llama.cpp for best quality
            ProviderSelection {
                provider_type: "llamacpp".to_string(),
                model_size: self.recommend_model_size(profile),
                rationale: format!(
                    "llama.cpp selected: High memory system ({}MB available). llama.cpp provides best quality.",
                    profile.available_memory_mb
                ),
            }
        }
    }
    
    /// Recommend model size based on available memory
    pub fn recommend_model_size(&self, profile: &HardwareProfile) -> String {
        // Model size recommendations:
        // < 4GB available -> 3B model
        // 4-8GB available -> 7B model
        // 8-16GB available -> 8B model
        // > 16GB available -> 13B model (or larger)
        
        if profile.available_memory_mb < 4000 {
            "3b".to_string()
        } else if profile.available_memory_mb < 8000 {
            "7b".to_string()
        } else if profile.available_memory_mb < 16000 {
            "8b".to_string()
        } else {
            "13b".to_string()
        }
    }
    
    /// Generate model configuration based on hardware profile
    pub fn generate_model_config(&self, profile: &HardwareProfile) -> LocalModelConfig {
        let selection = self.select_provider(profile);
        
        // Calculate optimal context size based on available memory
        let n_ctx = if profile.available_memory_mb < 4000 {
            1024 // Small context for low memory
        } else if profile.available_memory_mb < 8000 {
            2048 // Medium context
        } else {
            4096 // Large context for high memory
        };
        
        // Use all available CPU cores (with cap for efficiency)
        let n_threads = profile.cpu_cores.min(8);
        
        // GPU offloading: use all layers if GPU available
        let n_gpu_layers = if profile.has_gpu {
            // Estimate layers based on GPU memory
            // Typical: ~200MB per layer for 7B model
            (profile.gpu_memory_mb / 200).min(32)
        } else {
            0
        };
        
        // Generate model filename and check if exists
        let model_filename = if selection.provider_type == "bitnet" {
            format!("bitnet-{}.bitnet", selection.model_size)
        } else {
            format!("llama-{}.gguf", selection.model_size)
        };
        
        let model_path = self.downloader.get_model_path(&model_filename);
        
        // Check if model exists, log warning if not
        if !self.downloader.check_model_exists(&model_filename) {
            tracing::warn!(
                "Model {} not found at {:?}. You may need to download it manually.",
                model_filename,
                model_path
            );
            
            // Get recommended source
            if let Some(source) = self.downloader.get_recommended_source(&selection.model_size, &selection.provider_type) {
                tracing::info!("Recommended download: {}", source.get_url());
            }
        }
        
        LocalModelConfig {
            model_path: model_path.to_string_lossy().to_string(),
            n_ctx,
            n_threads,
            n_gpu_layers,
        }
    }
    
    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>, LocalManagerError> {
        self.downloader.list_models().await
            .map_err(|e| LocalManagerError::ProviderCreationFailed(e.to_string()))
    }
    
    /// Create provider automatically based on detected hardware
    pub async fn create_provider_auto(&self) -> Result<Box<dyn LLMProvider>, LocalManagerError> {
        let profile = self.detect_hardware();
        let selection = self.select_provider(&profile);
        let config = self.generate_model_config(&profile);
        
        tracing::info!(
            provider_type = %selection.provider_type,
            model_size = %selection.model_size,
            memory_mb = profile.available_memory_mb,
            "Auto-selected local LLM provider: {}",
            selection.rationale
        );
        
        self.create_provider(&selection.provider_type, config).await
    }
    
    /// Create provider with explicit type and config
    pub async fn create_provider(
        &self,
        provider_type: &str,
        config: LocalModelConfig,
    ) -> Result<Box<dyn LLMProvider>, LocalManagerError> {
        match provider_type {
            "llamacpp" => {
                let llamacpp_config = LlamaCppConfig {
                    model_path: config.model_path,
                    n_ctx: config.n_ctx,
                    n_threads: config.n_threads,
                    n_gpu_layers: config.n_gpu_layers,
                };
                
                let client = LlamaCppClient::new(llamacpp_config)
                    .map_err(|e| LocalManagerError::ProviderCreationFailed(e.to_string()))?;
                
                let provider = LlamaCppLLMProvider::new(client);
                Ok(Box::new(provider))
            }
            "bitnet" => {
                let bitnet_config = BitNetConfig {
                    model_path: config.model_path,
                    n_ctx: config.n_ctx,
                    n_threads: config.n_threads,
                    use_extreme_efficiency: true, // Always use extreme efficiency
                };
                
                let client = BitNetClient::new(bitnet_config)
                    .map_err(|e| LocalManagerError::ProviderCreationFailed(e.to_string()))?;
                
                let provider = BitNetLLMProvider::new(client);
                Ok(Box::new(provider))
            }
            _ => Err(LocalManagerError::InvalidConfig(
                format!("Unknown provider type: {}", provider_type)
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_manager_creation() {
        let manager = LocalLLMManager::new();
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_hardware_detection() {
        let manager = LocalLLMManager::new().expect("manager creation failed");
        let profile = manager.detect_hardware();
        
        assert!(profile.total_memory_mb > 0);
        assert!(profile.cpu_cores > 0);
    }
    
    #[test]
    fn test_provider_selection_low_memory() {
        let manager = LocalLLMManager::new().expect("manager creation failed");
        
        let profile = HardwareProfile {
            total_memory_mb: 3072,
            available_memory_mb: 1500,
            cpu_cores: 4,
            has_gpu: false,
            gpu_memory_mb: 0,
        };
        
        let selection = manager.select_provider(&profile);
        assert_eq!(selection.provider_type, "bitnet");
    }
    
    #[test]
    fn test_provider_selection_high_memory() {
        let manager = LocalLLMManager::new().expect("manager creation failed");
        
        let profile = HardwareProfile {
            total_memory_mb: 16384,
            available_memory_mb: 12000,
            cpu_cores: 8,
            has_gpu: false,
            gpu_memory_mb: 0,
        };
        
        let selection = manager.select_provider(&profile);
        assert_eq!(selection.provider_type, "llamacpp");
    }
    
    #[test]
    fn test_model_size_recommendation() {
        let manager = LocalLLMManager::new().expect("manager creation failed");
        
        let profile_3b = HardwareProfile {
            total_memory_mb: 3000,
            available_memory_mb: 1500,
            cpu_cores: 4,
            has_gpu: false,
            gpu_memory_mb: 0,
        };
        assert_eq!(manager.recommend_model_size(&profile_3b), "3b");
        
        let profile_7b = HardwareProfile {
            total_memory_mb: 6000,
            available_memory_mb: 5000,
            cpu_cores: 4,
            has_gpu: false,
            gpu_memory_mb: 0,
        };
        assert_eq!(manager.recommend_model_size(&profile_7b), "7b");
    }
}
