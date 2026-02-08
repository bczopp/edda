//! Performance Test Suite (Phase 20.2.1).
//! Response-Time, Throughput, Streaming-Performance.

use geri::metrics::MetricsCollector;
use geri::streaming::{StreamingError, StreamingManager};
use std::time::Instant;

// --- Response-Time: lokale LLMs < 500 ms, Cloud < 2 s (simuliert) ---

#[test]
fn performance_response_time_local_simulated_under_500ms() {
    let mut collector = MetricsCollector::new();
    let simulated_local_ms = 50u64;
    for _ in 0..20 {
        collector.record_response(simulated_local_ms);
    }
    let snap = collector.snapshot();
    let avg = snap.average_response_time_ms().unwrap();
    assert!(avg < 500.0, "simulated local avg {} ms should be < 500", avg);
    assert_eq!(snap.request_count(), 20);
}

#[test]
fn performance_response_time_cloud_simulated_under_2s() {
    let mut collector = MetricsCollector::new();
    let simulated_cloud_ms = 800u64;
    for _ in 0..10 {
        collector.record_response(simulated_cloud_ms);
    }
    let snap = collector.snapshot();
    let avg = snap.average_response_time_ms().unwrap();
    assert!(avg < 2000.0, "simulated cloud avg {} ms should be < 2000", avg);
    assert_eq!(snap.request_count(), 10);
}

#[test]
fn performance_response_time_min_max_tracked() {
    let mut collector = MetricsCollector::new();
    collector.record_response(10);
    collector.record_response(100);
    collector.record_response(50);
    let snap = collector.snapshot();
    assert_eq!(snap.min_response_time_ms(), Some(10));
    assert_eq!(snap.max_response_time_ms(), Some(100));
    assert_eq!(snap.average_response_time_ms(), Some(160.0 / 3.0));
}

// --- Throughput: viele Requests in kurzer Zeit ---

#[test]
fn performance_throughput_many_requests_recorded() {
    let mut collector = MetricsCollector::new();
    let start = Instant::now();
    for i in 0..500u64 {
        collector.record_response(i % 100);
    }
    let elapsed_ms = start.elapsed().as_millis();
    let snap = collector.snapshot();
    assert_eq!(snap.request_count(), 500);
    assert!(elapsed_ms < 5000, "500 record_response should complete in < 5s");
}

#[test]
fn performance_throughput_average_latency_consistent() {
    let mut collector = MetricsCollector::new();
    for _ in 0..100 {
        collector.record_response(25);
    }
    let snap = collector.snapshot();
    assert_eq!(snap.average_response_time_ms(), Some(25.0));
    assert_eq!(snap.total_response_time_ms(), 2500);
}

// --- Streaming: viele Chunks schnell verarbeitet ---

#[test]
fn performance_streaming_many_chunks_collected() {
    let manager = StreamingManager;
    let chunks: Vec<Result<String, StreamingError>> = (0..1000)
        .map(|i| Ok(format!("chunk-{}", i)))
        .collect();
    let start = Instant::now();
    let out = manager.collect_chunks(chunks).unwrap();
    let elapsed_ms = start.elapsed().as_millis();
    assert!(out.contains("chunk-0"));
    assert!(out.contains("chunk-999"));
    assert!(elapsed_ms < 1000, "1000 chunks should collect in < 1s");
}

#[test]
fn performance_streaming_stops_on_first_error() {
    let manager = StreamingManager;
    let chunks: Vec<Result<String, StreamingError>> = vec![
        Ok("a".into()),
        Ok("b".into()),
        Err(StreamingError::StreamError("fail".into())),
        Ok("c".into()),
    ];
    let out = manager.collect_chunks(chunks);
    assert!(out.is_err());
}
