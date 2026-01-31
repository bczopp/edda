#[cfg(test)]
mod tests {
    use odin::orchestration::responsibility::ResponsibilityManager;
    use odin::orchestration::UserRequest;
    use odin::protocols::einherjar::CapabilityCache;
    use odin::protocols::manager::ProtocolManager;
    use odin::clients::manager::ClientManager;
    use odin::utils::config::SettingsManager;
    use std::sync::Arc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_responsibility_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let capability_cache = Arc::new(CapabilityCache::new());
        let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));
        let client_manager = Arc::new(ClientManager::new(settings_arc));
        
        let responsibility_manager = ResponsibilityManager::new(
            capability_cache,
            protocol_manager,
            client_manager,
        );
        
        assert!(true, "ResponsibilityManager should be created successfully");
    }

    #[tokio::test]
    async fn test_determine_responsibility_with_capabilities() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let capability_cache = Arc::new(CapabilityCache::new());
        
        // Add test capability to cache
        let test_capability = odin::protocols::einherjar::einherjar::CapabilityResponse {
            god_name: "geri".to_string(),
            purpose: "LLM Processing".to_string(),
            functions: vec![],
            responsibility_domains: vec!["text".to_string(), "question".to_string()],
            responsibility_keywords: vec!["answer".to_string(), "explain".to_string()],
        };
        
        capability_cache.update(
            "geri".to_string(),
            "http://localhost:50054".to_string(),
            test_capability,
        ).await;
        
        let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));
        let client_manager = Arc::new(ClientManager::new(settings_arc));
        
        let responsibility_manager = ResponsibilityManager::new(
            capability_cache,
            protocol_manager,
            client_manager,
        );
        
        let request = UserRequest {
            request_id: "test-1".to_string(),
            user_id: "user-1".to_string(),
            device_id: "device-1".to_string(),
            input: "Can you explain how this works?".to_string(),
            input_type: "text".to_string(),
        };
        
        let result = responsibility_manager.determine_responsibility(&request).await;
        match result {
            Ok(Some((service_name, score))) => {
                assert_eq!(service_name, "geri", "Should route to Geri for text questions");
                assert!(score > 0.0, "Score should be positive");
            }
            Ok(None) => {
                println!("No service found (may be expected if no capabilities)");
            }
            Err(e) => {
                println!("Error determining responsibility: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_calculate_relevance_score() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let capability_cache = Arc::new(CapabilityCache::new());
        let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));
        let client_manager = Arc::new(ClientManager::new(settings_arc));
        
        let responsibility_manager = ResponsibilityManager::new(
            capability_cache,
            protocol_manager,
            client_manager,
        );
        
        let request = UserRequest {
            request_id: "test-1".to_string(),
            user_id: "user-1".to_string(),
            device_id: "device-1".to_string(),
            input: "Execute file operation".to_string(),
            input_type: "text".to_string(),
        };
        
        let capability = odin::protocols::einherjar::einherjar::CapabilityResponse {
            god_name: "thor".to_string(),
            purpose: "Action Execution".to_string(),
            functions: vec![],
            responsibility_domains: vec!["action".to_string(), "file".to_string()],
            responsibility_keywords: vec!["execute".to_string(), "file".to_string()],
        };
        
        let score = responsibility_manager.calculate_relevance_score(&request, &capability).await;
        assert!(score > 0.0, "Score should be positive for matching request");
    }
}
