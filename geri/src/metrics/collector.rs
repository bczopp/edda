//! Performance-Metrics-Collector (Phase 15.1.1): Response-Zeiten, Durchsatz, Latency.

/// Snapshot der aktuellen Metriken (Response-Zeiten, Request-Count).
#[derive(Debug, Clone, Default)]
pub struct MetricsSnapshot {
    request_count: u64,
    total_response_time_ms: u64,
    min_response_time_ms: Option<u64>,
    max_response_time_ms: Option<u64>,
}

impl MetricsSnapshot {
    /// Anzahl der erfassten Requests.
    pub fn request_count(&self) -> u64 {
        self.request_count
    }

    /// Summe aller Response-Zeiten in ms.
    pub fn total_response_time_ms(&self) -> u64 {
        self.total_response_time_ms
    }

    /// Minimale Response-Zeit in ms; None wenn keine Requests.
    pub fn min_response_time_ms(&self) -> Option<u64> {
        self.min_response_time_ms
    }

    /// Maximale Response-Zeit in ms; None wenn keine Requests.
    pub fn max_response_time_ms(&self) -> Option<u64> {
        self.max_response_time_ms
    }

    /// Durchschnittliche Response-Zeit in ms; None wenn keine Requests.
    pub fn average_response_time_ms(&self) -> Option<f64> {
        if self.request_count == 0 {
            return None;
        }
        Some(self.total_response_time_ms as f64 / self.request_count as f64)
    }
}

/// Sammelt Response-Zeiten und Request-Count für Durchsatz- und Latency-Metriken.
#[derive(Debug, Clone, Default)]
pub struct MetricsCollector {
    request_count: u64,
    total_response_time_ms: u64,
    min_response_time_ms: Option<u64>,
    max_response_time_ms: Option<u64>,
}

impl MetricsCollector {
    /// Erstellt einen leeren Collector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Erfasst eine Response mit der angegebenen Dauer in ms.
    pub fn record_response(&mut self, response_time_ms: u64) {
        self.request_count += 1;
        self.total_response_time_ms += response_time_ms;
        self.min_response_time_ms = Some(
            self.min_response_time_ms
                .map(|m| m.min(response_time_ms))
                .unwrap_or(response_time_ms),
        );
        self.max_response_time_ms = Some(
            self.max_response_time_ms
                .map(|m| m.max(response_time_ms))
                .unwrap_or(response_time_ms),
        );
    }

    /// Liefert einen Snapshot der aktuellen Metriken.
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            request_count: self.request_count,
            total_response_time_ms: self.total_response_time_ms,
            min_response_time_ms: self.min_response_time_ms,
            max_response_time_ms: self.max_response_time_ms,
        }
    }
}
