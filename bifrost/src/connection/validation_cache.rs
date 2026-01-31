//! Validation Cache Manager (Phase 18.2.1). Cache connection validation results; TTL; invalidation on status updates.

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Cached validation outcome (from Heimdall / Connection Validation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationResult {
    Allowed,
    Denied,
}

/// Caches validation results with TTL; invalidate on status updates.
pub struct ValidationCacheManager {
    ttl: Duration,
    cache: RwLock<HashMap<String, (ValidationResult, Instant)>>,
}

impl ValidationCacheManager {
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Returns cached validation result if present and not expired; otherwise None.
    pub async fn get(&self, key: &str) -> Option<ValidationResult> {
        let now = Instant::now();
        let mut map = self.cache.write().unwrap();
        let (result, inserted_at) = map.get(key)?;
        if now.duration_since(*inserted_at) >= self.ttl {
            map.remove(key);
            return None;
        }
        Some(*result)
    }

    /// Caches validation result with TTL (e.g. after receiving ConnectionValidationResponse).
    pub async fn set(&self, key: &str, result: ValidationResult) {
        let now = Instant::now();
        self.cache
            .write()
            .unwrap()
            .insert(key.to_string(), (result, now));
    }

    /// Invalidates cache entry (e.g. on status update from Heimdall).
    pub async fn invalidate(&self, key: &str) {
        self.cache.write().unwrap().remove(key);
    }

    /// Invalidates all entries (e.g. on global status update).
    pub async fn invalidate_all(&self) {
        self.cache.write().unwrap().clear();
    }
}
