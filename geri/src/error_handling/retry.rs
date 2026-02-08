//! Retry-Manager (Phase 16.2.1): Exponential-Backoff, Max-Retry-Count, Retry-Delay.

use std::time::Duration;

/// Maximale Wartezeit pro Retry (Cap für Exponential-Backoff).
const MAX_DELAY_MS: u64 = 60_000;

/// Berechnet Retry-Delay (Exponential-Backoff) und prüft Max-Retry-Count.
#[derive(Debug, Clone)]
pub struct RetryManager {
    max_retries: u32,
    base_delay_ms: u64,
}

impl RetryManager {
    /// Erstellt einen Retry-Manager mit max. Anzahl Versuchen und Basis-Delay in ms.
    pub fn new(max_retries: u32, base_delay_ms: u64) -> Self {
        Self {
            max_retries,
            base_delay_ms: base_delay_ms.max(1),
        }
    }

    /// Gibt `true` zurück, wenn bei diesem Versuch (0-basiert) noch ein Retry erlaubt ist.
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_retries
    }

    /// Berechnet die Wartezeit für den angegebenen Versuch (0 = erster Retry); Exponential-Backoff, gecappt bei 60s.
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let exp = 2u64.saturating_pow(attempt.min(20));
        let ms = (self.base_delay_ms * exp).min(MAX_DELAY_MS);
        Duration::from_millis(ms)
    }
}
