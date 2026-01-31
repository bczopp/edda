use crate::urd::registry::Provider;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CachedProvider {
    provider: Provider,
    cached_at: Instant,
}

pub struct ProviderCache {
    cache: Arc<RwLock<HashMap<String, CachedProvider>>>,
    ttl: Duration,
}

impl ProviderCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub async fn get(&self, provider_id: &str) -> Option<Provider> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(provider_id) {
            if cached.cached_at.elapsed() < self.ttl {
                return Some(cached.provider.clone());
            }
        }
        None
    }

    pub async fn set(&self, provider_id: String, provider: Provider) {
        let mut cache = self.cache.write().await;
        cache.insert(provider_id, CachedProvider {
            provider,
            cached_at: Instant::now(),
        });
    }

    pub async fn invalidate(&self, provider_id: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(provider_id);
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}
