#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use nornen::verdandi::router::RequestRouter;
    use nornen::coordinator::NornenCoordinator;
    use std::sync::Arc;
    use std::collections::HashMap;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_coordination_flow() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1".to_string()],
            "http://localhost:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        // Create router and coordinator
        let router = Arc::new(RequestRouter::new(registry.clone()));
        let coordinator = Arc::new(NornenCoordinator::new(registry.clone(), router.clone()));
        
        // Coordinate request
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "capability1".to_string());
        
        let result = coordinator.coordinate_request("req1", "test_request", &context).await.unwrap();
        assert_eq!(result.provider_id, "provider1");
        assert_eq!(result.decision, "route");
    }
}
