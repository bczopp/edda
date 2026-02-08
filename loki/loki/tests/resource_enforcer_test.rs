//! Tests for Resource Enforcer

use loki::resources::enforcer::ResourceEnforcer;
use loki::resources::limits::ResourceLimits;
use loki::resources::monitor::ResourceMonitor;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_resource_enforcer_new() {
    let limits = Arc::new(RwLock::new(ResourceLimits::default()));
    let monitor = Arc::new(ResourceMonitor::new(limits).unwrap());
    
    let enforcer = ResourceEnforcer::new(monitor);
    assert!(enforcer.is_ok());
}

#[tokio::test]
async fn test_resource_enforcer_enforce_memory_limit() {
    let limits = Arc::new(RwLock::new(ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 5000,
        max_cpu_percent: 50,
        max_disk_mb: 0,
    }));
    let monitor = Arc::new(ResourceMonitor::new(limits).unwrap());
    let enforcer = ResourceEnforcer::new(monitor).unwrap();
    
    // Check within limit
    let result = enforcer.enforce_memory_limit(15).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
    
    // Check exceeds limit
    let result = enforcer.enforce_memory_limit(25).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should trigger enforcement
}

#[tokio::test]
async fn test_resource_enforcer_enforce_cpu_limit() {
    let limits = Arc::new(RwLock::new(ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 5000,
        max_cpu_percent: 50,
        max_disk_mb: 0,
    }));
    let monitor = Arc::new(ResourceMonitor::new(limits).unwrap());
    let enforcer = ResourceEnforcer::new(monitor).unwrap();
    
    // Check within limit
    let result = enforcer.enforce_cpu_limit(45).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
    
    // Check exceeds limit
    let result = enforcer.enforce_cpu_limit(55).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should trigger enforcement
}

#[tokio::test]
async fn test_resource_enforcer_enforce_execution_time() {
    let limits = Arc::new(RwLock::new(ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 5000,
        max_cpu_percent: 50,
        max_disk_mb: 0,
    }));
    let monitor = Arc::new(ResourceMonitor::new(limits).unwrap());
    let enforcer = ResourceEnforcer::new(monitor).unwrap();
    
    // Check within limit
    let result = enforcer.enforce_execution_time(4000).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
    
    // Check exceeds limit
    let result = enforcer.enforce_execution_time(6000).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should trigger enforcement
}
