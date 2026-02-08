//! Storage manager – key-value, serialization (Phase 8.3.1).
//! Implementation follows TDD; tests in hel/tests/storage_manager_test.rs.

use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

use crate::error::{HelError, Result};

/// In-memory key-value storage with optional persistence (Phase 8: simple, IoT-friendly).
pub struct StorageManager {
    store: RwLock<HashMap<String, String>>,
    base_path: Option<std::path::PathBuf>,
}

impl StorageManager {
    pub fn new_in_memory() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
            base_path: None,
        }
    }

    pub fn new_persistent(base_path: impl AsRef<Path>) -> Result<Self> {
        let base = base_path.as_ref().to_path_buf();
        std::fs::create_dir_all(&base).map_err(HelError::Filesystem)?;
        let path = base.join("storage.json");
        let store = if path.exists() {
            let s = std::fs::read_to_string(&path).map_err(HelError::Filesystem)?;
            serde_json::from_str(&s).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Ok(Self {
            store: RwLock::new(store),
            base_path: Some(path),
        })
    }

    fn persist(&self, store: &HashMap<String, String>) -> Result<()> {
        if let Some(ref p) = self.base_path {
            let s = serde_json::to_string_pretty(store).map_err(|e| HelError::Storage(e.to_string()))?;
            std::fs::write(p, s).map_err(HelError::Filesystem)?;
        }
        Ok(())
    }

    /// Get value by key.
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let store = self.store.read().map_err(|_| HelError::Storage("lock".into()))?;
        Ok(store.get(key).cloned())
    }

    /// Set key to value.
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut store = self.store.write().map_err(|_| HelError::Storage("lock".into()))?;
        store.insert(key.to_string(), value.to_string());
        self.persist(&store)
    }

    /// Remove key.
    pub fn remove(&self, key: &str) -> Result<Option<String>> {
        let mut store = self.store.write().map_err(|_| HelError::Storage("lock".into()))?;
        let v = store.remove(key);
        self.persist(&store)?;
        Ok(v)
    }

    /// List all keys.
    pub fn keys(&self) -> Result<Vec<String>> {
        let store = self.store.read().map_err(|_| HelError::Storage("lock".into()))?;
        let mut k: Vec<String> = store.keys().cloned().collect();
        k.sort();
        Ok(k)
    }
}
