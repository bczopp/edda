//! Connection Quality Monitor (Phase 9.4.1). Latency, packet-loss, quality score, degradation detection.
//! QualityBasedRouter (Phase 9.4.2): routing by quality, failover on degradation.

use crate::connection::ConnectionManager;
use crate::message::BifrostMessage;
use crate::routing::MessageRouter;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::Instant;
use thiserror::Error;

const MAX_SCORE: u8 = 100;

/// Snapshot of connection quality (avg latency, score).
#[derive(Debug, Clone)]
pub struct QualitySnapshot {
    pub avg_latency_ms: u64,
    pub score: u8,
    pub success_count: u32,
    pub failure_count: u32,
}

struct ConnectionState {
    latencies: std::collections::VecDeque<Duration>,
    max_latencies: usize,
    success_count: u32,
    failure_count: u32,
}

impl ConnectionState {
    fn new(max_latencies: usize) -> Self {
        Self {
            latencies: std::collections::VecDeque::new(),
            max_latencies: max_latencies.max(1),
            success_count: 0,
            failure_count: 0,
        }
    }

    fn add_latency(&mut self, d: Duration) {
        self.latencies.push_back(d);
        while self.latencies.len() > self.max_latencies {
            self.latencies.pop_front();
        }
    }

    fn avg_latency_ms(&self) -> u64 {
        if self.latencies.is_empty() {
            return 0;
        }
        let sum: u64 = self.latencies.iter().map(|d| d.as_millis() as u64).sum();
        (sum / self.latencies.len() as u64).min(u64::MAX)
    }

    fn score(&self, max_latency: Duration, _now: Instant) -> u8 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            return MAX_SCORE;
        }
        let loss_ratio = self.failure_count as f64 / total as f64;
        let latency_score = if self.latencies.is_empty() {
            1.0
        } else {
            let avg_ms = self.avg_latency_ms();
            let max_ms = max_latency.as_millis() as u64;
            if max_ms == 0 {
                1.0
            } else {
                (1.0 - (avg_ms as f64 / max_ms as f64).min(1.0)).max(0.0)
            }
        };
        let reliability = 1.0 - loss_ratio;
        let combined = (latency_score * 0.5 + reliability * 0.5).clamp(0.0, 1.0);
        (combined * MAX_SCORE as f64) as u8
    }
}

/// Collects latency and packet success/failure per connection; computes quality score; detects degradation.
pub struct ConnectionQualityMonitor {
    /// Max number of latency samples per connection.
    window_size: usize,
    /// Latency above this is penalized in score.
    max_latency: Duration,
    /// Score below this is considered degraded.
    degradation_threshold: u8,
    state: RwLock<HashMap<String, ConnectionState>>,
}

impl ConnectionQualityMonitor {
    pub fn new(
        window_size: usize,
        max_latency: Duration,
        degradation_threshold: u8,
    ) -> Self {
        Self {
            window_size: window_size.max(1),
            max_latency,
            degradation_threshold: degradation_threshold.min(100),
            state: RwLock::new(HashMap::new()),
        }
    }

    pub fn record_latency(&self, connection_id: &str, latency: Duration) {
        let mut map = self.state.write().unwrap();
        let st = map
            .entry(connection_id.to_string())
            .or_insert_with(|| ConnectionState::new(self.window_size));
        st.add_latency(latency);
    }

    pub fn record_success(&self, connection_id: &str) {
        let mut map = self.state.write().unwrap();
        let st = map
            .entry(connection_id.to_string())
            .or_insert_with(|| ConnectionState::new(self.window_size));
        st.success_count = st.success_count.saturating_add(1);
    }

    pub fn record_failure(&self, connection_id: &str) {
        let mut map = self.state.write().unwrap();
        let st = map
            .entry(connection_id.to_string())
            .or_insert_with(|| ConnectionState::new(self.window_size));
        st.failure_count = st.failure_count.saturating_add(1);
    }

    pub fn get_score(&self, connection_id: &str) -> Option<u8> {
        let map = self.state.read().unwrap();
        let st = map.get(connection_id)?;
        Some(st.score(self.max_latency, Instant::now()))
    }

    pub fn is_degraded(&self, connection_id: &str) -> bool {
        match self.get_score(connection_id) {
            None => true,
            Some(s) => s < self.degradation_threshold,
        }
    }

    pub fn snapshot(&self, connection_id: &str) -> Option<QualitySnapshot> {
        let map = self.state.read().unwrap();
        let st = map.get(connection_id)?;
        Some(QualitySnapshot {
            avg_latency_ms: st.avg_latency_ms(),
            score: st.score(self.max_latency, Instant::now()),
            success_count: st.success_count,
            failure_count: st.failure_count,
        })
    }
}

#[derive(Error, Debug)]
#[error("connection quality degraded")]
pub struct QualityDegradedError;

/// Provider for connection IDs by device (for quality-based routing; impl by ConnectionManager).
#[async_trait]
pub trait ConnectionListProvider: Send + Sync {
    async fn list_by_device(&self, device_id: &str) -> Vec<String>;
}

#[async_trait]
impl ConnectionListProvider for ConnectionManager {
    async fn list_by_device(&self, device_id: &str) -> Vec<String> {
        self.list_by_device(device_id).await
    }
}

/// Routes via direct when quality is ok; returns QualityDegradedError when all connections for target are degraded (caller can failover).
pub struct QualityBasedRouter {
    list_provider: Arc<dyn ConnectionListProvider>,
    router: MessageRouter,
    monitor: Arc<ConnectionQualityMonitor>,
}

impl QualityBasedRouter {
    pub fn new(
        list_provider: Arc<dyn ConnectionListProvider>,
        router: MessageRouter,
        monitor: Arc<ConnectionQualityMonitor>,
    ) -> Self {
        Self {
            list_provider,
            router,
            monitor,
        }
    }

    pub async fn route_message(
        &self,
        message: BifrostMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let conn_ids = self
            .list_provider
            .list_by_device(&message.target_device_id)
            .await;
        let all_degraded = !conn_ids.is_empty()
            && conn_ids
                .iter()
                .all(|id| self.monitor.is_degraded(id));
        if all_degraded {
            return Err(QualityDegradedError.into());
        }
        self.router.route_message(message).await
    }
}

/// Stub for tests: returns a fixed list of connection IDs per device.
pub struct StubConnectionListProvider {
    connections: std::sync::RwLock<HashMap<String, Vec<String>>>,
}

impl StubConnectionListProvider {
    pub fn new() -> Self {
        Self {
            connections: std::sync::RwLock::new(HashMap::new()),
        }
    }

    pub fn set_connections(&self, device_id: &str, ids: Vec<String>) {
        self.connections
            .write()
            .unwrap()
            .insert(device_id.to_string(), ids);
    }
}

impl Default for StubConnectionListProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ConnectionListProvider for StubConnectionListProvider {
    async fn list_by_device(&self, device_id: &str) -> Vec<String> {
        self.connections
            .read()
            .unwrap()
            .get(device_id)
            .cloned()
            .unwrap_or_default()
    }
}
