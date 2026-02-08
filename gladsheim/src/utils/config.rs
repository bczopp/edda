//! Configuration system for Gladsheim

use serde::{Deserialize, Serialize};
use crate::utils::{GladsheimError, Result};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Midgard,
    Alfheim,
    Asgard,
    Ragnarok,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GladsheimConfig {
    pub grpc_host: String,
    pub grpc_port: u16,
    pub max_services: u32,
    pub resource_limits: ResourceLimitsConfig,
    pub health_monitoring: HealthMonitoringConfig,
    pub service_loader: ServiceLoaderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimitsConfig {
    pub default_memory_mb: u64,
    pub default_cpu_percent: f32,
    pub max_memory_mb: u64,
    pub max_cpu_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    pub check_interval_ms: u64,
    pub auto_restart: bool,
    pub max_restart_attempts: u32,
    pub restart_backoff_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLoaderConfig {
    pub startup_timeout_ms: u64,
    pub shutdown_timeout_ms: u64,
    pub graceful_shutdown: bool,
}

impl GladsheimConfig {
    pub fn default() -> Self {
        Self::for_platform(Platform::Asgard) // Default to server
    }
    
    pub fn for_platform(platform: Platform) -> Self {
        let (max_services, resource_limits) = match platform {
            Platform::Midgard => (
                15,
                ResourceLimitsConfig {
                    default_memory_mb: 512,
                    default_cpu_percent: 25.0,
                    max_memory_mb: 2048,
                    max_cpu_percent: 100.0,
                },
            ),
            Platform::Alfheim => (
                5,
                ResourceLimitsConfig {
                    default_memory_mb: 256,
                    default_cpu_percent: 25.0,
                    max_memory_mb: 1024,
                    max_cpu_percent: 100.0,
                },
            ),
            Platform::Asgard => (
                25,
                ResourceLimitsConfig {
                    default_memory_mb: 2048,
                    default_cpu_percent: 75.0,
                    max_memory_mb: 8192,
                    max_cpu_percent: 100.0,
                },
            ),
            Platform::Ragnarok => (
                8,
                ResourceLimitsConfig {
                    default_memory_mb: 512,
                    default_cpu_percent: 30.0,
                    max_memory_mb: 2048,
                    max_cpu_percent: 100.0,
                },
            ),
        };
        
        Self {
            grpc_host: "127.0.0.1".to_string(),
            grpc_port: 50060,
            max_services,
            resource_limits,
            health_monitoring: HealthMonitoringConfig {
                check_interval_ms: 5000,
                auto_restart: true,
                max_restart_attempts: 3,
                restart_backoff_ms: 1000,
            },
            service_loader: ServiceLoaderConfig {
                startup_timeout_ms: 5000,
                shutdown_timeout_ms: 1000,
                graceful_shutdown: true,
            },
        }
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.grpc_port == 0 {
            return Err(GladsheimError::ConfigurationError("grpc_port cannot be 0".to_string()));
        }
        
        if self.max_services == 0 {
            return Err(GladsheimError::ConfigurationError("max_services cannot be 0".to_string()));
        }
        
        if self.resource_limits.default_memory_mb > self.resource_limits.max_memory_mb {
            return Err(GladsheimError::ConfigurationError(
                "default_memory_mb cannot exceed max_memory_mb".to_string()
            ));
        }
        
        if self.resource_limits.default_cpu_percent < 0.0 || self.resource_limits.default_cpu_percent > 100.0 {
            return Err(GladsheimError::ConfigurationError(
                "default_cpu_percent must be between 0.0 and 100.0".to_string()
            ));
        }
        
        if self.health_monitoring.check_interval_ms == 0 {
            return Err(GladsheimError::ConfigurationError(
                "check_interval_ms cannot be 0".to_string()
            ));
        }
        
        Ok(())
    }
    
    pub fn from_json(json: &str) -> Result<Self> {
        let config: GladsheimConfig = serde_json::from_str(json)
            .map_err(|e| GladsheimError::ConfigurationError(format!("Failed to parse JSON: {}", e)))?;
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| GladsheimError::ConfigurationError(format!("Failed to serialize JSON: {}", e)))
    }
}

impl HealthMonitoringConfig {
    pub fn check_interval(&self) -> Duration {
        Duration::from_millis(self.check_interval_ms)
    }
    
    pub fn restart_backoff(&self) -> Duration {
        Duration::from_millis(self.restart_backoff_ms)
    }
}

impl ServiceLoaderConfig {
    pub fn startup_timeout(&self) -> Duration {
        Duration::from_millis(self.startup_timeout_ms)
    }
    
    pub fn shutdown_timeout(&self) -> Duration {
        Duration::from_millis(self.shutdown_timeout_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_defaults() {
        let midgard = GladsheimConfig::for_platform(Platform::Midgard);
        assert_eq!(midgard.max_services, 15);
        
        let alfheim = GladsheimConfig::for_platform(Platform::Alfheim);
        assert_eq!(alfheim.max_services, 5);
    }
    
    #[test]
    fn test_config_validation_success() {
        let config = GladsheimConfig::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_validation_failure() {
        let mut config = GladsheimConfig::default();
        config.grpc_port = 0;
        assert!(config.validate().is_err());
    }
}
