//! Cache-Invalidator (Phase 11.2.1): Event-basierte und Timeout-basierte Invalidation.

use std::time::{Duration, Instant};

use crate::cache::CacheManager;

/// Grund für die Cache-Invalidierung (Model-Update, Provider-Status-Änderung).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidationEvent {
    /// Model wurde aktualisiert.
    ModelUpdate,
    /// Provider-Status hat sich geändert.
    ProviderStatusChange,
}

/// Löst bei Events oder Timeout-Fallback die Invalidierung des Response-Caches aus.
#[derive(Debug, Clone)]
pub struct CacheInvalidator {
    timeout: Duration,
    last_invalidation: Option<Instant>,
}

impl CacheInvalidator {
    /// Erstellt einen Invalidator mit Timeout für Fallback-Invalidation (z. B. alle N Minuten).
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            last_invalidation: None,
        }
    }

    /// Invalidiert den Cache bei einem Event (Model-Update, Provider-Status-Änderung).
    pub fn invalidate_on_event(&mut self, cache: &mut CacheManager, _event: InvalidationEvent) {
        cache.invalidate_all();
        self.last_invalidation = Some(Instant::now());
    }

    /// Invalidiert den Cache, wenn seit letzter Invalidierung das Timeout abgelaufen ist (Fallback).
    /// Gibt `true` zurück, wenn invalidiert wurde.
    pub fn invalidate_on_timeout(&mut self, cache: &mut CacheManager) -> bool {
        let should_invalidate = self
            .last_invalidation
            .map_or(true, |t| t.elapsed() >= self.timeout);
        if should_invalidate {
            cache.invalidate_all();
            self.last_invalidation = Some(Instant::now());
            true
        } else {
            false
        }
    }
}
