//! TTS Cache Manager for caching frequently used TTS phrases

use shared::audio::AudioBuffer;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Cache entry with expiration timestamp
#[derive(Clone)]
struct CacheEntry {
    audio_buffer: AudioBuffer,
    expires_at: Instant,
    last_accessed: Instant,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub size: usize,
}

/// TTS Cache Manager for caching TTS audio results
pub struct TTSCacheManager {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_size: usize,
    ttl: Duration,
    stats: Arc<RwLock<CacheStats>>,
}

impl TTSCacheManager {
    /// Create a new TTS cache manager
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        info!("Creating TTSCacheManager with max_size={}, ttl={:?}", max_size, ttl);
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            ttl,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }
    
    /// Get max cache size
    pub fn max_size(&self) -> usize {
        self.max_size
    }
    
    /// Get TTL
    pub fn ttl(&self) -> Duration {
        self.ttl
    }
    
    /// Generate cache key from text, language, and voice
    pub fn generate_key(&self, text: &str, language: &str, voice: &str) -> String {
        format!("{}:{}:{}", text, language, voice)
    }
    
    /// Get cached audio buffer
    pub async fn get(&self, key: &str) -> Option<AudioBuffer> {
        let mut cache = self.cache.write().await;
        let mut stats = self.stats.write().await;
        
        if let Some(entry) = cache.get_mut(key) {
            // Check expiration
            if entry.expires_at < Instant::now() {
                debug!("Cache entry expired for key: {}", key);
                cache.remove(key);
                stats.misses += 1;
                return None;
            }
            
            // Update last accessed time
            entry.last_accessed = Instant::now();
            stats.hits += 1;
            debug!("Cache hit for key: {}", key);
            return Some(entry.audio_buffer.clone());
        }
        
        stats.misses += 1;
        debug!("Cache miss for key: {}", key);
        None
    }
    
    /// Set cached audio buffer
    pub async fn set(&self, key: &str, audio_buffer: AudioBuffer) {
        let mut cache = self.cache.write().await;
        
        // Check if we need to evict entries
        if cache.len() >= self.max_size && !cache.contains_key(key) {
            self.evict_oldest(&mut cache).await;
        }
        
        let expires_at = Instant::now() + self.ttl;
        let entry = CacheEntry {
            audio_buffer,
            expires_at,
            last_accessed: Instant::now(),
        };
        
        cache.insert(key.to_string(), entry);
        debug!("Cached entry for key: {}", key);
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.size = cache.len();
    }
    
    /// Evict oldest entry (LRU)
    async fn evict_oldest(&self, cache: &mut HashMap<String, CacheEntry>) {
        if cache.is_empty() {
            return;
        }
        
        let mut oldest_key: Option<String> = None;
        let mut oldest_time = Instant::now();
        
        for (key, entry) in cache.iter() {
            if entry.last_accessed < oldest_time {
                oldest_time = entry.last_accessed;
                oldest_key = Some(key.clone());
            }
        }
        
        if let Some(key) = oldest_key {
            cache.remove(&key);
            debug!("Evicted cache entry: {}", key);
        }
    }
    
    /// Clear all cache entries
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        let count = cache.len();
        cache.clear();
        info!("Cleared {} cache entries", count);
        
        // Reset stats
        let mut stats = self.stats.write().await;
        stats.size = 0;
    }
    
    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let mut stats = self.stats.write().await;
        stats.size = cache.len();
        stats.clone()
    }
    
    /// Clean expired entries
    pub async fn clean_expired(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        let initial_size = cache.len();
        
        cache.retain(|_key, entry| entry.expires_at >= now);
        
        let removed = initial_size - cache.len();
        if removed > 0 {
            debug!("Cleaned {} expired cache entries", removed);
        }
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.size = cache.len();
    }
}

impl Default for TTSCacheManager {
    fn default() -> Self {
        Self::new(100, Duration::from_secs(3600))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cache_entry_expiration() {
        let cache = TTSCacheManager::new(100, Duration::from_millis(100));
        let key = cache.generate_key("Test", "en-US", "male");
        
        let audio_buffer = AudioBuffer {
            samples: vec![0i16; 100],
            sample_rate: 44100,
            channels: 1,
            duration_ms: 10,
        };
        
        cache.set(&key, audio_buffer.clone()).await;
        assert!(cache.get(&key).await.is_some());
        
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert!(cache.get(&key).await.is_none());
    }
}
