//! Tests for Gladsheim Configuration

use gladsheim::utils::config::{GladsheimConfig, ResourceLimitsConfig, HealthMonitoringConfig, ServiceLoaderConfig, Platform};

#[test]
fn test_gladsheim_config_default() {
    let config = GladsheimConfig::default();
    assert_eq!(config.grpc_port, 50060);
    assert_eq!(config.max_services, 25);
}

#[test]
fn test_gladsheim_config_platform_specific() {
    let midgard = GladsheimConfig::for_platform(Platform::Midgard);
    assert_eq!(midgard.max_services, 15);
    assert_eq!(midgard.resource_limits.default_memory_mb, 512);
    
    let alfheim = GladsheimConfig::for_platform(Platform::Alfheim);
    assert_eq!(alfheim.max_services, 5);
    assert_eq!(alfheim.resource_limits.default_memory_mb, 256);
    
    let asgard = GladsheimConfig::for_platform(Platform::Asgard);
    assert_eq!(asgard.max_services, 25);
    assert_eq!(asgard.resource_limits.default_memory_mb, 2048);
    
    let ragnarok = GladsheimConfig::for_platform(Platform::Ragnarok);
    assert_eq!(ragnarok.max_services, 8);
    assert_eq!(ragnarok.resource_limits.default_memory_mb, 512);
}

#[test]
fn test_resource_limits_config() {
    let limits = ResourceLimitsConfig {
        default_memory_mb: 512,
        default_cpu_percent: 25.0,
        max_memory_mb: 2048,
        max_cpu_percent: 100.0,
    };
    
    assert_eq!(limits.default_memory_mb, 512);
    assert_eq!(limits.default_cpu_percent, 25.0);
}

#[test]
fn test_health_monitoring_config() {
    let health = HealthMonitoringConfig {
        check_interval_ms: 5000,
        auto_restart: true,
        max_restart_attempts: 3,
        restart_backoff_ms: 1000,
    };
    
    assert_eq!(health.check_interval_ms, 5000);
    assert!(health.auto_restart);
    assert_eq!(health.max_restart_attempts, 3);
}

#[test]
fn test_service_loader_config() {
    let loader = ServiceLoaderConfig {
        startup_timeout_ms: 5000,
        shutdown_timeout_ms: 1000,
        graceful_shutdown: true,
    };
    
    assert_eq!(loader.startup_timeout_ms, 5000);
    assert_eq!(loader.shutdown_timeout_ms, 1000);
    assert!(loader.graceful_shutdown);
}

#[test]
fn test_config_validation() {
    let mut config = GladsheimConfig::default();
    
    // Valid config
    assert!(config.validate().is_ok());
    
    // Invalid port
    config.grpc_port = 0;
    assert!(config.validate().is_err());
    
    // Invalid max_services
    config.grpc_port = 50060;
    config.max_services = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_config_from_json() {
    let json = r#"
    {
        "grpc_port": 50061,
        "max_services": 10,
        "resource_limits": {
            "default_memory_mb": 1024,
            "default_cpu_percent": 50.0
        }
    }
    "#;
    
    let config: GladsheimConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.grpc_port, 50061);
    assert_eq!(config.max_services, 10);
    assert_eq!(config.resource_limits.default_memory_mb, 1024);
}
