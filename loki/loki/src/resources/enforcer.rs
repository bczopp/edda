//! Resource Enforcer for enforcing resource limits

use super::monitor::ResourceMonitor;
use shared::{LokiError, Result};
use std::sync::Arc;
use tracing::{info, warn, error};

/// Enforcement action to take when resource limits are exceeded
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementAction {
    /// No action needed - within limits
    None,
    /// Warning - approaching limit
    Warning,
    /// Critical - limit exceeded, script should be terminated
    Critical,
}

/// Resource Enforcer enforces resource limits and takes actions when exceeded
pub struct ResourceEnforcer {
    monitor: Arc<ResourceMonitor>,
}

impl ResourceEnforcer {
    pub fn new(monitor: Arc<ResourceMonitor>) -> Result<Self> {
        Ok(Self {
            monitor,
        })
    }
    
    /// Enforce memory limit - returns true if limit exceeded
    pub async fn enforce_memory_limit(&self, memory_mb: u32) -> Result<bool> {
        let exceeds = self.monitor.check_memory_usage(memory_mb).await?;
        
        if exceeds {
            warn!("Memory limit exceeded: {} MB", memory_mb);
            // TODO: Implement actual enforcement (e.g., terminate script)
        }
        
        Ok(exceeds)
    }
    
    /// Enforce CPU limit - returns true if limit exceeded
    pub async fn enforce_cpu_limit(&self, cpu_percent: u32) -> Result<bool> {
        let exceeds = self.monitor.check_cpu_usage(cpu_percent).await?;
        
        if exceeds {
            warn!("CPU limit exceeded: {}%", cpu_percent);
            // TODO: Implement actual enforcement (e.g., throttle or terminate script)
        }
        
        Ok(exceeds)
    }
    
    /// Enforce execution time limit - returns true if limit exceeded
    pub async fn enforce_execution_time(&self, execution_time_ms: u32) -> Result<bool> {
        let exceeds = self.monitor.check_execution_time(execution_time_ms).await?;
        
        if exceeds {
            warn!("Execution time limit exceeded: {} ms", execution_time_ms);
            // TODO: Implement actual enforcement (e.g., terminate script)
        }
        
        Ok(exceeds)
    }
    
    /// Enforce disk limit - returns true if limit exceeded
    pub async fn enforce_disk_limit(&self, disk_mb: u32) -> Result<bool> {
        let exceeds = self.monitor.check_disk_usage(disk_mb).await?;
        
        if exceeds {
            warn!("Disk limit exceeded: {} MB", disk_mb);
            // TODO: Implement actual enforcement (e.g., prevent writes or terminate script)
        }
        
        Ok(exceeds)
    }
    
    /// Check all resource limits and determine enforcement action
    pub async fn check_and_enforce(
        &self,
        memory_mb: u32,
        cpu_percent: u32,
        execution_time_ms: u32,
        disk_mb: u32,
    ) -> Result<EnforcementAction> {
        let memory_exceeds = self.enforce_memory_limit(memory_mb).await?;
        let cpu_exceeds = self.enforce_cpu_limit(cpu_percent).await?;
        let time_exceeds = self.enforce_execution_time(execution_time_ms).await?;
        let disk_exceeds = self.enforce_disk_limit(disk_mb).await?;
        
        if memory_exceeds || cpu_exceeds || time_exceeds || disk_exceeds {
            error!("Resource limits exceeded - critical action required");
            Ok(EnforcementAction::Critical)
        } else {
            // Check if approaching limits (warning threshold)
            let limits = self.monitor.get_limits().await;
            
            let memory_warning = memory_mb as f32 > limits.max_memory_mb as f32 * 0.8;
            let cpu_warning = cpu_percent as f32 > limits.max_cpu_percent as f32 * 0.8;
            let time_warning = execution_time_ms as f32 > limits.max_execution_time_ms as f32 * 0.8;
            
            if memory_warning || cpu_warning || time_warning {
                warn!("Approaching resource limits - warning");
                Ok(EnforcementAction::Warning)
            } else {
                Ok(EnforcementAction::None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_enforcement_action_none() {
        let limits = Arc::new(tokio::sync::RwLock::new(
            super::super::limits::ResourceLimits {
                max_memory_mb: 20,
                max_execution_time_ms: 5000,
                max_cpu_percent: 50,
                max_disk_mb: 0,
            }
        ));
        let monitor = Arc::new(ResourceMonitor::new(limits).unwrap());
        let enforcer = ResourceEnforcer::new(monitor).unwrap();
        
        let action = enforcer.check_and_enforce(5, 20, 2000, 0).await.unwrap();
        assert_eq!(action, EnforcementAction::None);
    }
    
    #[tokio::test]
    async fn test_enforcement_action_warning() {
        let limits = Arc::new(tokio::sync::RwLock::new(
            super::super::limits::ResourceLimits {
                max_memory_mb: 20,
                max_execution_time_ms: 5000,
                max_cpu_percent: 50,
                max_disk_mb: 0,
            }
        ));
        let monitor = Arc::new(ResourceMonitor::new(limits).unwrap());
        let enforcer = ResourceEnforcer::new(monitor).unwrap();
        
        // 17 MB is > 80% of 20 MB
        let action = enforcer.check_and_enforce(17, 20, 2000, 0).await.unwrap();
        assert_eq!(action, EnforcementAction::Warning);
    }
    
    #[tokio::test]
    async fn test_enforcement_action_critical() {
        let limits = Arc::new(tokio::sync::RwLock::new(
            super::super::limits::ResourceLimits {
                max_memory_mb: 20,
                max_execution_time_ms: 5000,
                max_cpu_percent: 50,
                max_disk_mb: 0,
            }
        ));
        let monitor = Arc::new(ResourceMonitor::new(limits).unwrap());
        let enforcer = ResourceEnforcer::new(monitor).unwrap();
        
        // 25 MB exceeds 20 MB limit
        let action = enforcer.check_and_enforce(25, 20, 2000, 0).await.unwrap();
        assert_eq!(action, EnforcementAction::Critical);
    }
}
