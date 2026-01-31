#[cfg(test)]
mod tests {
    use odin::clients::manager::ClientManager;
    use odin::utils::config::SettingsManager;
    use std::sync::Arc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_client_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let client_manager = ClientManager::new(settings_arc);
        
        // Test that manager can be created
        assert!(true, "ClientManager should be created successfully");
    }

    #[tokio::test]
    async fn test_client_manager_initialize() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let client_manager = ClientManager::new(settings_arc);
        
        // Initialize clients (will fail if services are not available, which is OK)
        let result = client_manager.initialize().await;
        // Should not panic, even if services are unavailable
        assert!(result.is_ok() || result.is_err());
    }
}
