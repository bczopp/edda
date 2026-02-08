//! Resource Limits for script execution

use shared::{LokiError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceLimitType {
    Memory,
    Cpu,
    ExecutionTime,
    Disk,
}

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_mb: u32,
    pub max_execution_time_ms: u32,
    pub max_cpu_percent: u32,
    pub max_disk_mb: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 10,
            max_execution_time_ms: 5000,
            max_cpu_percent: 50,
            max_disk_mb: 0, // 0 means no limit
        }
    }
}

impl ResourceLimits {
    pub fn validate(&self) -> Result<()> {
        if self.max_memory_mb == 0 {
            return Err(LokiError::ConfigurationError(
                "max_memory_mb cannot be 0".to_string()
            ));
        }
        
        if self.max_execution_time_ms == 0 {
            return Err(LokiError::ConfigurationError(
                "max_execution_time_ms cannot be 0".to_string()
            ));
        }
        
        if self.max_cpu_percent > 100 {
            return Err(LokiError::ConfigurationError(
                "max_cpu_percent cannot exceed 100".to_string()
            ));
        }
        
        Ok(())
    }
    
    pub fn exceeds_limit(&self, limit_type: ResourceLimitType, value: u32) -> bool {
        match limit_type {
            ResourceLimitType::Memory => value > self.max_memory_mb,
            ResourceLimitType::Cpu => value > self.max_cpu_percent,
            ResourceLimitType::ExecutionTime => value > self.max_execution_time_ms,
            ResourceLimitType::Disk => {
                if self.max_disk_mb == 0 {
                    false // No limit
                } else {
                    value > self.max_disk_mb
                }
            }
        }
    }
    
    pub fn get_limit(&self, limit_type: ResourceLimitType) -> u32 {
        match limit_type {
            ResourceLimitType::Memory => self.max_memory_mb,
            ResourceLimitType::Cpu => self.max_cpu_percent,
            ResourceLimitType::ExecutionTime => self.max_execution_time_ms,
            ResourceLimitType::Disk => self.max_disk_mb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_limit() {
        let limits = ResourceLimits {
            max_memory_mb: 20,
            max_execution_time_ms: 10000,
            max_cpu_percent: 80,
            max_disk_mb: 100,
        };
        
        assert_eq!(limits.get_limit(ResourceLimitType::Memory), 20);
        assert_eq!(limits.get_limit(ResourceLimitType::Cpu), 80);
        assert_eq!(limits.get_limit(ResourceLimitType::ExecutionTime), 10000);
        assert_eq!(limits.get_limit(ResourceLimitType::Disk), 100);
    }
}
