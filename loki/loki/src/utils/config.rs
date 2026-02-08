//! Configuration system for Loki

use serde::{Deserialize, Serialize};
use super::{LokiError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiConfig {
    pub grpc_port: u16,
    pub script_storage_path: String,
    pub resource_limits: ResourceLimitsConfig,
    pub children_config: ChildrenConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimitsConfig {
    pub max_memory_mb: u32,
    pub max_execution_time_ms: u32,
    pub max_cpu_percent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildrenConfig {
    pub fenrir: ChildServiceConfig,
    pub jormungandr: ChildServiceConfig,
    pub hel: ChildServiceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildServiceConfig {
    pub enabled: bool,
    pub address: String,
}

impl Default for LokiConfig {
    fn default() -> Self {
        Self {
            grpc_port: 50070,
            script_storage_path: "./scripts".to_string(),
            resource_limits: ResourceLimitsConfig::default(),
            children_config: ChildrenConfig::default(),
        }
    }
}

impl Default for ResourceLimitsConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 10,
            max_execution_time_ms: 5000,
            max_cpu_percent: 50,
        }
    }
}

impl Default for ChildrenConfig {
    fn default() -> Self {
        Self {
            fenrir: ChildServiceConfig {
                enabled: false,
                address: "127.0.0.1:50071".to_string(),
            },
            jormungandr: ChildServiceConfig {
                enabled: false,
                address: "127.0.0.1:50072".to_string(),
            },
            hel: ChildServiceConfig {
                enabled: false,
                address: "127.0.0.1:50073".to_string(),
            },
        }
    }
}

impl LokiConfig {
    pub fn validate(&self) -> Result<()> {
        if self.grpc_port == 0 {
            return Err(LokiError::ConfigurationError(
                "grpc_port cannot be 0".to_string()
            ));
        }
        
        if self.resource_limits.max_memory_mb == 0 {
            return Err(LokiError::ConfigurationError(
                "max_memory_mb cannot be 0".to_string()
            ));
        }
        
        if self.resource_limits.max_execution_time_ms == 0 {
            return Err(LokiError::ConfigurationError(
                "max_execution_time_ms cannot be 0".to_string()
            ));
        }
        
        if self.resource_limits.max_cpu_percent > 100 {
            return Err(LokiError::ConfigurationError(
                "max_cpu_percent cannot exceed 100".to_string()
            ));
        }
        
        Ok(())
    }
    
    pub fn from_json(json: &str) -> Result<Self> {
        let config: LokiConfig = serde_json::from_str(json)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to parse JSON: {}", e)))?;
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to serialize JSON: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimitsConfig::default();
        assert_eq!(limits.max_memory_mb, 10);
        assert_eq!(limits.max_execution_time_ms, 5000);
        assert_eq!(limits.max_cpu_percent, 50);
    }
    
    #[test]
    fn test_children_config_default() {
        let config = ChildrenConfig::default();
        assert!(!config.fenrir.enabled);
        assert!(!config.jormungandr.enabled);
        assert!(!config.hel.enabled);
    }
}
