//! Anomaly Detector (Phase 15.2.1). Unusual connection patterns, anomaly score, alerts.

use std::collections::VecDeque;
use std::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum AnomalyEvent {
    Connect {
        connection_id: String,
        device_id: String,
    },
    Disconnect {
        connection_id: String,
    },
}

/// Alert when anomaly score exceeds threshold.
#[derive(Debug, Clone)]
pub struct AnomalyAlert {
    pub score: u8,
    pub message: String,
}

/// Detects unusual connection patterns; computes anomaly score; emits alerts above threshold.
pub struct AnomalyDetector {
    /// Max events to consider in window.
    window_size: usize,
    /// Time window for rate calculation.
    window_duration: Duration,
    /// Score above this triggers alert (0-100).
    alert_threshold: u8,
    /// Recent connect timestamps (for rate).
    connect_timestamps: RwLock<VecDeque<Instant>>,
}

impl AnomalyDetector {
    pub fn new(
        window_size: usize,
        window_duration: Duration,
        alert_threshold: u8,
    ) -> Self {
        Self {
            window_size: window_size.max(1),
            window_duration,
            alert_threshold: alert_threshold.min(100),
            connect_timestamps: RwLock::new(VecDeque::new()),
        }
    }

    pub fn record(&self, event: AnomalyEvent) {
        let now = Instant::now();
        match event {
            AnomalyEvent::Connect { .. } => {
                let mut q = self.connect_timestamps.write().unwrap();
                q.push_back(now);
                while q.len() > self.window_size {
                    q.pop_front();
                }
                while q.front().map(|t| now.duration_since(*t) > self.window_duration).unwrap_or(false) {
                    q.pop_front();
                }
            }
            AnomalyEvent::Disconnect { .. } => {}
        }
    }

    /// Returns anomaly score 0-100 (higher = more anomalous). Based on connect rate in window.
    /// Rate = events per second over the actual time span (first to last event) so that many
    /// connects in short real time yield a high score.
    pub fn get_anomaly_score(&self) -> u8 {
        let now = Instant::now();
        let mut q = self.connect_timestamps.write().unwrap();
        while q.front().map(|t| now.duration_since(*t) > self.window_duration).unwrap_or(false) {
            q.pop_front();
        }
        let count = q.len();
        if count <= 1 {
            return 0;
        }
        let first = *q.front().unwrap();
        let last = *q.back().unwrap();
        let span_secs = last.duration_since(first).as_secs_f64().max(0.001);
        let rate = count as f64 / span_secs;
        let max_reasonable = 10.0;
        let score = (rate / max_reasonable).min(1.0) * 100.0;
        score as u8
    }

    pub fn check_alert(&self) -> Option<AnomalyAlert> {
        let score = self.get_anomaly_score();
        if score >= self.alert_threshold {
            Some(AnomalyAlert {
                score,
                message: format!("anomaly score {} above threshold {}", score, self.alert_threshold),
            })
        } else {
            None
        }
    }
}
