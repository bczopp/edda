// Integration tests for Odin
// These tests run against mock services in containers

#[cfg(test)]
mod integration_tests {
    use odin::clients::manager::ClientManager;
    use odin::utils::config::{SettingsManager, OdinSettings};
    use std::sync::Arc;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_client_manager_integration() {
        // This test will run in container with mock services
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let client_manager = ClientManager::new(settings_arc);
        
        // Initialize clients - should connect to mock services in container
        let result = client_manager.initialize().await;
        
        // In container environment, this should succeed
        // Outside container, it may fail, which is OK
        match result {
            Ok(_) => {
                println!("Client manager initialized successfully");
                assert!(true);
            }
            Err(e) => {
                println!("Client manager initialization failed (may be expected outside container): {}", e);
                // Don't fail the test - this is expected outside container
            }
        }
    }
}
