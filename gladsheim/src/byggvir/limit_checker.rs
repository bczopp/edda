//! Resource Limit Checker for Byggvir

use crate::utils::{GladsheimError, Result};
use crate::byggvir::limits::ResourceLimits;
use tracing::warn;

pub struct ResourceLimitChecker;

impl ResourceLimitChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub fn check_memory_limit(&self, memory_bytes: u64, limits: &ResourceLimits) -> Result<()> {
        let memory_mb = memory_bytes / (1024 * 1024);
        
        if memory_mb > limits.max_memory_mb {
            warn!(
                "Memory limit exceeded: {} MB > {} MB",
                memory_mb, limits.max_memory_mb
            );
            return Err(GladsheimError::ResourceError(format!(
                "Memory limit exceeded: {} MB > {} MB",
                memory_mb, limits.max_memory_mb
            )));
        }
        
        Ok(())
    }
    
    pub fn check_cpu_limit(&self, cpu_percent: f32, limits: &ResourceLimits) -> Result<()> {
        if cpu_percent > limits.max_cpu_percent {
            warn!(
                "CPU limit exceeded: {}% > {}%",
                cpu_percent, limits.max_cpu_percent
            );
            return Err(GladsheimError::ResourceError(format!(
                "CPU limit exceeded: {}% > {}%",
                cpu_percent, limits.max_cpu_percent
            )));
        }
        
        Ok(())
    }
    
    pub fn check_limits(&self, memory_bytes: u64, cpu_percent: f32, limits: &ResourceLimits) -> Result<()> {
        self.check_memory_limit(memory_bytes, limits)?;
        self.check_cpu_limit(cpu_percent, limits)?;
        Ok(())
    }
    
    pub fn check_warning_threshold(
        &self,
        memory_bytes: u64,
        cpu_percent: f32,
        limits: &ResourceLimits,
        warning_percent: f32,
    ) -> (bool, bool) {
        let memory_mb = memory_bytes / (1024 * 1024);
        let memory_warning_threshold = (limits.max_memory_mb as f32 * warning_percent / 100.0) as u64;
        let cpu_warning_threshold = limits.max_cpu_percent * warning_percent / 100.0;
        
        let memory_warning = memory_mb >= memory_warning_threshold;
        let cpu_warning = cpu_percent >= cpu_warning_threshold;
        
        (memory_warning, cpu_warning)
    }
}

impl Default for ResourceLimitChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_limit_check() {
        let checker = ResourceLimitChecker::new();
        let limits = ResourceLimits::new(512, 50.0);
        
        // Within limit
        assert!(checker.check_memory_limit(256 * 1024 * 1024, &limits).is_ok());
        
        // Exceeds limit
        assert!(checker.check_memory_limit(1024 * 1024 * 1024, &limits).is_err());
    }
    
    #[test]
    fn test_cpu_limit_check() {
        let checker = ResourceLimitChecker::new();
        let limits = ResourceLimits::new(512, 50.0);
        
        // Within limit
        assert!(checker.check_cpu_limit(25.0, &limits).is_ok());
        
        // Exceeds limit
        assert!(checker.check_cpu_limit(75.0, &limits).is_err());
    }
    
    #[test]
    fn test_warning_threshold() {
        let checker = ResourceLimitChecker::new();
        let limits = ResourceLimits::new(512, 50.0);
        
        // 80% threshold
        let (mem_warn, cpu_warn) = checker.check_warning_threshold(
            410 * 1024 * 1024, // ~80% of 512MB
            40.0, // 80% of 50%
            &limits,
            80.0,
        );
        
        assert!(mem_warn);
        assert!(cpu_warn);
    }
}
