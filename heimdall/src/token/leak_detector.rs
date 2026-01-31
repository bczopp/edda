//! Token leak detection: anomaly detection, device tracking, alerts.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

/// Alert when suspicious token usage (e.g. same token from multiple devices) is detected.
#[derive(Debug, Clone)]
pub struct LeakAlert {
    pub token_id: String,
    pub distinct_devices: u32,
    pub device_ids: Vec<String>,
    pub message: String,
}

/// In-memory store: token_id -> [(device_id, timestamp)].
struct UsageStore {
    entries: HashMap<String, Vec<(String, i64)>>,
    window_seconds: i64,
}

impl UsageStore {
    fn new(window_seconds: i64) -> Self {
        Self {
            entries: HashMap::new(),
            window_seconds,
        }
    }

    fn record(&mut self, token_id: &str, device_id: &str) {
        let now = Utc::now().timestamp();
        self.entries
            .entry(token_id.to_string())
            .or_default()
            .push((device_id.to_string(), now));
        self.prune(token_id, now);
    }

    fn prune(&mut self, token_id: &str, now: i64) {
        if let Some(entries) = self.entries.get_mut(token_id) {
            let cutoff = now - self.window_seconds;
            entries.retain(|(_, ts)| *ts >= cutoff);
        }
    }

    fn usage_by_device(&self, token_id: &str, now: i64) -> HashMap<String, u32> {
        let cutoff = now - self.window_seconds;
        let mut counts: HashMap<String, u32> = HashMap::new();
        if let Some(entries) = self.entries.get(token_id) {
            for (device_id, ts) in entries {
                if *ts >= cutoff {
                    *counts.entry(device_id.clone()).or_insert(0) += 1;
                }
            }
        }
        counts
    }

    fn distinct_devices_in_window(&self, token_id: &str, now: i64) -> (u32, Vec<String>) {
        let counts = self.usage_by_device(token_id, now);
        let ids: Vec<String> = counts.keys().cloned().collect();
        let n = ids.len() as u32;
        (n, ids)
    }
}

/// Detects token leak anomalies: same token used from multiple devices, device tracking.
#[allow(dead_code)]
pub struct TokenLeakDetector {
    store: Arc<RwLock<UsageStore>>,
    max_devices_per_token: u32,
    window_seconds: i64,
}

impl TokenLeakDetector {
    pub fn new(max_devices_per_token: u32, window_seconds: i64) -> Self {
        Self {
            store: Arc::new(RwLock::new(UsageStore::new(window_seconds))),
            max_devices_per_token,
            window_seconds,
        }
    }

    /// Record token usage from a device (call on each validation if desired).
    pub async fn record_usage(&self, token_id: &str, device_id: &str) {
        let _now = Utc::now().timestamp();
        let mut store = self.store.write().await;
        store.record(token_id, device_id);
    }

    /// Usage count per device for this token within the time window.
    pub async fn get_usage_by_device(&self, token_id: &str) -> HashMap<String, u32> {
        let now = Utc::now().timestamp();
        let store = self.store.read().await;
        store.usage_by_device(token_id, now)
    }

    /// Returns a leak alert if the token was used from more than max_devices_per_token in the window.
    pub async fn check_anomaly(&self, token_id: &str) -> Option<LeakAlert> {
        let now = Utc::now().timestamp();
        let store = self.store.read().await;
        let (distinct, device_ids) = store.distinct_devices_in_window(token_id, now);
        if distinct > self.max_devices_per_token {
            Some(LeakAlert {
                token_id: token_id.to_string(),
                distinct_devices: distinct,
                device_ids: device_ids.clone(),
                message: format!(
                    "Token used from {} distinct devices (max {}); possible leak",
                    distinct, self.max_devices_per_token
                ),
            })
        } else {
            None
        }
    }
}
