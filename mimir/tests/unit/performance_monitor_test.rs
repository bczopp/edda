use mimir::monitoring::PerformanceMonitor;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_response_time_tracking() {
    let monitor = PerformanceMonitor::new();
    
    // Record query response time
    monitor.record_query_time(Duration::from_millis(25)).await;
    monitor.record_query_time(Duration::from_millis(30)).await;
    monitor.record_query_time(Duration::from_millis(20)).await;
    
    let stats = monitor.get_stats().await;
    assert_eq!(stats.query_count, 3);
    assert!(stats.avg_query_time_ms > 0.0);
}

#[tokio::test]
async fn test_write_time_tracking() {
    let monitor = PerformanceMonitor::new();
    
    // Record write response time
    monitor.record_write_time(Duration::from_millis(50)).await;
    monitor.record_write_time(Duration::from_millis(60)).await;
    
    let stats = monitor.get_stats().await;
    assert_eq!(stats.write_count, 2);
    assert!(stats.avg_write_time_ms > 0.0);
}

#[tokio::test]
async fn test_throughput_measurement() {
    let monitor = PerformanceMonitor::new();
    
    // Record multiple operations
    for _ in 0..10 {
        monitor.record_query_time(Duration::from_millis(10)).await;
    }
    
    // Wait a bit
    sleep(Duration::from_millis(100)).await;
    
    let stats = monitor.get_stats().await;
    assert_eq!(stats.query_count, 10);
    // Throughput should be calculated based on time window
    assert!(stats.queries_per_second >= 0.0);
}

#[tokio::test]
async fn test_resource_usage_monitoring() {
    let monitor = PerformanceMonitor::new();
    
    // Record pool stats
    monitor.update_pool_stats(10, 5, false).await;
    
    let stats = monitor.get_stats().await;
    assert_eq!(stats.pool_size, 10);
    assert_eq!(stats.pool_idle, 5);
    assert_eq!(stats.pool_is_closed, false);
}

#[tokio::test]
async fn test_stats_reset() {
    let monitor = PerformanceMonitor::new();
    
    monitor.record_query_time(Duration::from_millis(25)).await;
    monitor.record_write_time(Duration::from_millis(50)).await;
    
    let stats_before = monitor.get_stats().await;
    assert!(stats_before.query_count > 0);
    
    monitor.reset_stats().await;
    
    let stats_after = monitor.get_stats().await;
    assert_eq!(stats_after.query_count, 0);
    assert_eq!(stats_after.write_count, 0);
}
