//! Tests for Phase 9.4.1: ConnectionQualityMonitor (latency, packet-loss, score, degradation).

use bifrost::routing::ConnectionQualityMonitor;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn no_data_returns_none_score() {
    let monitor = ConnectionQualityMonitor::new(10, Duration::from_millis(500), 50);
    let score = monitor.get_score("conn-1");
    assert!(score.is_none());
}

#[test]
fn low_latency_high_score() {
    let monitor = Arc::new(ConnectionQualityMonitor::new(10, Duration::from_millis(500), 50));
    monitor.record_latency("conn-1", Duration::from_millis(10));
    monitor.record_success("conn-1");
    monitor.record_success("conn-1");
    let score = monitor.get_score("conn-1").unwrap();
    assert!(score >= 70);
}

#[test]
fn high_latency_low_score() {
    let monitor = Arc::new(ConnectionQualityMonitor::new(10, Duration::from_millis(100), 50));
    monitor.record_latency("conn-1", Duration::from_millis(200));
    monitor.record_success("conn-1");
    let score = monitor.get_score("conn-1").unwrap();
    assert!(score < 70);
}

#[test]
fn packet_loss_reduces_score() {
    let monitor = Arc::new(ConnectionQualityMonitor::new(10, Duration::from_millis(500), 50));
    for _ in 0..5 {
        monitor.record_success("conn-1");
    }
    for _ in 0..5 {
        monitor.record_failure("conn-1");
    }
    let score = monitor.get_score("conn-1").unwrap();
    assert!(score < 70);
}

#[test]
fn is_degraded_true_when_score_below_threshold() {
    let monitor = Arc::new(ConnectionQualityMonitor::new(10, Duration::from_millis(100), 50));
    monitor.record_latency("conn-1", Duration::from_millis(300));
    for _ in 0..3 {
        monitor.record_failure("conn-1");
    }
    assert!(monitor.is_degraded("conn-1"));
}

#[test]
fn snapshot_contains_latency_and_score() {
    let monitor = Arc::new(ConnectionQualityMonitor::new(10, Duration::from_millis(500), 50));
    monitor.record_latency("conn-1", Duration::from_millis(20));
    monitor.record_success("conn-1");
    let snap = monitor.snapshot("conn-1").unwrap();
    assert!(snap.avg_latency_ms > 0);
    assert!(snap.score <= 100);
}
