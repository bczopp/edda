//! Model-Performance-Tracker (Phase 15.2.1): Tokens/s, Response-Zeiten, Verfügbarkeit pro Model.

use std::collections::HashMap;

/// Aggregierte Metriken pro Model (Response-Zeiten, Tokens, Verfügbarkeit).
#[derive(Debug, Clone, Default)]
pub struct ModelMetrics {
    request_count: u64,
    success_count: u64,
    total_response_time_ms: u64,
    total_tokens: u64,
}

impl ModelMetrics {
    /// Anzahl der erfassten Requests für dieses Model.
    pub fn request_count(&self) -> u64 {
        self.request_count
    }

    /// Anzahl der erfolgreichen Requests.
    pub fn success_count(&self) -> u64 {
        self.success_count
    }

    /// Summe aller Response-Zeiten in ms.
    pub fn total_response_time_ms(&self) -> u64 {
        self.total_response_time_ms
    }

    /// Summe aller generierten Tokens.
    pub fn total_tokens(&self) -> u64 {
        self.total_tokens
    }

    /// Verfügbarkeit (Anteil erfolgreicher Requests); 0.0–1.0.
    pub fn availability(&self) -> f64 {
        if self.request_count == 0 {
            return 1.0;
        }
        self.success_count as f64 / self.request_count as f64
    }

    /// Durchschnittliche Response-Zeit in ms; None wenn keine Requests.
    pub fn average_response_time_ms(&self) -> Option<f64> {
        if self.request_count == 0 {
            return None;
        }
        Some(self.total_response_time_ms as f64 / self.request_count as f64)
    }

    /// Durchschnittliche Tokens pro Sekunde (total_tokens / (total_time_s)); None wenn total_time_ms == 0.
    pub fn average_tokens_per_second(&self) -> Option<f64> {
        if self.total_response_time_ms == 0 {
            return None;
        }
        let total_time_s = self.total_response_time_ms as f64 / 1000.0;
        Some(self.total_tokens as f64 / total_time_s)
    }
}

/// Trackt pro Model: Tokens/s, Response-Zeiten, Verfügbarkeit.
#[derive(Debug, Clone, Default)]
pub struct ModelPerformanceTracker {
    per_model: HashMap<String, ModelMetrics>,
}

impl ModelPerformanceTracker {
    /// Erstellt einen leeren Tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Erfasst eine Response für das angegebene Model (response_time_ms, tokens_generated, success).
    pub fn record_response(
        &mut self,
        model_id: &str,
        response_time_ms: u64,
        tokens_generated: u32,
        success: bool,
    ) {
        let m = self.per_model.entry(model_id.to_string()).or_default();
        m.request_count += 1;
        if success {
            m.success_count += 1;
        }
        m.total_response_time_ms += response_time_ms;
        m.total_tokens += tokens_generated as u64;
    }

    /// Liefert die aggregierten Metriken für das Model; None wenn noch keine Einträge.
    pub fn get_model_metrics(&self, model_id: &str) -> Option<ModelMetrics> {
        self.per_model.get(model_id).cloned()
    }
}
