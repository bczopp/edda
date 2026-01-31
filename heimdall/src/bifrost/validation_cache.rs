//! Validation cache: connection validation results, TTL, invalidate on permission change.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

struct CacheEntry {
    value: bool,
    expires_at: i64,
}

/// Caches connection validation results (e.g. 5 min TTL); invalidate on permission/device change.
pub struct ValidationCacheManager {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl_seconds: i64,
}

impl ValidationCacheManager {
    pub fn new(ttl_seconds: i64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl_seconds,
        }
    }

    pub async fn get(&self, key: &str) -> Option<bool> {
        let cache = self.cache.read().await;
        let entry = cache.get(key)?;
        let now = Utc::now().timestamp();
        if now > entry.expires_at {
            drop(cache);
            let mut cache = self.cache.write().await;
            cache.remove(key);
            return None;
        }
        Some(entry.value)
    }

    pub async fn set(&self, key: &str, allowed: bool) {
        let expires_at = Utc::now().timestamp() + self.ttl_seconds;
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), CacheEntry { value: allowed, expires_at });
    }

    pub async fn invalidate(&self, key: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(key);
    }

    /// Invalidate all entries for a device (e.g. on permission change). Key format: "source:target".
    pub async fn invalidate_device(&self, device_id: &str) {
        let prefix = format!("{}:", device_id);
        let suffix = format!(":{}", device_id);
        let mut cache = self.cache.write().await;
        cache.retain(|key, _| !key.starts_with(&prefix) && !key.ends_with(&suffix));
    }
}
