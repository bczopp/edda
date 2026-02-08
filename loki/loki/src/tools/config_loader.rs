//! Tool Configuration Loader with Hot-Reload support

use super::config::ToolConfig;
use crate::utils::{LokiError, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::PathBuf;
use tracing::{info, warn, error};

pub struct ToolConfigLoader {
    config: Arc<RwLock<ToolConfig>>,
    config_path: PathBuf,
    watcher: Option<RecommendedWatcher>,
}

impl ToolConfigLoader {
    pub fn new(config_path: PathBuf) -> Result<Self> {
        info!("Creating ToolConfigLoader for: {:?}", config_path);
        
        let config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| LokiError::ConfigurationError(format!("Failed to read tool config: {}", e)))?;
            ToolConfig::from_toml(&content)?
        } else {
            info!("Tool config file not found, using defaults");
            ToolConfig::default()
        };
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
            watcher: None,
        })
    }
    
    pub async fn load(&mut self) -> Result<()> {
        if !self.config_path.exists() {
            warn!("Tool config file does not exist, using defaults");
            return Ok(());
        }
        
        let content = std::fs::read_to_string(&self.config_path)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to read tool config: {}", e)))?;
        
        let new_config = ToolConfig::from_toml(&content)?;
        
        {
            let mut config = self.config.write().await;
            *config = new_config;
        }
        
        info!("Tool configuration loaded successfully");
        Ok(())
    }
    
    pub async fn get_config(&self) -> ToolConfig {
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
                        info!("Tool config file changed, reloading...");
                        
                        // Reload config
                        if config_path.exists() {
                            match std::fs::read_to_string(&config_path) {
                                Ok(content) => {
                                    match ToolConfig::from_toml(&content) {
                                        Ok(new_config) => {
                                            let rt = tokio::runtime::Handle::try_current();
                                            if let Ok(handle) = rt {
                                                handle.block_on(async {
                                                    let mut cfg = config.write().await;
                                                    *cfg = new_config;
                                                });
                                                info!("Tool configuration reloaded successfully");
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to parse tool config: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to read tool config file: {}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Tool config file watcher error: {}", e);
                }
            }
        })
        .map_err(|e| LokiError::ConfigurationError(format!("Failed to create watcher: {}", e)))?;
        
        watcher.watch(&self.config_path.parent().unwrap_or(std::path::Path::new(".")), RecursiveMode::NonRecursive)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to watch tool config: {}", e)))?;
        
        self.watcher = Some(watcher);
        info!("Tool config file watcher started");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_tool_config_loader_new() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("tools.toml");
        
        let loader = ToolConfigLoader::new(config_path.clone());
        assert!(loader.is_ok());
    }
    
    #[tokio::test]
    async fn test_tool_config_loader_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("tools.toml");
        
        let toml = r#"
[[tools]]
name = "test_tool"
description = "Test tool"
return_type = "String"

[tools.script]
inline = "return 'test'"
"#;
        std::fs::write(&config_path, toml).unwrap();
        
        let mut loader = ToolConfigLoader::new(config_path.clone()).unwrap();
        loader.load().await.unwrap();
        
        let config = loader.get_config().await;
        assert_eq!(config.tools.len(), 1);
        assert_eq!(config.tools[0].name, "test_tool");
    }
}
