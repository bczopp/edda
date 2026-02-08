//! Resource Enforcer for Byggvir

use crate::byggvir::limits::ResourceLimits;
use crate::byggvir::limit_checker::ResourceLimitChecker;
use tracing::{warn, error};

#[derive(Debug, Clone, PartialEq)]
pub enum EnforcementAction {
    Ok,
    Warning { message: String },
    Critical { message: String, should_stop: bool },
}

pub struct ResourceEnforcer {
    checker: ResourceLimitChecker,
    warning_threshold_percent: f32,
    critical_threshold_percent: f32,
}

impl ResourceEnforcer {
    pub fn new() -> Self {
        Self {
            checker: ResourceLimitChecker::new(),
            warning_threshold_percent: 80.0,
            critical_threshold_percent: 100.0,
        }
    }
    
    pub async fn check_and_enforce(
        &self,
        memory_bytes: u64,
        cpu_percent: f32,
        limits: &ResourceLimits,
    ) -> EnforcementAction {
        let memory_mb = memory_bytes / (1024 * 1024);
        let memory_exceeded = memory_mb > limits.max_memory_mb;
        let cpu_exceeded = cpu_percent > limits.max_cpu_percent;

        // Check critical (over limit) first
        if memory_exceeded || cpu_exceeded {
            let message = format!(
                "Resource limit exceeded: Memory {} MB > {} MB, CPU {}% > {}%",
                memory_mb, limits.max_memory_mb,
                cpu_percent, limits.max_cpu_percent
            );
            error!("{}", message);
            return EnforcementAction::Critical {
                message,
                should_stop: true,
            };
        }

        // Then check warning threshold (e.g. 80%)
        let (memory_warning, cpu_warning) = self.checker.check_warning_threshold(
            memory_bytes,
            cpu_percent,
            limits,
            self.warning_threshold_percent,
        );
        if memory_warning || cpu_warning {
            let message = format!(
                "Resource warning: Memory {}%, CPU {}%",
                (memory_bytes * 100 / (limits.max_memory_mb * 1024 * 1024)),
                (cpu_percent * 100.0 / limits.max_cpu_percent)
            );
            warn!("{}", message);
            return EnforcementAction::Warning { message };
        }

        EnforcementAction::Ok
    }
    
    pub fn set_warning_threshold(&mut self, percent: f32) {
        self.warning_threshold_percent = percent;
    }
    
    pub fn set_critical_threshold(&mut self, percent: f32) {
        self.critical_threshold_percent = percent;
    }
}

impl Default for ResourceEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_enforcement_ok() {
        let enforcer = ResourceEnforcer::new();
        let limits = ResourceLimits::new(512, 50.0);
        
        let action = enforcer.check_and_enforce(256 * 1024 * 1024, 25.0, &limits).await;
        assert_eq!(action, EnforcementAction::Ok);
    }
    
    #[tokio::test]
    async fn test_enforcement_warning() {
        let enforcer = ResourceEnforcer::new();
        let limits = ResourceLimits::new(512, 50.0);
        
        // 80% of limit
        let action = enforcer.check_and_enforce(410 * 1024 * 1024, 40.0, &limits).await;
        assert!(matches!(action, EnforcementAction::Warning { .. }));
    }
    
    #[tokio::test]
    async fn test_enforcement_critical() {
        let enforcer = ResourceEnforcer::new();
        let limits = ResourceLimits::new(512, 50.0);
        
        // Exceeds limit
        let action = enforcer.check_and_enforce(600 * 1024 * 1024, 60.0, &limits).await;
        assert!(matches!(action, EnforcementAction::Critical { should_stop: true, .. }));
    }
}
