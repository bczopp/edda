use nornen::urd::registry::ProviderRegistry;
use nornen::verdandi::router::RequestRouter;
use nornen::coordinator::NornenCoordinator;
use nornen::cache::ProviderCache;
use nornen::monitoring::collector::MetricsCollector;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;

/// Helper to create a test registry with PostgreSQL
async fn setup_test_registry() -> Arc<ProviderRegistry> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/nornen_test".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await
        .expect("Failed to connect to test database");
    
    Arc::new(ProviderRegistry::new(pool).await
        .expect("Failed to create ProviderRegistry"))
}

/// Helper to create a test registry with cache
async fn setup_test_registry_with_cache() -> (Arc<ProviderRegistry>, Arc<ProviderCache>) {
    let cache = Arc::new(ProviderCache::new(1000, 300));
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/nornen_test".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await
        .expect("Failed to connect to test database");
    
    let registry = Arc::new(ProviderRegistry::new_with_cache(pool, cache.clone()).await
        .expect("Failed to create ProviderRegistry with cache"));
    
    (registry, cache)
}

#[tokio::test]
#[ignore] // Performance test - run explicitly
async fn test_provider_registry_query_performance() {
    let registry = setup_test_registry().await;
    
    // Register test providers
    for i in 0..100 {
        registry.register_provider(
            &format!("provider_{}", i),
            &format!("Provider {}", i),
            &vec!["llm".to_string(), "stt".to_string()],
            &format!("http://provider{}.example.com", i),
            &serde_json::json!({}),
        ).await.expect("Failed to register provider");
    }
    
    // Warm-up query
    let _ = registry.query_providers(&vec!["llm".to_string()], Some("active")).await;
    
    // Measure query performance
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = registry.query_providers(&vec!["llm".to_string()], Some("active")).await;
    }
    
    let elapsed = start.elapsed();
    let avg_time_ms = elapsed.as_millis() as f64 / iterations as f64;
    let qps = iterations as f64 / elapsed.as_secs_f64();
    
    println!("Provider Registry Query Performance:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", elapsed);
    println!("  Average time per query: {:.2}ms", avg_time_ms);
    println!("  Queries per second: {:.2}", qps);
    
    // Assert performance targets
    assert!(avg_time_ms < 50.0, "Average query time should be < 50ms, got {:.2}ms", avg_time_ms);
    assert!(qps > 100.0, "Should handle > 100 queries/second, got {:.2}", qps);
}

#[tokio::test]
#[ignore] // Performance test - run explicitly
async fn test_provider_cache_performance() {
    let (registry, cache) = setup_test_registry_with_cache().await;
    
    // Register test providers
    for i in 0..50 {
        registry.register_provider(
            &format!("provider_{}", i),
            &format!("Provider {}", i),
            &vec!["llm".to_string()],
            &format!("http://provider{}.example.com", i),
            &serde_json::json!({}),
        ).await.expect("Failed to register provider");
    }
    
    // Query once to populate cache
    let providers = registry.query_providers(&vec!["llm".to_string()], Some("active")).await.unwrap();
    cache.set(&vec!["llm".to_string()], Some("active"), &providers).await;
    
    // Measure cache hit performance
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = cache.get(&vec!["llm".to_string()], Some("active")).await;
    }
    
    let elapsed = start.elapsed();
    let avg_time_us = elapsed.as_micros() as f64 / iterations as f64;
    let qps = iterations as f64 / elapsed.as_secs_f64();
    
    println!("Provider Cache Performance:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", elapsed);
    println!("  Average time per get: {:.2}μs", avg_time_us);
    println!("  Gets per second: {:.2}", qps);
    
    // Cache should be very fast
    assert!(avg_time_us < 100.0, "Average cache get should be < 100μs, got {:.2}μs", avg_time_us);
    assert!(qps > 10000.0, "Should handle > 10000 cache gets/second, got {:.2}", qps);
}

#[tokio::test]
#[ignore] // Performance test - run explicitly
async fn test_request_router_performance() {
    let (registry, cache) = setup_test_registry_with_cache().await;
    let router = Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()));
    
    // Register test providers
    for i in 0..50 {
        registry.register_provider(
            &format!("provider_{}", i),
            &format!("Provider {}", i),
            &vec!["llm".to_string()],
            &format!("http://provider{}.example.com", i),
            &serde_json::json!({}),
        ).await.expect("Failed to register provider");
    }
    
    // Warm-up
    let _ = router.route_request(&vec!["llm".to_string()], &HashMap::new()).await;
    
    // Measure routing performance
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = router.route_request(&vec!["llm".to_string()], &HashMap::new()).await;
    }
    
    let elapsed = start.elapsed();
    let avg_time_ms = elapsed.as_millis() as f64 / iterations as f64;
    let qps = iterations as f64 / elapsed.as_secs_f64();
    
    println!("Request Router Performance:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", elapsed);
    println!("  Average time per route: {:.2}ms", avg_time_ms);
    println!("  Routes per second: {:.2}", qps);
    
    // Router should be fast (includes cache lookup)
    assert!(avg_time_ms < 10.0, "Average route time should be < 10ms, got {:.2}ms", avg_time_ms);
    assert!(qps > 100.0, "Should handle > 100 routes/second, got {:.2}", qps);
}

#[tokio::test]
#[ignore] // Performance test - run explicitly
async fn test_coordinator_end_to_end_performance() {
    let (registry, cache) = setup_test_registry_with_cache().await;
    let router = Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()));
    let coordinator = Arc::new(NornenCoordinator::new(registry.clone(), router.clone()));
    
    // Register test providers
    for i in 0..50 {
        registry.register_provider(
            &format!("provider_{}", i),
            &format!("Provider {}", i),
            &vec!["llm".to_string()],
            &format!("http://provider{}.example.com", i),
            &serde_json::json!({}),
        ).await.expect("Failed to register provider");
    }
    
    // Warm-up
    let mut warmup_context = HashMap::new();
    warmup_context.insert("required_capabilities".to_string(), "llm".to_string());
    let _ = coordinator.coordinate_request("req_0", "llm_request", &warmup_context).await;
    
    // Measure end-to-end coordination performance
    let iterations = 500;
    let start = Instant::now();
    
    for i in 0..iterations {
        let mut context = HashMap::new();
        context.insert("required_capabilities".to_string(), "llm".to_string());
        let _ = coordinator.coordinate_request(
            &format!("req_{}", i),
            "llm_request",
            &context,
        ).await;
    }
    
    let elapsed = start.elapsed();
    let avg_time_ms = elapsed.as_millis() as f64 / iterations as f64;
    let qps = iterations as f64 / elapsed.as_secs_f64();
    
    println!("Coordinator End-to-End Performance:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", elapsed);
    println!("  Average time per coordination: {:.2}ms", avg_time_ms);
    println!("  Coordinations per second: {:.2}", qps);
    
    // End-to-end should meet the < 100ms target
    assert!(avg_time_ms < 100.0, "Average coordination time should be < 100ms, got {:.2}ms", avg_time_ms);
    assert!(qps > 10.0, "Should handle > 10 coordinations/second, got {:.2}", qps);
}

#[tokio::test]
#[ignore] // Performance test - run explicitly
async fn test_concurrent_load_performance() {
    let (registry, cache) = setup_test_registry_with_cache().await;
    let router = Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()));
    let coordinator = Arc::new(NornenCoordinator::new(registry.clone(), router.clone()));
    
    // Register test providers
    for i in 0..50 {
        registry.register_provider(
            &format!("provider_{}", i),
            &format!("Provider {}", i),
            &vec!["llm".to_string()],
            &format!("http://provider{}.example.com", i),
            &serde_json::json!({}),
        ).await.expect("Failed to register provider");
    }
    
    // Measure concurrent load performance
    let concurrent_requests = 50;
    let requests_per_worker = 20;
    let total_requests = concurrent_requests * requests_per_worker;
    
    let start = Instant::now();
    
    let handles: Vec<_> = (0..concurrent_requests)
        .map(|worker_id| {
            let coordinator = coordinator.clone();
            tokio::spawn(async move {
                for i in 0..requests_per_worker {
                    let mut context = HashMap::new();
                    context.insert("required_capabilities".to_string(), "llm".to_string());
                    let _ = coordinator.coordinate_request(
                        &format!("req_{}_{}", worker_id, i),
                        "llm_request",
                        &context,
                    ).await;
                }
            })
        })
        .collect();
    
    // Wait for all workers to complete
    for handle in handles {
        handle.await.expect("Worker failed");
    }
    
    let elapsed = start.elapsed();
    let avg_time_ms = elapsed.as_millis() as f64 / total_requests as f64;
    let qps = total_requests as f64 / elapsed.as_secs_f64();
    
    println!("Concurrent Load Performance:");
    println!("  Concurrent workers: {}", concurrent_requests);
    println!("  Requests per worker: {}", requests_per_worker);
    println!("  Total requests: {}", total_requests);
    println!("  Total time: {:?}", elapsed);
    println!("  Average time per request: {:.2}ms", avg_time_ms);
    println!("  Requests per second: {:.2}", qps);
    
    // Under concurrent load, should still meet performance targets
    assert!(avg_time_ms < 200.0, "Average request time under load should be < 200ms, got {:.2}ms", avg_time_ms);
    assert!(qps > 50.0, "Should handle > 50 requests/second under load, got {:.2}", qps);
}

#[tokio::test]
#[ignore] // Performance test - run explicitly
async fn test_cache_hit_rate_impact() {
    let (registry, cache) = setup_test_registry_with_cache().await;
    let router_with_cache = Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()));
    let router_without_cache = Arc::new(RequestRouter::new(registry.clone()));
    
    // Register test providers
    for i in 0..50 {
        registry.register_provider(
            &format!("provider_{}", i),
            &format!("Provider {}", i),
            &vec!["llm".to_string()],
            &format!("http://provider{}.example.com", i),
            &serde_json::json!({}),
        ).await.expect("Failed to register provider");
    }
    
    // Warm-up cache
    for _ in 0..10 {
        let _ = router_with_cache.select_provider(&vec!["llm".to_string()], &HashMap::new()).await;
    }
    
    // Measure with cache
    let iterations = 1000;
    let start_with_cache = Instant::now();
    for _ in 0..iterations {
        let _ = router_with_cache.select_provider(&vec!["llm".to_string()], &HashMap::new()).await;
    }
    let elapsed_with_cache = start_with_cache.elapsed();
    
    // Measure without cache
    let start_without_cache = Instant::now();
    for _ in 0..iterations {
        let _ = router_without_cache.select_provider(&vec!["llm".to_string()], &HashMap::new()).await;
    }
    let elapsed_without_cache = start_without_cache.elapsed();
    
    let speedup = elapsed_without_cache.as_secs_f64() / elapsed_with_cache.as_secs_f64();
    
    println!("Cache Impact on Performance:");
    println!("  Iterations: {}", iterations);
    println!("  Time with cache: {:?}", elapsed_with_cache);
    println!("  Time without cache: {:?}", elapsed_without_cache);
    println!("  Speedup: {:.2}x", speedup);
    
    // Cache should provide significant speedup
    assert!(speedup > 2.0, "Cache should provide > 2x speedup, got {:.2}x", speedup);
}
