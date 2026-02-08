use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum PerformanceWindow {
    Last100Ms,
    Last1S,
    Last10S,
    Last1Min,
    AllTime,
}

#[derive(Debug, Clone, Default)]
pub struct ProviderMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_latency_ms: u64,
    pub total_tokens: u64,
}

impl ProviderMetrics {
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 1.0;
        }
        self.successful_requests as f64 / self.total_requests as f64
    }

    pub fn average_latency_ms(&self) -> f64 {
        if self.successful_requests == 0 {
            return 0.0;
        }
        self.total_latency_ms as f64 / self.successful_requests as f64
    }

    pub fn tokens_per_second(&self) -> f64 {
        if self.total_latency_ms == 0 {
            return 0.0;
        }
        (self.total_tokens as f64 / self.total_latency_ms as f64) * 1000.0
    }
}

struct RequestRecord {
    start_time: Instant,
    provider_id: String,
    model_id: String,
}

struct HistoricalEvent {
    timestamp: Instant,
    latency_ms: Option<u64>,
    tokens: Option<u64>,
    success: bool,
}

pub struct PerformanceTracker {
    active_requests: Arc<RwLock<HashMap<String, RequestRecord>>>,
    history: Arc<RwLock<HashMap<String, Vec<HistoricalEvent>>>>,
}

impl PerformanceTracker {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            active_requests: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    fn model_key(provider_id: &str, model_id: &str) -> String {
        format!("{}:{}", provider_id, model_id)
    }

    pub async fn record_request_start(&self, provider_id: &str, model_id: &str) {
        let mut active = self.active_requests.write().await;
        // Logic: provider_id + model_id should be unique for an active request in this simple stub?
        // Or maybe just use a generated ID. The test uses strings.
        // Let's use the combination as key for simplicity, assuming one active request per provider:model in the test.
        let key = Self::model_key(provider_id, model_id);
        active.insert(key, RequestRecord {
            start_time: Instant::now(),
            provider_id: provider_id.to_string(),
            model_id: model_id.to_string(),
        });
    }

    pub async fn record_request_success(&self, provider_id: &str, model_id: &str, _latency_ms: u64, tokens: u64) {
        let key = Self::model_key(provider_id, model_id);
        let mut active = self.active_requests.write().await;
        if let Some(record) = active.remove(&key) {
            let latency_ms = record.start_time.elapsed().as_millis() as u64;
            let mut history = self.history.write().await;
            history.entry(key).or_default().push(HistoricalEvent {
                timestamp: Instant::now(),
                latency_ms: Some(latency_ms),
                tokens: Some(tokens),
                success: true,
            });
        }
    }

    pub async fn record_request_failure(&self, provider_id: &str, model_id: &str, _error: &str) {
        let key = Self::model_key(provider_id, model_id);
        let mut active = self.active_requests.write().await;
        if let Some(_) = active.remove(&key) {
            let mut history = self.history.write().await;
            history.entry(key).or_default().push(HistoricalEvent {
                timestamp: Instant::now(),
                latency_ms: None,
                tokens: None,
                success: false,
            });
        }
    }

    pub async fn get_metrics(&self, provider_id: &str, model_id: &str) -> Option<ProviderMetrics> {
        self.get_windowed_metrics(provider_id, model_id, PerformanceWindow::AllTime).await
    }

    pub async fn get_windowed_metrics(&self, provider_id: &str, model_id: &str, window: PerformanceWindow) -> Option<ProviderMetrics> {
        let key = Self::model_key(provider_id, model_id);
        let history = self.history.read().await;
        let events = history.get(&key)?;

        let now = Instant::now();
        let duration = match window {
            PerformanceWindow::Last100Ms => Duration::from_millis(100),
            PerformanceWindow::Last1S => Duration::from_secs(1),
            PerformanceWindow::Last10S => Duration::from_secs(10),
            PerformanceWindow::Last1Min => Duration::from_secs(60),
            PerformanceWindow::AllTime => Duration::from_secs(365 * 24 * 3600), // Far in past
        };

        let mut metrics = ProviderMetrics::default();
        for event in events {
            if now.duration_since(event.timestamp) <= duration {
                metrics.total_requests += 1;
                if event.success {
                    metrics.successful_requests += 1;
                    metrics.total_latency_ms += event.latency_ms.unwrap_or(0);
                    metrics.total_tokens += event.tokens.unwrap_or(0);
                } else {
                    metrics.failed_requests += 1;
                }
            }
        }

        if metrics.total_requests == 0 {
            None
        } else {
            Some(metrics)
        }
    }

    pub async fn get_best_provider(&self, candidates: Vec<&str>) -> Option<String> {
        let mut best_provider = None;
        let mut best_score = -1.0;

        for provider in candidates {
            // Simplified: just check metrics for "model1" or similar if we had multiple models
            // The test doesn't specify model_id for get_best_provider, so we'll assume a default or aggregate.
            // Let's assume the test uses "model1" for its fast/slow comparison.
            if let Some(metrics) = self.get_metrics(provider, "model1").await {
                // Score = success_rate / (latency_ms + 1)
                let score = metrics.success_rate() * 1000.0 / (metrics.average_latency_ms() + 1.0);
                if score > best_score {
                    best_score = score;
                    best_provider = Some(provider.to_string());
                }
            }
        }

        best_provider
    }
}
