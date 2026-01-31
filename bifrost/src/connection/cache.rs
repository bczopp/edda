//! Connection Cache Manager (Phase 18.1.1). Cache connection info; TTL; invalidation on status updates.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Cached connection info (no stream).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionInfo {
    pub connection_id: String,
    pub device_id: String,
    pub user_id: String,
}

/// Caches connection information with TTL; invalidate on status updates.
pub struct ConnectionCacheManager {
    _connection_manager: Arc<super::ConnectionManager>,
    ttl: Duration,
    cache: RwLock<HashMap<String, (ConnectionInfo, Instant)>>,
}

impl ConnectionCacheManager {
    pub fn new(
        connection_manager: Arc<super::ConnectionManager>,
        ttl: Duration,
    ) -> Self {
        Self {
            _connection_manager: connection_manager,
            ttl,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Puts connection info into cache (e.g. after fetching from ConnectionManager or for tests).
    pub fn put_connection_info(&self, info: ConnectionInfo) {
        let now = Instant::now();
        self.cache
            .write()
            .unwrap()
            .insert(info.connection_id.clone(), (info, now));
    }

    /// Returns cached connection info if present and not expired; otherwise None.
    pub async fn get_connection_info(&self, connection_id: &str) -> Option<ConnectionInfo> {
        let now = Instant::now();
        let mut map = self.cache.write().unwrap();
        let entry = map.get(connection_id)?;
        let (info, inserted_at) = entry;
        if now.duration_since(*inserted_at) >= self.ttl {
            map.remove(connection_id);
            return None;
        }
        Some(info.clone())
    }

    /// Invalidates cache entry for connection (e.g. on disconnect).
    pub fn invalidate(&self, connection_id: &str) {
        self.cache.write().unwrap().remove(connection_id);
    }

    /// Clears entire cache (e.g. on bulk status update).
    pub fn invalidate_all(&self) {
        self.cache.write().unwrap().clear();
    }
}
