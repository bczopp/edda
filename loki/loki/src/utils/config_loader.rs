//! Configuration Loader with Hot-Reload support

use super::config::LokiConfig;
use super::{LokiError, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::PathBuf;
use tracing::{info, warn, error};

pub struct ConfigLoader {
    config: Arc<RwLock<LokiConfig>>,
    config_path: PathBuf,
    watcher: Option<RecommendedWatcher>,
}

impl ConfigLoader {
    pub fn new(config_path: PathBuf) -> Result<Self> {
        info!("Creating ConfigLoader for: {:?}", config_path);
        
        let config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| LokiError::ConfigurationError(format!("Failed to read config: {}", e)))?;
            LokiConfig::from_json(&content)?
        } else {
            info!("Config file not found, using defaults");
            LokiConfig::default()
        };
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
            watcher: None,
        })
    }
    
    pub async fn load(&mut self) -> Result<()> {
        if !self.config_path.exists() {
            warn!("Config file does not exist, using defaults");
            return Ok(());
        }
        
        let content = std::fs::read_to_string(&self.config_path)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to read config: {}", e)))?;
        
        let new_config = LokiConfig::from_json(&content)?;
        
        {
            let mut config = self.config.write().await;
            *config = new_config;
        }
        
        info!("Configuration loaded successfully");
        Ok(())
    }
    
    pub async fn get_config(&self) -> LokiConfig {
        let config = self.config.read().await;
        config.clone()
    }
    
    pub fn start_watching(&mut self) -> Result<()> {
        use notify::Watcher as _;
        
        let config = Arc::clone(&self.config);
        let config_path = self.config_path.clone();
        
        let mut watcher = notify::recommended_watcher(move |result: notify::Result<Event>| {
            match result {
                Ok(event) => {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        info!("Config file changed, reloading...");
                        
                        // Reload config
                        if config_path.exists() {
                            match std::fs::read_to_string(&config_path) {
                                Ok(content) => {
                                    match LokiConfig::from_json(&content) {
                                        Ok(new_config) => {
                                            let rt = tokio::runtime::Handle::try_current();
                                            if let Ok(handle) = rt {
                                                handle.block_on(async {
                                                    let mut cfg = config.write().await;
                                                    *cfg = new_config;
                                                });
                                                info!("Configuration reloaded successfully");
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to parse config: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to read config file: {}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Config file watcher error: {}", e);
                }
            }
        })
        .map_err(|e| LokiError::ConfigurationError(format!("Failed to create watcher: {}", e)))?;
        
        watcher.watch(&self.config_path.parent().unwrap_or(std::path::Path::new(".")), RecursiveMode::NonRecursive)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to watch config: {}", e)))?;
        
        self.watcher = Some(watcher);
        info!("Config file watcher started");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_config_loader_new() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        let loader = ConfigLoader::new(config_path.clone());
        assert!(loader.is_ok());
    }
    
    #[tokio::test]
    async fn test_config_loader_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        let config_json = r#"{
            "grpc_port": 50075,
            "script_storage_path": "./scripts",
            "resource_limits": {
                "max_memory_mb": 20,
                "max_execution_time_ms": 10000,
                "max_cpu_percent": 80
            }
        }"#;
        std::fs::write(&config_path, config_json).unwrap();
        
        let mut loader = ConfigLoader::new(config_path.clone()).unwrap();
        loader.load().await.unwrap();
        
        let config = loader.get_config().await;
        assert_eq!(config.grpc_port, 50075);
        assert_eq!(config.resource_limits.max_memory_mb, 20);
    }
}
