//! Metrics Collector (Phase 15.2.1): Performance-Metriken, Query-Volumes.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Sammelt Performance-Metriken (Search-Zeit, Indexing-Zeit, Volumes).
pub struct MetricsCollector {
    indexing_count: AtomicU64,
    query_count: AtomicU64,
    indexing_time_ms_sum: AtomicU64,
    indexing_time_count: AtomicU64,
    search_time_ms_sum: AtomicU64,
    search_time_count: AtomicU64,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            indexing_count: AtomicU64::new(0),
            query_count: AtomicU64::new(0),
            indexing_time_ms_sum: AtomicU64::new(0),
            indexing_time_count: AtomicU64::new(0),
            search_time_ms_sum: AtomicU64::new(0),
            search_time_count: AtomicU64::new(0),
        }
    }

    pub fn record_indexing_time(&self, duration: Duration) {
        self.indexing_time_ms_sum
            .fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        self.indexing_time_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_search_time(&self, duration: Duration) {
        self.search_time_ms_sum
            .fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        self.search_time_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_indexing_count(&self) {
        self.indexing_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_query_count(&self) {
        self.query_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_indexing_count(&self) -> u64 {
        self.indexing_count.load(Ordering::Relaxed)
    }

    pub fn get_query_count(&self) -> u64 {
        self.query_count.load(Ordering::Relaxed)
    }

    pub fn get_avg_indexing_time_ms(&self) -> f64 {
        let sum = self.indexing_time_ms_sum.load(Ordering::Relaxed);
        let count = self.indexing_time_count.load(Ordering::Relaxed);
        if count == 0 {
            0.0
        } else {
            sum as f64 / count as f64
        }
    }

    pub fn get_avg_search_time_ms(&self) -> f64 {
        let sum = self.search_time_ms_sum.load(Ordering::Relaxed);
        let count = self.search_time_count.load(Ordering::Relaxed);
        if count == 0 {
            0.0
        } else {
            sum as f64 / count as f64
        }
    }
}
