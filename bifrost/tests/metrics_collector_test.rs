//! Tests for Phase 14.2.1: MetricsCollector (response time, throughput, quality, resource).

use bifrost::utils::metrics::MetricsCollector;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn record_response_time_and_snapshot() {
    let collector = Arc::new(MetricsCollector::new());
    collector.record_response_time("route", Duration::from_millis(50));
    collector.record_response_time("route", Duration::from_millis(100));
    let snap = collector.snapshot();
    assert!(snap.avg_response_time_ms("route").unwrap() >= 50);
    assert!(snap.avg_response_time_ms("route").unwrap() <= 100);
}

#[test]
fn record_throughput_and_snapshot() {
    let collector = Arc::new(MetricsCollector::new());
    collector.record_message_sent();
    collector.record_message_sent();
    collector.record_message_received();
    let snap = collector.snapshot();
    assert_eq!(snap.messages_sent, 2);
    assert_eq!(snap.messages_received, 1);
}

#[test]
fn record_connection_quality_and_snapshot() {
    let collector = Arc::new(MetricsCollector::new());
    collector.record_connection_quality("conn-1", 80);
    collector.record_connection_quality("conn-1", 60);
    let snap = collector.snapshot();
    let avg = snap.avg_connection_quality("conn-1").unwrap();
    assert!(avg >= 60 && avg <= 80);
}

#[test]
fn record_resource_usage_and_snapshot() {
    let collector = Arc::new(MetricsCollector::new());
    collector.record_connections_count(5);
    collector.record_memory_bytes(1024);
    let snap = collector.snapshot();
    assert_eq!(snap.connections_count, 5);
    assert_eq!(snap.memory_bytes, 1024);
}

#[test]
fn unknown_operation_returns_none() {
    let collector = MetricsCollector::new();
    let snap = collector.snapshot();
    assert!(snap.avg_response_time_ms("unknown").is_none());
}
