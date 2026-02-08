//! Tests for Loki configuration system

use loki::utils::config::{LokiConfig, ChildrenConfig, ResourceLimitsConfig};
use loki::utils::{LokiError, Result};
use loki::utils::config_loader;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_loki_config_default() {
    let config = LokiConfig::default();
    
    assert_eq!(config.grpc_port, 50070);
    assert_eq!(config.script_storage_path, "./scripts");
    assert_eq!(config.resource_limits.max_memory_mb, 10);
    assert_eq!(config.resource_limits.max_execution_time_ms, 5000);
    assert_eq!(config.resource_limits.max_cpu_percent, 50);
}

#[test]
fn test_loki_config_validation_valid() {
    let config = LokiConfig {
        grpc_port: 50070,
        script_storage_path: "./scripts".to_string(),
        resource_limits: ResourceLimitsConfig {
            max_memory_mb: 20,
            max_execution_time_ms: 10000,
            max_cpu_percent: 80,
        },
        children_config: ChildrenConfig::default(),
    };
    
    assert!(config.validate().is_ok());
}

#[test]
fn test_loki_config_validation_invalid_port() {
    let config = LokiConfig {
        grpc_port: 0,
        script_storage_path: "./scripts".to_string(),
        resource_limits: ResourceLimitsConfig::default(),
        children_config: ChildrenConfig::default(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
    match result.unwrap_err() {
        LokiError::ConfigurationError(msg) => {
            assert!(msg.contains("grpc_port"));
        }
        _ => panic!("Expected ConfigurationError"),
    }
}

#[test]
fn test_loki_config_validation_invalid_memory() {
    let config = LokiConfig {
        grpc_port: 50070,
        script_storage_path: "./scripts".to_string(),
        resource_limits: ResourceLimitsConfig {
            max_memory_mb: 0,
            max_execution_time_ms: 5000,
            max_cpu_percent: 50,
        },
        children_config: ChildrenConfig::default(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_loki_config_validation_invalid_cpu() {
    let config = LokiConfig {
        grpc_port: 50070,
        script_storage_path: "./scripts".to_string(),
        resource_limits: ResourceLimitsConfig {
            max_memory_mb: 10,
            max_execution_time_ms: 5000,
            max_cpu_percent: 101,
        },
        children_config: ChildrenConfig::default(),
    };
    
    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_loki_config_from_json() {
    let json = r#"{
        "grpc_port": 50071,
        "script_storage_path": "/custom/scripts",
        "resource_limits": {
            "max_memory_mb": 30,
            "max_execution_time_ms": 15000,
            "max_cpu_percent": 75
        },
        "children_config": {
            "fenrir": {
                "enabled": true,
                "address": "127.0.0.1:50071"
            },
            "jormungandr": {
                "enabled": true,
                "address": "127.0.0.1:50072"
            },
            "hel": {
                "enabled": true,
                "address": "127.0.0.1:50073"
            }
        }
    }"#;
    
    let config = LokiConfig::from_json(json).unwrap();
    
    assert_eq!(config.grpc_port, 50071);
    assert_eq!(config.script_storage_path, "/custom/scripts");
    assert_eq!(config.resource_limits.max_memory_mb, 30);
    assert_eq!(config.resource_limits.max_execution_time_ms, 15000);
    assert_eq!(config.resource_limits.max_cpu_percent, 75);
    assert!(config.children_config.fenrir.enabled);
}

#[test]
fn test_loki_config_to_json() {
    let config = LokiConfig {
        grpc_port: 50072,
        script_storage_path: "./test_scripts".to_string(),
        resource_limits: ResourceLimitsConfig {
            max_memory_mb: 25,
            max_execution_time_ms: 12000,
            max_cpu_percent: 60,
        },
        children_config: ChildrenConfig::default(),
    };
    
    let json = config.to_json().unwrap();
    let parsed = LokiConfig::from_json(&json).unwrap();
    
    assert_eq!(parsed.grpc_port, 50072);
    assert_eq!(parsed.script_storage_path, "./test_scripts");
    assert_eq!(parsed.resource_limits.max_memory_mb, 25);
}

#[tokio::test]
async fn test_config_loader_new_with_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("loki.json");
    
    let config_json = r#"{
        "grpc_port": 50073,
        "script_storage_path": "./scripts",
        "resource_limits": {
            "max_memory_mb": 20,
            "max_execution_time_ms": 10000,
            "max_cpu_percent": 80
        }
    }"#;
    
    fs::write(&config_path, config_json).unwrap();
    
    let loader = config_loader::ConfigLoader::new(config_path.clone());
    assert!(loader.is_ok());
    
    let loader = loader.unwrap();
    let config = loader.get_config().await;
    assert_eq!(config.grpc_port, 50073);
}

#[tokio::test]
async fn test_config_loader_new_without_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("nonexistent.json");
    
    let loader = config_loader::ConfigLoader::new(config_path.clone());
    assert!(loader.is_ok());
    
    let loader = loader.unwrap();
    let config = loader.get_config().await;
    assert_eq!(config.grpc_port, 50070); // Default
}

#[tokio::test]
async fn test_config_loader_load() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("loki.json");
    
    let config_json = r#"{
        "grpc_port": 50074,
        "script_storage_path": "./scripts",
        "resource_limits": {
            "max_memory_mb": 15,
            "max_execution_time_ms": 8000,
            "max_cpu_percent": 70
        }
    }"#;
    
    fs::write(&config_path, config_json).unwrap();
    
    let mut loader = config_loader::ConfigLoader::new(config_path.clone()).unwrap();
    loader.load().await.unwrap();
    
    let config = loader.get_config().await;
    assert_eq!(config.grpc_port, 50074);
    assert_eq!(config.resource_limits.max_memory_mb, 15);
}
