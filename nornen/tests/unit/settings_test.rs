#[cfg(test)]
mod tests {
    use nornen::utils::config::{NornenSettings, SettingsManager};
    use std::path::PathBuf;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_settings_default() {
        let settings = NornenSettings::default();
        assert_eq!(settings.grpc_port, 50060);
        assert_eq!(settings.database.url, "postgres://localhost/nornen");
    }

    #[tokio::test]
    async fn test_settings_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        let settings = NornenSettings {
            grpc_port: 60000,
            database: nornen::utils::config::DatabaseConfig {
                url: "postgres://test:5432/nornen".to_string(),
                max_connections: 20,
                min_connections: 5,
            },
        };
        
        let json = serde_json::to_string_pretty(&settings).unwrap();
        fs::write(&config_path, json).unwrap();
        
        let manager = SettingsManager::new(config_path.clone());
        manager.load().await.unwrap();
        
        let loaded = manager.get().await;
        assert_eq!(loaded.grpc_port, 60000);
        assert_eq!(loaded.database.url, "postgres://test:5432/nornen");
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
        assert_eq!(loaded.grpc_port, 50060); // Default value
    }

    #[tokio::test]
    async fn test_settings_validation_invalid_port() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Invalid JSON with port 0
        let invalid_json = r#"{"grpc_port": 0, "database": {"url": "postgres://localhost/nornen", "max_connections": 10, "min_connections": 2}}"#;
        fs::write(&config_path, invalid_json).unwrap();
        
        let manager = SettingsManager::new(config_path);
        // Validation should fail
        let result = manager.load().await;
        assert!(result.is_err());
    }
}
