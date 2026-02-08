use geri::performance::{PerformanceTracker, ProviderMetrics, PerformanceWindow};
use std::time::Duration;

#[tokio::test]
async fn test_tracker_creation() {
    let tracker = PerformanceTracker::new();
    assert!(tracker.is_ok());
}

#[tokio::test]
async fn test_record_request_success() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    tracker.record_request_start("provider1", "model1").await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    tracker.record_request_success("provider1", "model1", 100, 50).await;
    
    let metrics = tracker.get_metrics("provider1", "model1").await;
    assert!(metrics.is_some());
    
    let m = metrics.unwrap();
    assert_eq!(m.total_requests, 1);
    assert_eq!(m.successful_requests, 1);
    assert_eq!(m.failed_requests, 0);
}

#[tokio::test]
async fn test_record_request_failure() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    tracker.record_request_start("provider1", "model1").await;
    tracker.record_request_failure("provider1", "model1", "timeout").await;
    
    let metrics = tracker.get_metrics("provider1", "model1").await;
    assert!(metrics.is_some());
    
    let m = metrics.unwrap();
    assert_eq!(m.total_requests, 1);
    assert_eq!(m.successful_requests, 0);
    assert_eq!(m.failed_requests, 1);
}

#[tokio::test]
async fn test_calculate_success_rate() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    // 3 successful, 1 failed
    for _ in 0..3 {
        tracker.record_request_start("provider1", "model1").await;
        tracker.record_request_success("provider1", "model1", 100, 50).await;
    }
    
    tracker.record_request_start("provider1", "model1").await;
    tracker.record_request_failure("provider1", "model1", "error").await;
    
    let metrics = tracker.get_metrics("provider1", "model1").await.unwrap();
    let success_rate = metrics.success_rate();
    
    // 3/4 = 0.75 = 75%
    assert!((success_rate - 0.75).abs() < 0.01);
}

#[tokio::test]
async fn test_calculate_average_latency() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    tracker.record_request_start("provider1", "model1").await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    tracker.record_request_success("provider1", "model1", 100, 50).await;
    
    tracker.record_request_start("provider1", "model1").await;
    tokio::time::sleep(Duration::from_millis(20)).await;
    tracker.record_request_success("provider1", "model1", 100, 50).await;
    
    let metrics = tracker.get_metrics("provider1", "model1").await.unwrap();
    let avg_latency = metrics.average_latency_ms();
    
    // Average should be around 15ms
    assert!(avg_latency >= 10.0);
    assert!(avg_latency <= 25.0);
}

#[tokio::test]
async fn test_calculate_tokens_per_second() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    tracker.record_request_start("provider1", "model1").await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    // 100 tokens in 100ms = 1000 tokens/second
    tracker.record_request_success("provider1", "model1", 50, 50).await;
    
    let metrics = tracker.get_metrics("provider1", "model1").await.unwrap();
    let tps = metrics.tokens_per_second();
    
    // Should be around 1000 tokens/second
    assert!(tps > 500.0);
}

#[tokio::test]
async fn test_windowed_metrics() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    // Old request (outside window)
    tracker.record_request_start("provider1", "model1").await;
    tracker.record_request_success("provider1", "model1", 100, 50).await;
    
    // Wait to create time separation
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    // Recent request (inside window)
    tracker.record_request_start("provider1", "model1").await;
    tracker.record_request_success("provider1", "model1", 200, 100).await;
    
    // Get metrics for last 100ms window
    let window = PerformanceWindow::Last100Ms;
    let metrics = tracker.get_windowed_metrics("provider1", "model1", window).await;
    
    assert!(metrics.is_some());
}

#[tokio::test]
async fn test_multiple_providers() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    tracker.record_request_start("provider1", "model1").await;
    tracker.record_request_success("provider1", "model1", 100, 50).await;
    
    tracker.record_request_start("provider2", "model2").await;
    tracker.record_request_success("provider2", "model2", 200, 100).await;
    
    let m1 = tracker.get_metrics("provider1", "model1").await.unwrap();
    let m2 = tracker.get_metrics("provider2", "model2").await.unwrap();
    
    assert_eq!(m1.total_requests, 1);
    assert_eq!(m2.total_requests, 1);
}

#[tokio::test]
async fn test_compare_providers() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    // Provider1: Fast, high success rate
    for _ in 0..5 {
        tracker.record_request_start("provider1", "model1").await;
        tokio::time::sleep(Duration::from_millis(5)).await;
        tracker.record_request_success("provider1", "model1", 100, 50).await;
    }
    
    // Provider2: Slower, lower success rate
    for _ in 0..3 {
        tracker.record_request_start("provider2", "model2").await;
        tokio::time::sleep(Duration::from_millis(20)).await;
        tracker.record_request_success("provider2", "model2", 100, 50).await;
    }
    
    tracker.record_request_start("provider2", "model2").await;
    tracker.record_request_failure("provider2", "model2", "timeout").await;
    
    let m1 = tracker.get_metrics("provider1", "model1").await.unwrap();
    let m2 = tracker.get_metrics("provider2", "model2").await.unwrap();
    
    assert!(m1.success_rate() > m2.success_rate());
    assert!(m1.average_latency_ms() < m2.average_latency_ms());
}

#[tokio::test]
async fn test_get_best_provider() {
    let mut tracker = PerformanceTracker::new().expect("tracker creation failed");
    
    // Setup multiple providers with different performance
    tracker.record_request_start("fast_provider", "model1").await;
    tokio::time::sleep(Duration::from_millis(5)).await;
    tracker.record_request_success("fast_provider", "model1", 100, 50).await;
    
    tracker.record_request_start("slow_provider", "model1").await;
    tokio::time::sleep(Duration::from_millis(50)).await;
    tracker.record_request_success("slow_provider", "model1", 100, 50).await;
    
    let best = tracker.get_best_provider(vec!["fast_provider", "slow_provider"]).await;
    assert!(best.is_some());
    assert_eq!(best.unwrap(), "fast_provider");
}
