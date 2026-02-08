use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, trace};

#[derive(Debug, Clone)]
pub struct CacheEntry {
    data: Vec<u8>,
    inserted_at: Instant,
    last_accessed: Instant,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

pub struct CacheManager {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_size: usize,
    ttl: Duration,
    stats: Arc<RwLock<CacheStats>>,
}

impl CacheManager {
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            ttl,
            stats: Arc::new(RwLock::new(CacheStats {
                hits: 0,
                misses: 0,
                evictions: 0,
            })),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut cache = self.cache.write().await;
        
        // Check if entry exists
        if let Some(entry) = cache.get_mut(key) {
            // Check if expired
            if entry.inserted_at.elapsed() > self.ttl {
                trace!("Cache entry {} expired", key);
                cache.remove(key);
                let mut stats = self.stats.write().await;
                stats.misses += 1;
                return None;
            }
            
            // Update last accessed time (for LRU)
            entry.last_accessed = Instant::now();
            
            let mut stats = self.stats.write().await;
            stats.hits += 1;
            debug!("Cache hit for key: {}", key);
            Some(entry.data.clone())
        } else {
            let mut stats = self.stats.write().await;
            stats.misses += 1;
            trace!("Cache miss for key: {}", key);
            None
        }
    }

    pub async fn set(&self, key: String, value: Vec<u8>) {
        let mut cache = self.cache.write().await;
        
        // Check if we need to evict (LRU strategy)
        if cache.len() >= self.max_size && !cache.contains_key(&key) {
            // Find least recently used entry
            let lru_key = cache
                .iter()
                .min_by_key(|(_, entry)| entry.last_accessed)
                .map(|(k, _)| k.clone());
            
            if let Some(key_to_evict) = lru_key {
                cache.remove(&key_to_evict);
                let mut stats = self.stats.write().await;
                stats.evictions += 1;
                debug!("Evicted cache entry: {}", key_to_evict);
            }
        }
        
        // Insert or update entry
        cache.insert(
            key.clone(),
            CacheEntry {
                data: value,
                inserted_at: Instant::now(),
                last_accessed: Instant::now(),
            },
        );
        
        debug!("Cached entry: {}", key);
    }

    pub async fn invalidate(&self, key: &str) {
        let mut cache = self.cache.write().await;
        if cache.remove(key).is_some() {
            debug!("Invalidated cache entry: {}", key);
        }
    }

    pub async fn invalidate_user(&self, user_id: &str) {
        let mut cache = self.cache.write().await;
        // Cache keys are in format "user_id:data_id"
        let keys_to_remove: Vec<String> = cache
            .keys()
            .filter(|k| k.starts_with(&format!("{}:", user_id)))
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            cache.remove(&key);
            debug!("Invalidated cache entry for user {}: {}", user_id, key);
        }
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        debug!("Cache cleared");
    }

    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    pub async fn cleanup_expired(&self) -> usize {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        let expired_keys: Vec<String> = cache
            .iter()
            .filter(|(_, entry)| now.duration_since(entry.inserted_at) > self.ttl)
            .map(|(k, _)| k.clone())
            .collect();
        
        let count = expired_keys.len();
        for key in expired_keys {
            cache.remove(&key);
        }
        
        if count > 0 {
            debug!("Cleaned up {} expired cache entries", count);
        }
        
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic() {
        let cache = CacheManager::new(100, Duration::from_secs(60));
        
        // Cache miss
        assert_eq!(cache.get("key1").await, None);
        
        // Store
        cache.set("key1".to_string(), b"value1".to_vec()).await;
        
        // Cache hit
        assert_eq!(cache.get("key1").await, Some(b"value1".to_vec()));
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = CacheManager::new(100, Duration::from_millis(100));
        
        cache.set("key1".to_string(), b"value1".to_vec()).await;
        assert_eq!(cache.get("key1").await, Some(b"value1".to_vec()));
        
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert_eq!(cache.get("key1").await, None);
    }
}
