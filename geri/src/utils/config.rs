use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("grpc_port must be non-zero")]
    InvalidPort,
    #[error("default_local_llm must be non-empty")]
    EmptyDefaultLocalLlm,
    #[error("vision_model must be non-empty")]
    EmptyVisionModel,
    #[error("Invalid local provider type: {0}")]
    InvalidLocalProviderType(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalProviderConfig {
    /// Provider type: "llamacpp", "bitnet", or "auto"
    pub provider_type: String,
    /// Path to llama.cpp models directory
    pub llamacpp_models_dir: String,
    /// Path to BitNet models directory
    pub bitnet_models_dir: String,
    /// Enable automatic provider selection based on hardware
    pub auto_select: bool,
    /// Minimum memory (MB) to use llama.cpp (otherwise use BitNet)
    pub llamacpp_min_memory_mb: u32,
}

impl Default for LocalProviderConfig {
    fn default() -> Self {
        Self {
            provider_type: "auto".to_string(),
            llamacpp_models_dir: "./models/llamacpp".to_string(),
            bitnet_models_dir: "./models/bitnet".to_string(),
            auto_select: true,
            llamacpp_min_memory_mb: 8000, // 8GB minimum for llama.cpp
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeriSettings {
    pub grpc_port: u16,
    pub default_local_llm: String,
    pub vision_model: String,
    #[serde(default)]
    pub local_provider: LocalProviderConfig,
}

impl GeriSettings {
    pub fn validate(&self) -> Result<(), SettingsError> {
        if self.grpc_port == 0 {
            return Err(SettingsError::InvalidPort);
        }
        if self.default_local_llm.trim().is_empty() {
            return Err(SettingsError::EmptyDefaultLocalLlm);
        }
        if self.vision_model.trim().is_empty() {
            return Err(SettingsError::EmptyVisionModel);
        }
        
        // Validate local provider type
        let valid_types = ["llamacpp", "bitnet", "auto"];
        if !valid_types.contains(&self.local_provider.provider_type.as_str()) {
            return Err(SettingsError::InvalidLocalProviderType(
                self.local_provider.provider_type.clone()
            ));
        }
        
        Ok(())
    }
}

impl Default for GeriSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50054,
            default_local_llm: "llama3-8b".to_string(),
            vision_model: "gpt-4v".to_string(),
            local_provider: LocalProviderConfig::default(),
        }
    }
}


pub struct SettingsManager {
    config_path: PathBuf,
    settings: Arc<RwLock<GeriSettings>>,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path,
            settings: Arc::new(RwLock::new(GeriSettings::default())),
        }
    }

    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            let content = tokio::fs::read_to_string(&self.config_path).await?;
            let settings: GeriSettings = serde_json::from_str(&content)?;
            settings.validate()?;
            *self.settings.write().await = settings;
            info!("Configuration loaded from {}", self.config_path.display());
        } else {
            let default_settings = GeriSettings::default();
            let content = serde_json::to_string_pretty(&default_settings)?;
            tokio::fs::write(&self.config_path, content).await?;
            *self.settings.write().await = default_settings;
            info!("Default configuration created at {}", self.config_path.display());
        }
        Ok(())
    }

    pub async fn get(&self) -> GeriSettings {
        self.settings.read().await.clone()
    }

    pub fn start_hot_reload(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.config_path.clone();
        let settings = Arc::clone(&self.settings);
        
        let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |result: Result<Event, notify::Error>| {
            match result {
                Ok(event) => {
                    if let EventKind::Modify(_) = event.kind {
                        if event.paths.iter().any(|p| p == &config_path) {
                            info!("Configuration file changed, reloading...");
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            if let Err(e) = rt.block_on(async {
                                let content = tokio::fs::read_to_string(&config_path).await?;
                                let new_settings: GeriSettings = serde_json::from_str(&content)?;
                                new_settings.validate()?;
                                *settings.write().await = new_settings;
                                Ok::<(), Box<dyn std::error::Error>>(())
                            }) {
                                error!("Failed to reload configuration: {}", e);
                            } else {
                                info!("Configuration reloaded successfully");
                            }
                        }
                    }
                }
                Err(e) => error!("File watcher error: {}", e),
            }
        })?;
        
        watcher.watch(&self.config_path.parent().unwrap(), RecursiveMode::NonRecursive)?;
        info!("Hot-reload watcher started for configuration");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default_ok() {
        let s = GeriSettings::default();
        assert!(s.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_port() {
        let mut s = GeriSettings::default();
        s.grpc_port = 0;
        assert!(s.validate().is_err());
        assert!(matches!(s.validate(), Err(SettingsError::InvalidPort)));
    }

    #[test]
    fn test_validate_empty_default_local_llm() {
        let mut s = GeriSettings::default();
        s.default_local_llm = String::new();
        assert!(s.validate().is_err());
        assert!(matches!(s.validate(), Err(SettingsError::EmptyDefaultLocalLlm)));
    }

    #[test]
    fn test_validate_empty_vision_model() {
        let mut s = GeriSettings::default();
        s.vision_model = "   ".to_string();
        assert!(s.validate().is_err());
        assert!(matches!(s.validate(), Err(SettingsError::EmptyVisionModel)));
    }
}
