use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

pub struct CacheEntry<T> {
    value: T,
    expires_at: i64,
}

pub struct TokenValidationCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry<bool>>>>,
    ttl_seconds: i64,
}

impl TokenValidationCache {
    pub fn new(ttl_seconds: i64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl_seconds,
        }
    }

    pub async fn get(&self, token_id: &str) -> Option<bool> {
        let cache = self.cache.read().await;
        let entry = cache.get(token_id)?;
        
        let now = Utc::now().timestamp();
        if now > entry.expires_at {
            // Expired - remove and return None
            drop(cache);
            let mut cache = self.cache.write().await;
            cache.remove(token_id);
            return None;
        }
        
        Some(entry.value)
    }

    pub async fn set(&self, token_id: String, valid: bool) {
        let expires_at = Utc::now().timestamp() + self.ttl_seconds;
        let mut cache = self.cache.write().await;
        cache.insert(token_id, CacheEntry {
            value: valid,
            expires_at,
        });
    }

    pub async fn invalidate(&self, token_id: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(token_id);
    }

    pub async fn cleanup_expired(&self) {
        let now = Utc::now().timestamp();
        let mut cache = self.cache.write().await;
        cache.retain(|_, entry| entry.expires_at > now);
    }
}

pub struct PermissionCheckCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry<bool>>>>,
    ttl_seconds: i64,
}

impl PermissionCheckCache {
    pub fn new(ttl_seconds: i64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl_seconds,
        }
    }

    pub async fn get(&self, cache_key: &str) -> Option<bool> {
        let cache = self.cache.read().await;
        let entry = cache.get(cache_key)?;
        
        let now = Utc::now().timestamp();
        if now > entry.expires_at {
            drop(cache);
            let mut cache = self.cache.write().await;
            cache.remove(cache_key);
            return None;
        }
        
        Some(entry.value)
    }

    pub async fn set(&self, cache_key: String, allowed: bool) {
        let expires_at = Utc::now().timestamp() + self.ttl_seconds;
        let mut cache = self.cache.write().await;
        cache.insert(cache_key, CacheEntry {
            value: allowed,
            expires_at,
        });
    }

    pub async fn invalidate_device(&self, device_id: &str) {
        let mut cache = self.cache.write().await;
        cache.retain(|key, _| !key.starts_with(&format!("{}:", device_id)));
    }

    pub async fn cleanup_expired(&self) {
        let now = Utc::now().timestamp();
        let mut cache = self.cache.write().await;
        cache.retain(|_, entry| entry.expires_at > now);
    }
}
