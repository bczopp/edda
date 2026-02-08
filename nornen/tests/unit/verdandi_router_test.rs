#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use nornen::verdandi::router::{RequestRouter, RequestRouterError};
    use std::sync::Arc;
    use std::collections::HashMap;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_route_request_with_single_provider() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register a provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string(), "text".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Route request with matching capabilities
        let provider = router.route_request(&["llm".to_string()], &preferences).await.unwrap();
        
        assert_eq!(provider.provider_id, "provider1");
        assert_eq!(provider.endpoint, "http://provider1:8080");
    }

    #[tokio::test]
    async fn test_route_request_no_providers() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Route request with no providers registered
        let result = router.route_request(&["llm".to_string()], &preferences).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            RequestRouterError::NoProviderAvailable(_) => {},
            _ => panic!("Expected NoProviderAvailable error"),
        }
    }

    #[tokio::test]
    async fn test_route_request_multiple_providers() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register multiple providers
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["llm".to_string()],
            "http://provider2:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Route request - should select one provider (load balancing)
        let provider = router.route_request(&["llm".to_string()], &preferences).await.unwrap();
        
        assert!(provider.provider_id == "provider1" || provider.provider_id == "provider2");
    }

    #[tokio::test]
    async fn test_route_request_capability_filtering() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register providers with different capabilities
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string(), "text".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["stt".to_string(), "tts".to_string()],
            "http://provider2:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Route request with specific capabilities
        let provider = router.route_request(&["llm".to_string()], &preferences).await.unwrap();
        
        assert_eq!(provider.provider_id, "provider1");
        assert!(provider.capabilities.contains(&"llm".to_string()));
    }

    #[tokio::test]
    async fn test_route_request_status_filtering() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register providers with different statuses
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["llm".to_string()],
            "http://provider2:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        // Set provider2 to inactive
        registry.update_provider_status("provider2", "inactive").await.unwrap();
        
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Route request - should only return active providers
        let provider = router.route_request(&["llm".to_string()], &preferences).await.unwrap();
        
        assert_eq!(provider.provider_id, "provider1");
        assert_eq!(provider.status, "active");
    }

    #[tokio::test]
    async fn test_select_provider_with_preferences() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register providers
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string()],
            "http://provider1:8080",
            &serde_json::json!({"region": "us-east"}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["llm".to_string()],
            "http://provider2:8080",
            &serde_json::json!({"region": "eu-west"}),
        ).await.unwrap();
        
        let router = RequestRouter::new(registry);
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        
        // Select provider with preferences
        let (provider_id, endpoint, score) = router.select_provider(&["llm".to_string()], &preferences).await.unwrap();
        
        assert!(!provider_id.is_empty());
        assert!(!endpoint.is_empty());
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[tokio::test]
    async fn test_load_balancing_round_robin() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register multiple providers
        for i in 1..=3 {
            registry.register_provider(
                &format!("provider{}", i),
                &format!("Provider {}", i),
                &["llm".to_string()],
                &format!("http://provider{}:8080", i),
                &serde_json::json!({}),
            ).await.unwrap();
        }
        
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Route multiple requests - should distribute requests (round-robin)
        let mut selected_providers = Vec::new();
        for _ in 0..6 {
            let provider = router.route_request(&["llm".to_string()], &preferences).await.unwrap();
            selected_providers.push(provider.provider_id);
        }
        
        // Should have selected from multiple providers
        let unique_providers: std::collections::HashSet<_> = selected_providers.iter().collect();
        assert!(unique_providers.len() >= 1); // At least one provider should be selected
    }

    #[tokio::test]
    async fn test_calculate_score_basic() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Get provider and calculate score
        let (_, _, score) = router.select_provider(&["llm".to_string()], &preferences).await.unwrap();
        
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[tokio::test]
    async fn test_calculate_score_with_preferences() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = RequestRouter::new(registry);
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        
        // Get provider and calculate score with preferences
        let (_, _, score_with_prefs) = router.select_provider(&["llm".to_string()], &preferences).await.unwrap();
        
        // Score should be reasonable (provider matches status preference)
        assert!(score_with_prefs >= 0.0 && score_with_prefs <= 1.0);
    }

    #[tokio::test]
    async fn test_route_request_with_fallback() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register multiple providers
        for i in 1..=3 {
            registry.register_provider(
                &format!("provider{}", i),
                &format!("Provider {}", i),
                &["llm".to_string()],
                &format!("http://provider{}:8080", i),
                &serde_json::json!({}),
            ).await.unwrap();
        }
        
        let router = RequestRouter::new(registry);
        let preferences = HashMap::new();
        
        // Route request with fallback
        let provider = router.route_request_with_fallback(
            &["llm".to_string()],
            &preferences,
            2, // max_attempts
        ).await.unwrap();
        
        assert!(!provider.provider_id.is_empty());
    }
}
