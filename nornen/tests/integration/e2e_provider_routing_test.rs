#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use nornen::verdandi::router::RequestRouter;
    use nornen::coordinator::NornenCoordinator;
    use crate::common::TestDatabase;
    use serde_json::json;
    use std::sync::Arc;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_e2e_provider_registration_to_routing() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Initialize components
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = Arc::new(NornenCoordinator::new(registry.clone(), router.clone()));
        
        // Step 1: Register multiple providers with different capabilities
        registry.register_provider(
            "provider1",
            "LLM Provider 1",
            &["llm".to_string(), "text".to_string()],
            "http://provider1:8080",
            &json!({"region": "us-east", "model": "gpt-4"}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider2",
            "LLM Provider 2",
            &["llm".to_string()],
            "http://provider2:8080",
            &json!({"region": "eu-west", "model": "claude-3"}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider3",
            "Text Provider",
            &["text".to_string()],
            "http://provider3:8080",
            &json!({"region": "us-west"}),
        ).await.unwrap();
        
        // Step 2: Query providers by capability
        let llm_providers = registry.query_providers(&["llm".to_string()], Some("active")).await.unwrap();
        assert_eq!(llm_providers.len(), 2);
        assert!(llm_providers.iter().any(|p| p.provider_id == "provider1"));
        assert!(llm_providers.iter().any(|p| p.provider_id == "provider2"));
        
        let text_providers = registry.query_providers(&["text".to_string()], Some("active")).await.unwrap();
        assert_eq!(text_providers.len(), 2);
        
        // Step 3: Route request with single capability
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        
        let provider = router.route_request(
            &["llm".to_string()],
            &preferences,
        ).await.unwrap();
        
        assert!(provider.provider_id == "provider1" || provider.provider_id == "provider2");
        assert!(provider.capabilities.contains(&"llm".to_string()));
        
        // Step 4: Route request with multiple capabilities
        let provider = router.route_request(
            &["llm".to_string(), "text".to_string()],
            &preferences,
        ).await.unwrap();
        
        assert_eq!(provider.provider_id, "provider1");
        assert!(provider.capabilities.contains(&"llm".to_string()));
        assert!(provider.capabilities.contains(&"text".to_string()));
        
        // Step 5: Use coordinator to coordinate request
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "llm,text".to_string());
        context.insert("pref_status".to_string(), "active".to_string());
        
        let result = coordinator.coordinate_request("req1", "test_request", &context).await.unwrap();
        assert_eq!(result.decision, "route");
        assert_eq!(result.provider_id, "provider1");
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }

    #[tokio::test]
    async fn test_e2e_provider_update_affects_routing() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Initialize components
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = Arc::new(RequestRouter::new(registry.clone()));
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string()],
            "http://provider1:8080",
            &json!({}),
        ).await.unwrap();
        
        // Route request - should find provider
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        
        let provider = router.route_request(
            &["llm".to_string()],
            &preferences,
        ).await.unwrap();
        assert_eq!(provider.provider_id, "provider1");
        
        // Update provider status to inactive
        registry.update_provider_status("provider1", "inactive").await.unwrap();
        
        // Route request - should not find inactive provider
        let result = router.route_request(
            &["llm".to_string()],
            &preferences,
        ).await;
        assert!(result.is_err());
        
        // Update provider status back to active
        registry.update_provider_status("provider1", "active").await.unwrap();
        
        // Route request - should find provider again
        let provider = router.route_request(
            &["llm".to_string()],
            &preferences,
        ).await.unwrap();
        assert_eq!(provider.provider_id, "provider1");
    }

    #[tokio::test]
    async fn test_e2e_fallback_routing() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Initialize components
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = Arc::new(RequestRouter::new(registry.clone()));
        
        // Register multiple providers with same capability
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string()],
            "http://provider1:8080",
            &json!({}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["llm".to_string()],
            "http://provider2:8080",
            &json!({}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider3",
            "Provider 3",
            &["llm".to_string()],
            "http://provider3:8080",
            &json!({}),
        ).await.unwrap();
        
        // Use fallback routing - should try multiple providers
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        
        let provider = router.route_request_with_fallback(
            &["llm".to_string()],
            &preferences,
            3, // max_attempts
        ).await.unwrap();
        
        assert!(provider.provider_id == "provider1" || 
                provider.provider_id == "provider2" || 
                provider.provider_id == "provider3");
    }

    #[tokio::test]
    async fn test_e2e_preference_based_routing() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Initialize components
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = Arc::new(RequestRouter::new(registry.clone()));
        
        // Register providers with different regions
        registry.register_provider(
            "provider-us",
            "US Provider",
            &["llm".to_string()],
            "http://provider-us:8080",
            &json!({"region": "us-east"}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider-eu",
            "EU Provider",
            &["llm".to_string()],
            "http://provider-eu:8080",
            &json!({"region": "eu-west"}),
        ).await.unwrap();
        
        // Route with region preference
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        preferences.insert("region".to_string(), "us-east".to_string());
        
        let provider = router.route_request(
            &["llm".to_string()],
            &preferences,
        ).await.unwrap();
        
        // Should prefer US provider due to region preference
        assert_eq!(provider.provider_id, "provider-us");
        assert_eq!(provider.metadata["region"], "us-east");
    }

    #[tokio::test]
    async fn test_e2e_provider_capability_update() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Initialize components
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = Arc::new(RequestRouter::new(registry.clone()));
        
        // Register provider with single capability
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string()],
            "http://provider1:8080",
            &json!({}),
        ).await.unwrap();
        
        // Route request for llm - should work
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        
        let provider = router.route_request(
            &["llm".to_string()],
            &preferences,
        ).await.unwrap();
        assert_eq!(provider.provider_id, "provider1");
        
        // Route request for text - should fail
        let result = router.route_request(
            &["text".to_string()],
            &preferences,
        ).await;
        assert!(result.is_err());
        
        // Update provider to add text capability
        registry.update_provider(
            "provider1",
            None,
            Some(&["llm".to_string(), "text".to_string()]),
            None,
            None,
        ).await.unwrap();
        
        // Route request for text - should work now
        let provider = router.route_request(
            &["text".to_string()],
            &preferences,
        ).await.unwrap();
        assert_eq!(provider.provider_id, "provider1");
        assert!(provider.capabilities.contains(&"text".to_string()));
        
        // Route request for both capabilities - should work
        let provider = router.route_request(
            &["llm".to_string(), "text".to_string()],
            &preferences,
        ).await.unwrap();
        assert_eq!(provider.provider_id, "provider1");
    }
}
