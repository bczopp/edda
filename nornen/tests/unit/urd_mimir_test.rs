#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use crate::utils::mock_mimir::create_registry_with_mimir;
    use crate::utils::test_helpers::wait_for_service;
    use serde_json::json;

    // These tests require a Mimir service (mock or real) to be running
    // In container-based testing, use docker-compose.test.yml with mock-mimir service
    // For local testing, set MIMIR_URL environment variable or use default localhost:50051

    fn get_mimir_url() -> String {
        std::env::var("MIMIR_URL").unwrap_or_else(|_| "http://localhost:50051".to_string())
    }

    async fn setup_registry() -> Result<ProviderRegistry, Box<dyn std::error::Error>> {
        let mimir_url = get_mimir_url();
        
        // Extract host:port from URL for TCP connection check
        let host_port = mimir_url
            .replace("http://", "")
            .replace("https://", "");
        
        // Wait for Mimir service to be ready (for container-based tests)
        // Try to connect via TCP first
        if !wait_for_service(&host_port, 20).await {
            // If TCP check fails, still try to connect - gRPC might be ready even if TCP check fails
            // This is a best-effort check
        }

        // Try to create registry - this will fail if Mimir is not available
        create_registry_with_mimir(&mimir_url).await
    }

    #[tokio::test]
    // Note: These tests require a Mimir service (mock or real)
    // In docker-compose.test.yml, the mock-mimir service is started automatically
    // For local testing, set MIMIR_URL environment variable or use default localhost:50051
    async fn test_register_provider_with_mimir() {
        crate::common::setup_logging();
        let registry = setup_registry().await.expect("Failed to setup registry");
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1".to_string(), "capability2".to_string()],
            "http://localhost:8080",
            &json!({"version": "1.0"}),
        ).await.expect("Failed to register provider");
        
        // Query providers to verify it was stored
        let providers = registry.query_providers(&["capability1".to_string()], Some("active"))
            .await.expect("Failed to query providers");
        
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].provider_id, "provider1");
        assert_eq!(providers[0].name, "Test Provider");
        assert!(providers[0].capabilities.contains(&"capability1".to_string()));
    }

    #[tokio::test]
    async fn test_register_provider_duplicate_with_mimir() {
        crate::common::setup_logging();
        let registry = setup_registry().await.expect("Failed to setup registry");
        
        // Register provider first time
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1".to_string()],
            "http://localhost:8080",
            &json!({}),
        ).await.expect("Failed to register provider");
        
        // Try to register same provider again - should fail
        let result = registry.register_provider(
            "provider1",
            "Test Provider 2",
            &["capability1".to_string()],
            "http://localhost:8081",
            &json!({}),
        ).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Provider already exists"));
    }

    #[tokio::test]
    #[ignore] // Requires Mimir service
    async fn test_query_providers_by_capability_with_mimir() {
        crate::common::setup_logging();
        let registry = setup_registry().await.expect("Failed to setup registry");
        
        // Register multiple providers with different capabilities
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string(), "text".to_string()],
            "http://provider1:8080",
            &json!({}),
        ).await.expect("Failed to register provider1");
        
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["llm".to_string()],
            "http://provider2:8080",
            &json!({}),
        ).await.expect("Failed to register provider2");
        
        registry.register_provider(
            "provider3",
            "Provider 3",
            &["text".to_string()],
            "http://provider3:8080",
            &json!({}),
        ).await.expect("Failed to register provider3");
        
        // Query by single capability
        let providers = registry.query_providers(&["llm".to_string()], Some("active"))
            .await.expect("Failed to query providers");
        assert_eq!(providers.len(), 2);
        assert!(providers.iter().any(|p| p.provider_id == "provider1"));
        assert!(providers.iter().any(|p| p.provider_id == "provider2"));
        
        // Query by multiple capabilities (must have all)
        let providers = registry.query_providers(&["llm".to_string(), "text".to_string()], Some("active"))
            .await.expect("Failed to query providers");
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].provider_id, "provider1");
    }

    #[tokio::test]
    async fn test_update_provider_with_mimir() {
        crate::common::setup_logging();
        let registry = setup_registry().await.expect("Failed to setup registry");
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Original Name",
            &["capability1".to_string()],
            "http://original:8080",
            &json!({"version": "1.0"}),
        ).await.expect("Failed to register provider");
        
        // Update provider
        registry.update_provider(
            "provider1",
            Some("Updated Name"),
            Some(&["capability1".to_string(), "capability2".to_string()]),
            Some("http://updated:8080"),
            Some(&json!({"version": "2.0"})),
        ).await.expect("Failed to update provider");
        
        // Verify changes
        let providers = registry.query_providers(&["capability1".to_string()], Some("active"))
            .await.expect("Failed to query providers");
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].name, "Updated Name");
        assert_eq!(providers[0].endpoint, "http://updated:8080");
        assert_eq!(providers[0].capabilities.len(), 2);
        assert_eq!(providers[0].metadata["version"], "2.0");
    }

    #[tokio::test]
    #[ignore] // Requires Mimir service
    async fn test_update_provider_status_with_mimir() {
        crate::common::setup_logging();
        let registry = setup_registry().await.expect("Failed to setup registry");
        
        // Register provider with "active" status
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1".to_string()],
            "http://localhost:8080",
            &json!({}),
        ).await.expect("Failed to register provider");
        
        // Verify it's returned when querying for active
        let providers = registry.query_providers(&["capability1".to_string()], Some("active"))
            .await.expect("Failed to query providers");
        assert_eq!(providers.len(), 1);
        
        // Update status to inactive
        registry.update_provider_status("provider1", "inactive")
            .await.expect("Failed to update status");
        
        // Query for active providers - should not return inactive provider
        let providers = registry.query_providers(&["capability1".to_string()], Some("active"))
            .await.expect("Failed to query providers");
        assert_eq!(providers.len(), 0);
        
        // Query without status filter - should return inactive provider
        let providers = registry.query_providers(&["capability1".to_string()], None)
            .await.expect("Failed to query providers");
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].status, "inactive");
    }

    #[tokio::test]
    async fn test_list_providers_with_mimir() {
        crate::common::setup_logging();
        let registry = setup_registry().await.expect("Failed to setup registry");
        
        // Register multiple providers
        for i in 1..=5 {
            registry.register_provider(
                &format!("provider{}", i),
                &format!("Provider {}", i),
                &["capability1".to_string()],
                &format!("http://provider{}:8080", i),
                &json!({}),
            ).await.expect(&format!("Failed to register provider{}", i));
        }
        
        // List with pagination - first page
        let result = registry.list_providers(2, 0).await.expect("Failed to list providers");
        assert_eq!(result.providers.len(), 2);
        assert_eq!(result.total, 5);
        
        // Second page
        let result = registry.list_providers(2, 2).await.expect("Failed to list providers");
        assert_eq!(result.providers.len(), 2);
        
        // Third page
        let result = registry.list_providers(2, 4).await.expect("Failed to list providers");
        assert_eq!(result.providers.len(), 1);
        
        // Beyond available data
        let result = registry.list_providers(2, 10).await.expect("Failed to list providers");
        assert_eq!(result.providers.len(), 0);
    }
}
