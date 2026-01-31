#[cfg(test)]
mod tests {
    use thor::utils::config::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_load_default_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        let manager = SettingsManager::new(config_path.clone());
        
        // Load should create default config
        manager.load().await.unwrap();
        
        let settings = manager.get().await;
        assert_eq!(settings.grpc_port, 50052);
        assert_eq!(settings.max_concurrent_actions, 100);
    }

    #[tokio::test]
    async fn test_load_custom_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Create custom config
        let custom_config = r#"
        {
            "grpc_port": 50053,
            "heimdall_url": "http://localhost:50051",
            "max_concurrent_actions": 200,
            "action_timeout_seconds": 600,
            "enable_sandboxing": true,
            "enable_audit_logging": false
        }
        "#;
        fs::write(&config_path, custom_config).unwrap();
        
        let manager = SettingsManager::new(config_path);
        manager.load().await.unwrap();
        
        let settings = manager.get().await;
        assert_eq!(settings.grpc_port, 50053);
        assert_eq!(settings.max_concurrent_actions, 200);
        assert_eq!(settings.enable_sandboxing, true);
    }
}
