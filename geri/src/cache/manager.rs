//! Cache-Manager (Phase 11.1.1): Response-Caching, Cache-Key (Prompt-Hash), TTL-Expiration.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

/// Cached-Eintrag: Response und Einfügezeit für TTL-Prüfung.
#[derive(Debug, Clone)]
struct CacheEntry {
    response: String,
    inserted_at: Instant,
}

/// Cached LLM-Responses für Prompts (Cache-Key = Hash des Prompts, TTL-basierte Expiration).
#[derive(Debug, Clone)]
pub struct CacheManager {
    entries: HashMap<u64, CacheEntry>,
    ttl: Duration,
}

impl CacheManager {
    /// Erstellt einen Cache mit der angegebenen TTL (z. B. 60s).
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            ttl,
        }
    }

    /// Liefert den Cache-Key für einen Prompt (deterministischer Hash).
    fn cache_key(prompt: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        prompt.hash(&mut hasher);
        hasher.finish()
    }

    /// Liefert die gecachte Response für den Prompt, oder None bei Miss/Expiration.
    pub fn get(&mut self, prompt: &str) -> Option<String> {
        let key = Self::cache_key(prompt);
        let entry = self.entries.get(&key)?;
        if entry.inserted_at.elapsed() >= self.ttl {
            self.entries.remove(&key);
            return None;
        }
        Some(entry.response.clone())
    }

    /// Speichert die Response für den Prompt (überschreibt bei gleichem Key).
    pub fn insert(&mut self, prompt: &str, response: String) {
        let key = Self::cache_key(prompt);
        self.entries.insert(
            key,
            CacheEntry {
                response,
                inserted_at: Instant::now(),
            },
        );
    }

    /// Entfernt alle Einträge (für Event-/Timeout-Invalidation).
    pub fn invalidate_all(&mut self) {
        self.entries.clear();
    }
}
