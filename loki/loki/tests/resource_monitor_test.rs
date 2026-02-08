//! Tests for Resource Monitor

use loki::resources::monitor::ResourceMonitor;
use loki::resources::limits::ResourceLimits;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_resource_monitor_new() {
    let limits = Arc::new(RwLock::new(ResourceLimits::default()));
    let monitor = ResourceMonitor::new(limits);
    assert!(monitor.is_ok());
}

#[tokio::test]
async fn test_resource_monitor_check_memory() {
    let limits = Arc::new(RwLock::new(ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 5000,
        max_cpu_percent: 50,
        max_disk_mb: 0,
    }));
    
    let monitor = ResourceMonitor::new(limits).unwrap();
    
    // Check memory usage (mock - actual implementation would use sysinfo)
    let result = monitor.check_memory_usage(15).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
    
    let result = monitor.check_memory_usage(25).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Exceeds limit
}

#[tokio::test]
async fn test_resource_monitor_check_cpu() {
    let limits = Arc::new(RwLock::new(ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 5000,
        max_cpu_percent: 50,
        max_disk_mb: 0,
    }));
    
    let monitor = ResourceMonitor::new(limits).unwrap();
    
    let result = monitor.check_cpu_usage(45).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
    
    let result = monitor.check_cpu_usage(55).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Exceeds limit
}

#[tokio::test]
async fn test_resource_monitor_check_execution_time() {
    let limits = Arc::new(RwLock::new(ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 5000,
        max_cpu_percent: 50,
        max_disk_mb: 0,
    }));
    
    let monitor = ResourceMonitor::new(limits).unwrap();
    
    let result = monitor.check_execution_time(4000).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
    
    let result = monitor.check_execution_time(6000).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Exceeds limit
}
