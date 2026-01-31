#[cfg(test)]
mod tests {
    use mimir::utils::config::{MimirSettings, SettingsManager};
    use std::path::PathBuf;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_settings_default() {
        let settings = MimirSettings::default();
        assert_eq!(settings.grpc_port, 50059);
        assert_eq!(settings.database_url, "postgres://localhost/mimir");
        assert_eq!(settings.encryption_key_path, "keys/mimir.key");
    }

    #[tokio::test]
    async fn test_settings_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        let settings = MimirSettings {
            grpc_port: 60000,
            database_url: "postgres://test:5432/mimir".to_string(),
            encryption_key_path: "keys/test.key".to_string(),
        };
        
        let json = serde_json::to_string_pretty(&settings).unwrap();
        fs::write(&config_path, json).unwrap();
        
        let manager = SettingsManager::new(config_path.clone());
        manager.load().await.unwrap();
        
        let loaded = manager.get().await;
        assert_eq!(loaded.grpc_port, 60000);
        assert_eq!(loaded.database_url, "postgres://test:5432/mimir");
        assert_eq!(loaded.encryption_key_path, "keys/test.key");
    }

    #[tokio::test]
    async fn test_settings_create_default_if_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        assert!(!config_path.exists());
        
        let manager = SettingsManager::new(config_path.clone());
        manager.load().await.unwrap();
        
        assert!(config_path.exists());
        
        let loaded = manager.get().await;
        assert_eq!(loaded.grpc_port, 50059); // Default value
    }

    #[tokio::test]
    async fn test_settings_validation_invalid_port() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Invalid JSON with port 0
        let invalid_json = r#"{"grpc_port": 0, "database": {"url": "postgres://localhost/mimir", "max_connections": 10, "min_connections": 2}, "security": {"encryption_algorithm": "AES-256-GCM", "key_rotation_days": 90, "enable_audit_logging": true}, "data_retention": {"default_retention_days": 365, "enable_auto_deletion": false, "anonymize_on_deletion": true}, "encryption_key_path": "keys/mimir.key"}"#;
        fs::write(&config_path, invalid_json).unwrap();
        
        let manager = SettingsManager::new(config_path);
        // Validation should fail
        let result = manager.load().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_settings_hot_reload() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        let initial_settings = MimirSettings {
            grpc_port: 50059,
            database_url: "postgres://localhost/mimir".to_string(),
            encryption_key_path: "keys/mimir.key".to_string(),
        };
        
        let json = serde_json::to_string_pretty(&initial_settings).unwrap();
        fs::write(&config_path, json).unwrap();
        
        let manager = SettingsManager::new(config_path.clone());
        manager.load().await.unwrap();
        manager.start_hot_reload().unwrap();
        
        // Wait a bit for watcher to initialize
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Update settings
        let updated_settings = MimirSettings {
            grpc_port: 60000,
            database_url: "postgres://localhost/mimir".to_string(),
            encryption_key_path: "keys/mimir.key".to_string(),
        };
        
        let json = serde_json::to_string_pretty(&updated_settings).unwrap();
        fs::write(&config_path, json).unwrap();
        
        // Wait for hot reload
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        let loaded = manager.get().await;
        assert_eq!(loaded.grpc_port, 60000);
    }
}
