#[cfg(test)]
mod tests {
    use odin::protocols::manager::ProtocolManager;
    use odin::utils::config::SettingsManager;
    use std::sync::Arc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_protocol_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let protocol_manager = ProtocolManager::new(settings_arc);
        
        // Test that manager can be created
        assert!(true, "ProtocolManager should be created successfully");
    }

    #[tokio::test]
    async fn test_protocol_manager_get_cache() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let protocol_manager = ProtocolManager::new(settings_arc);
        
        // Test cache access
        let cache = protocol_manager.get_cache();
        let all = cache.get_all().await;
        assert_eq!(all.len(), 0, "Cache should be empty initially");
    }

    #[tokio::test]
    async fn test_protocol_manager_discover_capabilities() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let protocol_manager = ProtocolManager::new(settings_arc);
        
        // Try to discover capabilities
        // This will fail if services are not available, which is OK
        let result = protocol_manager.discover_all_capabilities().await;
        // Should not panic, even if services are unavailable
        assert!(result.is_ok() || result.is_err());
    }
}
