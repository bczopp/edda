//! Stub für Performance-Monitoring (z. B. Token-Validierungen).
//! Vollständiger MetricsCollector (Response-Zeiten, Durchsatz, Resource-Usage) optional später.

use std::sync::atomic::{AtomicU64, Ordering};

/// Stub: Zähler für Token-Validierungen (und ggf. weitere Metriken später).
#[derive(Debug, Default)]
pub struct MetricsCollector {
    token_validations: AtomicU64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Zählt eine Token-Validierung (Stub für spätere Integration in TokenValidator).
    pub fn record_token_validation(&self) {
        self.token_validations.fetch_add(1, Ordering::Relaxed);
    }

    /// Liefert die Anzahl der seit Start gezählten Token-Validierungen.
    pub fn get_token_validations(&self) -> u64 {
        self.token_validations.load(Ordering::Relaxed)
    }
}
