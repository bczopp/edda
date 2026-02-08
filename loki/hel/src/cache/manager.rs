//! Cache manager – in-memory, TTL, invalidation (Phase 8.4.1).
//! Implementation follows TDD; tests in hel/tests/cache_manager_test.rs.

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use crate::error::{HelError, Result};

struct Entry {
    value: String,
    expires_at: Instant,
}

/// In-memory cache with TTL and invalidation.
pub struct CacheManager {
    entries: RwLock<HashMap<String, Entry>>,
    default_ttl: Duration,
}

impl CacheManager {
    pub fn new(default_ttl_secs: u64) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            default_ttl: Duration::from_secs(default_ttl_secs),
        }
    }

    fn is_expired(entry: &Entry) -> bool {
        Instant::now() >= entry.expires_at
    }

    /// Get value; returns None if missing or expired.
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let mut entries = self.entries.write().map_err(|_| HelError::Cache("lock".into()))?;
        let v = entries.get(key).and_then(|e| {
            if Self::is_expired(e) {
                None
            } else {
                Some(e.value.clone())
            }
        });
        if v.is_none() {
            entries.remove(key);
        }
        Ok(v)
    }

    /// Set value with default TTL.
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        self.set_with_ttl(key, value, self.default_ttl)
    }

    /// Set value with custom TTL.
    pub fn set_with_ttl(&self, key: &str, value: &str, ttl: Duration) -> Result<()> {
        let mut entries = self.entries.write().map_err(|_| HelError::Cache("lock".into()))?;
        entries.insert(
            key.to_string(),
            Entry {
                value: value.to_string(),
                expires_at: Instant::now() + ttl,
            },
        );
        Ok(())
    }

    /// Invalidate key.
    pub fn invalidate(&self, key: &str) -> Result<()> {
        let mut entries = self.entries.write().map_err(|_| HelError::Cache("lock".into()))?;
        entries.remove(key);
        Ok(())
    }

    /// Invalidate all.
    pub fn invalidate_all(&self) -> Result<()> {
        let mut entries = self.entries.write().map_err(|_| HelError::Cache("lock".into()))?;
        entries.clear();
        Ok(())
    }
}
