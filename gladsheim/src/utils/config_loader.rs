//! Configuration Loader with Hot-Reload support

use crate::utils::config::GladsheimConfig;
use crate::utils::{GladsheimError, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::PathBuf;
use tracing::{info, warn, error};

pub struct ConfigLoader {
    config: Arc<RwLock<GladsheimConfig>>,
    config_path: PathBuf,
    watcher: Option<RecommendedWatcher>,
}

impl ConfigLoader {
    pub fn new(config_path: PathBuf) -> Result<Self> {
        info!("Creating ConfigLoader for: {:?}", config_path);
        
        let config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| GladsheimError::ConfigurationError(format!("Failed to read config: {}", e)))?;
            GladsheimConfig::from_json(&content)?
        } else {
            info!("Config file not found, using defaults");
            GladsheimConfig::default()
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
            .map_err(|e| GladsheimError::ConfigurationError(format!("Failed to read config: {}", e)))?;
        
        let new_config = GladsheimConfig::from_json(&content)?;
        
        {
            let mut config = self.config.write().await;
            *config = new_config;
        }
        
        info!("Configuration loaded successfully");
        Ok(())
    }
    
    pub async fn get_config(&self) -> GladsheimConfig {
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
                                    match GladsheimConfig::from_json(&content) {
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
        .map_err(|e| GladsheimError::ConfigurationError(format!("Failed to create watcher: {}", e)))?;
        
        watcher.watch(&self.config_path.parent().unwrap_or(std::path::Path::new(".")), RecursiveMode::NonRecursive)
            .map_err(|e| GladsheimError::ConfigurationError(format!("Failed to watch config: {}", e)))?;
        
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
        
        let config_json = r#"{"grpc_host": "127.0.0.1", "grpc_port": 50061, "max_services": 10, "resource_limits": {"default_memory_mb": 512, "default_cpu_percent": 25.0, "max_memory_mb": 2048, "max_cpu_percent": 100.0}, "health_monitoring": {"check_interval_ms": 5000, "auto_restart": true, "max_restart_attempts": 3, "restart_backoff_ms": 1000}, "service_loader": {"startup_timeout_ms": 5000, "shutdown_timeout_ms": 1000, "graceful_shutdown": true}}"#;
        std::fs::write(&config_path, config_json).unwrap();
        
        let mut loader = ConfigLoader::new(config_path.clone()).unwrap();
        loader.load().await.unwrap();
        
        let config = loader.get_config().await;
        assert_eq!(config.grpc_port, 50061);
        assert_eq!(config.max_services, 10);
    }
}
