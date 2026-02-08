#[cfg(test)]
mod tests {
    use nornen::monitoring::collector::MetricsCollector;
    use nornen::cache::ProviderCache;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_metrics_collector_record_success() {
        let collector = MetricsCollector::new(None);
        
        collector.record_success("provider1", 100).await;
        collector.record_success("provider1", 200).await;
        collector.record_success("provider2", 150).await;
        
        let metrics = collector.get_metrics(2).await;
        
        assert_eq!(metrics.requests.total_requests, 3);
        assert_eq!(metrics.requests.successful_requests, 3);
        assert_eq!(metrics.requests.failed_requests, 0);
        assert_eq!(metrics.providers.len(), 2);
        
        // Check provider1 metrics
        let provider1_metrics = metrics.providers.iter()
            .find(|p| p.provider_id == "provider1")
            .unwrap();
        assert_eq!(provider1_metrics.total_requests, 2);
        assert_eq!(provider1_metrics.successful_requests, 2);
        assert_eq!(provider1_metrics.average_response_time_ms, 150.0);
    }

    #[tokio::test]
    async fn test_metrics_collector_record_failure() {
        let collector = MetricsCollector::new(None);
        
        collector.record_success("provider1", 100).await;
        collector.record_failure("provider1", 50).await;
        collector.record_failure("provider2", 75).await;
        
        let metrics = collector.get_metrics(2).await;
        
        assert_eq!(metrics.requests.total_requests, 3);
        assert_eq!(metrics.requests.successful_requests, 1);
        assert_eq!(metrics.requests.failed_requests, 2);
        
        // Check provider1 metrics
        let provider1_metrics = metrics.providers.iter()
            .find(|p| p.provider_id == "provider1")
            .unwrap();
        assert_eq!(provider1_metrics.total_requests, 2);
        assert_eq!(provider1_metrics.successful_requests, 1);
        assert_eq!(provider1_metrics.failed_requests, 1);
    }

    #[tokio::test]
    async fn test_metrics_collector_average_response_time() {
        let collector = MetricsCollector::new(None);
        
        collector.record_success("provider1", 100).await;
        collector.record_success("provider1", 200).await;
        collector.record_success("provider1", 300).await;
        
        let metrics = collector.get_metrics(1).await;
        
        let provider1_metrics = metrics.providers.iter()
            .find(|p| p.provider_id == "provider1")
            .unwrap();
        
        // Average should be (100 + 200 + 300) / 3 = 200
        assert_eq!(provider1_metrics.average_response_time_ms, 200.0);
    }

    #[tokio::test]
    async fn test_metrics_collector_requests_per_second() {
        let collector = MetricsCollector::new(None);
        
        // Record some requests
        for _ in 0..10 {
            collector.record_success("provider1", 100).await;
        }
        
        // Wait a bit
        sleep(Duration::from_millis(100)).await;
        
        let metrics = collector.get_metrics(1).await;
        
        assert_eq!(metrics.requests.total_requests, 10);
        // Requests per second should be calculated based on uptime
        assert!(metrics.requests.requests_per_second > 0.0);
    }

    #[tokio::test]
    async fn test_metrics_collector_with_cache() {
        let cache = Arc::new(ProviderCache::new(100, 60));
        let collector = MetricsCollector::new(Some(cache.clone()));
        
        // Populate cache
        cache.set(&["llm".to_string()], Some("active"), vec![]).await;
        
        collector.record_success("provider1", 100).await;
        
        let metrics = collector.get_metrics(1).await;
        
        // Cache metrics should be included
        assert_eq!(metrics.system.cache_size, 1);
        assert_eq!(metrics.system.cache_max_size, 100);
    }

    #[tokio::test]
    async fn test_metrics_collector_reset() {
        let collector = MetricsCollector::new(None);
        
        collector.record_success("provider1", 100).await;
        collector.record_success("provider2", 200).await;
        
        let metrics_before = collector.get_metrics(2).await;
        assert_eq!(metrics_before.requests.total_requests, 2);
        assert_eq!(metrics_before.providers.len(), 2);
        
        collector.reset().await;
        
        let metrics_after = collector.get_metrics(0).await;
        assert_eq!(metrics_after.requests.total_requests, 0);
        assert_eq!(metrics_after.providers.len(), 0);
    }

    #[tokio::test]
    async fn test_metrics_collector_last_used() {
        let collector = MetricsCollector::new(None);
        
        collector.record_success("provider1", 100).await;
        
        let metrics = collector.get_metrics(1).await;
        let provider1_metrics = metrics.providers.iter()
            .find(|p| p.provider_id == "provider1")
            .unwrap();
        
        assert!(provider1_metrics.last_used.is_some());
    }
}
