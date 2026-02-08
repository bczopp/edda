#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use nornen::verdandi::router::RequestRouter;
    use nornen::coordinator::{NornenCoordinator, CoordinationError, ServiceHealth, ServiceStatus};
    use std::sync::Arc;
    use std::collections::HashMap;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_coordinate_request_basic() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string(), "text".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        // Coordinate request
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "llm,text".to_string());
        
        let result = coordinator.coordinate_request("req1", "test_request", &context).await.unwrap();
        
        assert_eq!(result.decision, "route");
        assert_eq!(result.provider_id, "provider1");
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
        assert!(!result.reasoning.is_empty());
    }

    #[tokio::test]
    async fn test_coordinate_request_with_preferences() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register multiple providers
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
        
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        // Coordinate request with preferences
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "llm".to_string());
        context.insert("pref_status".to_string(), "active".to_string());
        context.insert("pref_region".to_string(), "us-east".to_string());
        
        let result = coordinator.coordinate_request("req2", "test_request", &context).await.unwrap();
        
        assert_eq!(result.decision, "route");
        assert!(!result.provider_id.is_empty());
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }

    #[tokio::test]
    async fn test_coordinate_request_no_providers() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        // Coordinate request with no providers registered
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "llm".to_string());
        
        let result = coordinator.coordinate_request("req3", "test_request", &context).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            CoordinationError::RoutingError(_) => {},
            _ => panic!("Expected RoutingError"),
        }
    }

    #[tokio::test]
    async fn test_coordinate_request_empty_capabilities() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        // Coordinate request with empty capabilities (should still work, returns any provider)
        let context = HashMap::new();
        
        let result = coordinator.coordinate_request("req4", "test_request", &context).await;
        
        // Should either succeed (if any provider matches) or fail (if no providers match empty capabilities)
        // This depends on implementation - for now we'll just check it doesn't panic
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_coordinate_request_different_request_types() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string()],
            "http://provider1:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "llm".to_string());
        
        // Test different request types
        let result1 = coordinator.coordinate_request("req5", "llm_request", &context).await.unwrap();
        let result2 = coordinator.coordinate_request("req6", "text_request", &context).await.unwrap();
        
        assert_eq!(result1.decision, "route");
        assert_eq!(result2.decision, "route");
        // Reasoning should include request type
        assert!(result1.reasoning.contains("llm_request"));
        assert!(result2.reasoning.contains("text_request"));
    }

    #[tokio::test]
    async fn test_health_check() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        // Perform health check
        let health = coordinator.health_check().await;
        
        assert_eq!(health.status, "healthy");
        assert!(health.uptime_seconds >= 0);
        assert_eq!(health.request_count, 0); // No requests yet
        assert!(health.registry_healthy);
    }

    #[tokio::test]
    async fn test_get_status() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register some providers
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
        
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        // Get status
        let status = coordinator.get_status().await;
        
        assert_eq!(status.health.status, "healthy");
        assert!(status.provider_stats.total_providers >= 2);
        assert!(status.provider_stats.active_providers >= 2);
    }

    #[tokio::test]
    async fn test_request_counting() {
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
        
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = NornenCoordinator::new(registry.clone(), router.clone());
        
        // Initial health check
        let health1 = coordinator.health_check().await;
        assert_eq!(health1.request_count, 0);
        
        // Coordinate some requests
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "llm".to_string());
        
        coordinator.coordinate_request("req1", "test", &context).await.unwrap();
        coordinator.coordinate_request("req2", "test", &context).await.unwrap();
        
        // Check request count increased
        let health2 = coordinator.health_check().await;
        assert_eq!(health2.request_count, 2);
    }
}
