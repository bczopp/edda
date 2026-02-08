//! Resource Monitor for tracking script resource usage

use super::limits::ResourceLimits;
use shared::{LokiError, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Resource Monitor tracks and checks resource usage against limits
pub struct ResourceMonitor {
    limits: Arc<RwLock<ResourceLimits>>,
}

impl ResourceMonitor {
    pub fn new(limits: Arc<RwLock<ResourceLimits>>) -> Result<Self> {
        Ok(Self {
            limits,
        })
    }
    
    /// Check if memory usage exceeds limit
    pub async fn check_memory_usage(&self, memory_mb: u32) -> Result<bool> {
        let limits = self.limits.read().await;
        let exceeds = limits.exceeds_limit(super::limits::ResourceLimitType::Memory, memory_mb);
        
        if exceeds {
            warn!("Memory usage {} MB exceeds limit {} MB", memory_mb, limits.max_memory_mb);
        }
        
        Ok(exceeds)
    }
    
    /// Check if CPU usage exceeds limit
    pub async fn check_cpu_usage(&self, cpu_percent: u32) -> Result<bool> {
        let limits = self.limits.read().await;
        let exceeds = limits.exceeds_limit(super::limits::ResourceLimitType::Cpu, cpu_percent);
        
        if exceeds {
            warn!("CPU usage {}% exceeds limit {}%", cpu_percent, limits.max_cpu_percent);
        }
        
        Ok(exceeds)
    }
    
    /// Check if execution time exceeds limit
    pub async fn check_execution_time(&self, execution_time_ms: u32) -> Result<bool> {
        let limits = self.limits.read().await;
        let exceeds = limits.exceeds_limit(
            super::limits::ResourceLimitType::ExecutionTime,
            execution_time_ms
        );
        
        if exceeds {
            warn!("Execution time {} ms exceeds limit {} ms", 
                execution_time_ms, limits.max_execution_time_ms);
        }
        
        Ok(exceeds)
    }
    
    /// Check if disk usage exceeds limit
    pub async fn check_disk_usage(&self, disk_mb: u32) -> Result<bool> {
        let limits = self.limits.read().await;
        let exceeds = limits.exceeds_limit(super::limits::ResourceLimitType::Disk, disk_mb);
        
        if exceeds {
            warn!("Disk usage {} MB exceeds limit {} MB", disk_mb, limits.max_disk_mb);
        }
        
        Ok(exceeds)
    }
    
    /// Check all resource limits
    pub async fn check_all_limits(
        &self,
        memory_mb: u32,
        cpu_percent: u32,
        execution_time_ms: u32,
        disk_mb: u32,
    ) -> Result<bool> {
        let memory_exceeds = self.check_memory_usage(memory_mb).await?;
        let cpu_exceeds = self.check_cpu_usage(cpu_percent).await?;
        let time_exceeds = self.check_execution_time(execution_time_ms).await?;
        let disk_exceeds = self.check_disk_usage(disk_mb).await?;
        
        Ok(memory_exceeds || cpu_exceeds || time_exceeds || disk_exceeds)
    }
    
    /// Update resource limits
    pub async fn update_limits(&self, new_limits: ResourceLimits) -> Result<()> {
        new_limits.validate()?;
        
        let mut limits = self.limits.write().await;
        *limits = new_limits;
        info!("Resource limits updated");
        
        Ok(())
    }
    
    /// Get current resource limits
    pub async fn get_limits(&self) -> ResourceLimits {
        let limits = self.limits.read().await;
        limits.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_resource_monitor_update_limits() {
        let limits = Arc::new(RwLock::new(ResourceLimits::default()));
        let monitor = ResourceMonitor::new(limits.clone()).unwrap();
        
        let new_limits = ResourceLimits {
            max_memory_mb: 30,
            max_execution_time_ms: 15000,
            max_cpu_percent: 75,
            max_disk_mb: 200,
        };
        
        monitor.update_limits(new_limits).await.unwrap();
        
        let current_limits = monitor.get_limits().await;
        assert_eq!(current_limits.max_memory_mb, 30);
        assert_eq!(current_limits.max_cpu_percent, 75);
    }
}
