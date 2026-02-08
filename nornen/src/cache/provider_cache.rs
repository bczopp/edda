use crate::urd::registry::Provider;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Cache entry for provider queries
#[derive(Clone, Debug)]
struct CacheEntry {
    providers: Vec<Provider>,
    cached_at: SystemTime,
}

/// Cache key for provider queries
#[derive(Clone, Debug, PartialEq, Eq)]
struct CacheKey {
    capabilities: Vec<String>,
    status: Option<String>,
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash capabilities (sorted for consistency)
        let mut sorted_caps = self.capabilities.clone();
        sorted_caps.sort();
        sorted_caps.hash(state);
        self.status.hash(state);
    }
}

/// LRU-style cache for provider queries
pub struct ProviderCache {
    cache: Arc<RwLock<HashMap<CacheKey, CacheEntry>>>,
    max_size: usize,
    ttl_seconds: u64,
}

impl ProviderCache {
    /// Create a new ProviderCache
    pub fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            ttl_seconds,
        }
    }

    /// Get cached providers for a query
    pub async fn get(
        &self,
        capabilities: &[String],
        status: Option<&str>,
    ) -> Option<Vec<Provider>> {
        let key = CacheKey {
            capabilities: capabilities.to_vec(),
            status: status.map(|s| s.to_string()),
        };

        let cache_guard = self.cache.read().await;
        if let Some(entry) = cache_guard.get(&key) {
            // Check if entry is still valid (not expired)
            if entry.cached_at.elapsed().unwrap_or(Duration::from_secs(u64::MAX)).as_secs() < self.ttl_seconds {
                return Some(entry.providers.clone());
            }
        }
        None
    }

    /// Store providers in cache
    pub async fn set(
        &self,
        capabilities: &[String],
        status: Option<&str>,
        providers: Vec<Provider>,
    ) {
        let key = CacheKey {
            capabilities: capabilities.to_vec(),
            status: status.map(|s| s.to_string()),
        };

        let mut cache_guard = self.cache.write().await;

        // Evict old entries if cache is full
        if cache_guard.len() >= self.max_size && !cache_guard.contains_key(&key) {
            // Remove oldest entry (simple strategy: remove first entry)
            // In a real LRU, we'd track access order
            if let Some(oldest_key) = cache_guard.keys().next().cloned() {
                cache_guard.remove(&oldest_key);
            }
        }

        cache_guard.insert(
            key,
            CacheEntry {
                providers,
                cached_at: SystemTime::now(),
            },
        );
    }

    /// Invalidate cache entry for specific capabilities and status
    pub async fn invalidate(&self, capabilities: &[String], status: Option<&str>) {
        let key = CacheKey {
            capabilities: capabilities.to_vec(),
            status: status.map(|s| s.to_string()),
        };

        let mut cache_guard = self.cache.write().await;
        cache_guard.remove(&key);
    }

    /// Invalidate all cache entries (e.g., when provider is updated)
    pub async fn invalidate_all(&self) {
        let mut cache_guard = self.cache.write().await;
        cache_guard.clear();
    }

    /// Clean expired entries from cache
    pub async fn clean_expired(&self) {
        let mut cache_guard = self.cache.write().await;
        let now = SystemTime::now();
        
        cache_guard.retain(|_, entry| {
            entry.cached_at
                .elapsed()
                .map(|d| d.as_secs() < self.ttl_seconds)
                .unwrap_or(false)
        });
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache_guard = self.cache.read().await;
        let now = SystemTime::now();
        let mut expired_count = 0;
        
        for entry in cache_guard.values() {
            if entry.cached_at
                .elapsed()
                .map(|d| d.as_secs() >= self.ttl_seconds)
                .unwrap_or(true)
            {
                expired_count += 1;
            }
        }

        CacheStats {
            size: cache_guard.len(),
            max_size: self.max_size,
            expired_entries: expired_count,
        }
    }
}

pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub expired_entries: usize,
}
