//! Metrics Collector (Phase 14.2.1). Performance, connection quality, resource usage.
//! Performance Alert Manager (Phase 14.2.2). Threshold-based alerts.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Snapshot of current metrics (response times, throughput, quality, resource).
#[derive(Debug, Clone, Default)]
pub struct MetricsSnapshot {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub connections_count: u64,
    pub memory_bytes: u64,
    response_times: HashMap<String, (u64, u64)>,
    connection_quality: HashMap<String, (u64, u64)>,
}

impl MetricsSnapshot {
    pub fn avg_response_time_ms(&self, operation: &str) -> Option<u64> {
        self.response_times.get(operation).and_then(|(sum, n)| {
            if *n == 0 {
                None
            } else {
                Some(sum / n)
            }
        })
    }

    pub fn avg_connection_quality(&self, connection_id: &str) -> Option<u8> {
        self.connection_quality.get(connection_id).and_then(|(sum, n)| {
            if *n == 0 {
                None
            } else {
                Some((sum / n) as u8)
            }
        })
    }
}

/// Collects performance (response time, throughput), connection quality, resource usage.
pub struct MetricsCollector {
    messages_sent: AtomicU64,
    messages_received: AtomicU64,
    connections_count: AtomicU64,
    memory_bytes: AtomicU64,
    response_times: RwLock<HashMap<String, (u64, u64)>>,
    connection_quality: RwLock<HashMap<String, (u64, u64)>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            messages_sent: AtomicU64::new(0),
            messages_received: AtomicU64::new(0),
            connections_count: AtomicU64::new(0),
            memory_bytes: AtomicU64::new(0),
            response_times: RwLock::new(HashMap::new()),
            connection_quality: RwLock::new(HashMap::new()),
        }
    }

    pub fn record_response_time(&self, operation: &str, duration: Duration) {
        let ms = duration.as_millis() as u64;
        let mut map = self.response_times.write().unwrap();
        let entry = map.entry(operation.to_string()).or_insert((0, 0));
        entry.0 += ms;
        entry.1 += 1;
    }

    pub fn record_message_sent(&self) {
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_message_received(&self) {
        self.messages_received.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_connection_quality(&self, connection_id: &str, score: u8) {
        let mut map = self.connection_quality.write().unwrap();
        let entry = map.entry(connection_id.to_string()).or_insert((0, 0));
        entry.0 += score as u64;
        entry.1 += 1;
    }

    pub fn record_connections_count(&self, count: u64) {
        self.connections_count.store(count, Ordering::Relaxed);
    }

    pub fn record_memory_bytes(&self, bytes: u64) {
        self.memory_bytes.store(bytes, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            messages_sent: self.messages_sent.load(Ordering::Relaxed),
            messages_received: self.messages_received.load(Ordering::Relaxed),
            connections_count: self.connections_count.load(Ordering::Relaxed),
            memory_bytes: self.memory_bytes.load(Ordering::Relaxed),
            response_times: self.response_times.read().unwrap().clone(),
            connection_quality: self.connection_quality.read().unwrap().clone(),
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Alert for performance problems.
#[derive(Debug, Clone)]
pub struct PerformanceAlert {
    pub message: String,
    pub kind: AlertKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertKind {
    Latency,
    Connections,
}

/// Emits alerts when metrics exceed configured thresholds.
pub struct PerformanceAlertManager {
    metrics: Arc<MetricsCollector>,
    latency_threshold_ms: RwLock<HashMap<String, u64>>,
    connections_threshold: RwLock<Option<u64>>,
}

impl PerformanceAlertManager {
    pub fn new(metrics: Arc<MetricsCollector>) -> Self {
        Self {
            metrics,
            latency_threshold_ms: RwLock::new(HashMap::new()),
            connections_threshold: RwLock::new(None),
        }
    }

    pub fn set_latency_threshold_ms(&self, operation: &str, ms: u64) {
        self.latency_threshold_ms
            .write()
            .unwrap()
            .insert(operation.to_string(), ms);
    }

    pub fn set_connections_threshold(&self, count: u64) {
        *self.connections_threshold.write().unwrap() = Some(count);
    }

    pub fn check_alerts(&self) -> Vec<PerformanceAlert> {
        let mut alerts = Vec::new();
        let snap = self.metrics.snapshot();

        let thresholds = self.latency_threshold_ms.read().unwrap();
        for (op, &threshold_ms) in thresholds.iter() {
            if let Some(avg_ms) = snap.avg_response_time_ms(op) {
                if avg_ms > threshold_ms {
                    alerts.push(PerformanceAlert {
                        message: format!(
                            "latency for {} above threshold: avg {} ms > {} ms",
                            op, avg_ms, threshold_ms
                        ),
                        kind: AlertKind::Latency,
                    });
                }
            }
        }

        if let Some(threshold) = *self.connections_threshold.read().unwrap() {
            if snap.connections_count > threshold {
                alerts.push(PerformanceAlert {
                    message: format!(
                        "connections {} above threshold {}",
                        snap.connections_count, threshold
                    ),
                    kind: AlertKind::Connections,
                });
            }
        }

        alerts
    }
}
