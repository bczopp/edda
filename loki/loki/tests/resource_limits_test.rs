//! Tests for Resource Limits

use loki::resources::limits::ResourceLimits;
use loki::resources::limits::ResourceLimitType;

#[test]
fn test_resource_limits_default() {
    let limits = ResourceLimits::default();
    
    assert_eq!(limits.max_memory_mb, 10);
    assert_eq!(limits.max_execution_time_ms, 5000);
    assert_eq!(limits.max_cpu_percent, 50);
    assert_eq!(limits.max_disk_mb, 0); // No disk limit by default
}

#[test]
fn test_resource_limits_custom() {
    let limits = ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 10000,
        max_cpu_percent: 80,
        max_disk_mb: 100,
    };
    
    assert_eq!(limits.max_memory_mb, 20);
    assert_eq!(limits.max_execution_time_ms, 10000);
    assert_eq!(limits.max_cpu_percent, 80);
    assert_eq!(limits.max_disk_mb, 100);
}

#[test]
fn test_resource_limits_validate_valid() {
    let limits = ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 10000,
        max_cpu_percent: 80,
        max_disk_mb: 100,
    };
    
    assert!(limits.validate().is_ok());
}

#[test]
fn test_resource_limits_validate_invalid_memory() {
    let limits = ResourceLimits {
        max_memory_mb: 0,
        max_execution_time_ms: 10000,
        max_cpu_percent: 80,
        max_disk_mb: 100,
    };
    
    assert!(limits.validate().is_err());
}

#[test]
fn test_resource_limits_validate_invalid_cpu() {
    let limits = ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 10000,
        max_cpu_percent: 101,
        max_disk_mb: 100,
    };
    
    assert!(limits.validate().is_err());
}

#[test]
fn test_resource_limits_check_limit() {
    let limits = ResourceLimits {
        max_memory_mb: 20,
        max_execution_time_ms: 10000,
        max_cpu_percent: 80,
        max_disk_mb: 100,
    };
    
    // Check memory limit
    assert!(!limits.exceeds_limit(ResourceLimitType::Memory, 15));
    assert!(limits.exceeds_limit(ResourceLimitType::Memory, 25));
    
    // Check CPU limit
    assert!(!limits.exceeds_limit(ResourceLimitType::Cpu, 75));
    assert!(limits.exceeds_limit(ResourceLimitType::Cpu, 85));
    
    // Check execution time limit
    assert!(!limits.exceeds_limit(ResourceLimitType::ExecutionTime, 9000));
    assert!(limits.exceeds_limit(ResourceLimitType::ExecutionTime, 11000));
}
