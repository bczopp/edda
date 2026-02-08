//! Resource Limits for Byggvir

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: u64,
    pub max_cpu_percent: f32,
}

impl ResourceLimits {
    pub fn new(max_memory_mb: u64, max_cpu_percent: f32) -> Self {
        Self {
            max_memory_mb,
            max_cpu_percent,
        }
    }
    
    pub fn default_desktop() -> Self {
        Self::new(512, 25.0)
    }
    
    pub fn default_mobile() -> Self {
        Self::new(256, 25.0)
    }
    
    pub fn default_server() -> Self {
        Self::new(2048, 75.0)
    }
    
    pub fn default_terminal() -> Self {
        Self::new(512, 30.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resource_limits_creation() {
        let limits = ResourceLimits::new(1024, 50.0);
        assert_eq!(limits.max_memory_mb, 1024);
        assert_eq!(limits.max_cpu_percent, 50.0);
    }
    
    #[test]
    fn test_platform_defaults() {
        let desktop = ResourceLimits::default_desktop();
        assert_eq!(desktop.max_memory_mb, 512);
        
        let mobile = ResourceLimits::default_mobile();
        assert_eq!(mobile.max_memory_mb, 256);
        
        let server = ResourceLimits::default_server();
        assert_eq!(server.max_memory_mb, 2048);
        
        let terminal = ResourceLimits::default_terminal();
        assert_eq!(terminal.max_memory_mb, 512);
    }
}
