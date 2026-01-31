use nidhoggr::connection::ConnectionManager;
use nidhoggr::ratelimiter::RateLimiter;
use nidhoggr::performance::monitor::PerformanceMonitor;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_connection_latency_measurement() {
    let monitor = Arc::new(PerformanceMonitor::new());
    
    let connection_id = "test-connection".to_string();
    monitor.record_connection_start(connection_id.clone()).await;
    
    // Simulate connection time
    sleep(Duration::from_millis(50)).await;
    
    monitor.record_connection_end(connection_id).await;
    
    let avg_latency = monitor.get_avg_connection_latency().await;
    assert!(avg_latency > 0.0, "Connection latency should be measured");
    assert!(avg_latency < 100.0, "Connection latency should be reasonable");
}

#[tokio::test]
async fn test_routing_latency_measurement() {
    let monitor = Arc::new(PerformanceMonitor::new());
    
    let request_id = "test-request".to_string();
    monitor.record_routing_start(request_id.clone()).await;
    
    // Simulate routing time
    sleep(Duration::from_millis(10)).await;
    
    monitor.record_routing_end(request_id).await;
    
    let avg_latency = monitor.get_avg_routing_latency().await;
    assert!(avg_latency > 0.0, "Routing latency should be measured");
    assert!(avg_latency < 50.0, "Routing latency should be reasonable");
}

#[tokio::test]
async fn test_message_throughput() {
    let monitor = Arc::new(PerformanceMonitor::new());
    
    // Record multiple messages
    for _ in 0..100 {
        monitor.record_message().await;
    }
    
    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.total_messages, 100, "Should track total messages");
}

#[tokio::test]
async fn test_active_connections_tracking() {
    let monitor = Arc::new(PerformanceMonitor::new());
    let connection_manager = Arc::new(ConnectionManager::new());
    
    // Register connections
    use chrono::{Utc, Duration as ChronoDuration};
    for i in 0..10 {
        connection_manager.register_connection(
            &format!("device-{}", i),
            "test-user",
            Utc::now() + ChronoDuration::hours(1),
        ).await;
    }
    
    let active = connection_manager.get_active_connections().await;
    monitor.update_active_connections(active).await;
    
    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.active_connections, 10, "Should track active connections");
}
