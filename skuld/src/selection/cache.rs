//! Model-Selection-Cache (Phase 8): Caching der Modell-Auswahl pro Anforderungen (TTL optional).

use crate::selection::{ModelRequirements, ModelSelector};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

struct CachedEntry {
    model_id: String,
    created_at: Instant,
}

/// Cache für Ergebnisse von Model-Selection (gleiche Requirements → gleiches Modell ohne erneute Evaluation).
pub struct ModelSelectionCache {
    selector: std::sync::Arc<ModelSelector>,
    cache: Mutex<HashMap<String, CachedEntry>>,
    ttl_secs: Option<u64>,
}

impl ModelSelectionCache {
    pub fn new(selector: std::sync::Arc<ModelSelector>, ttl_secs: Option<u64>) -> Self {
        Self {
            selector,
            cache: Mutex::new(HashMap::new()),
            ttl_secs,
        }
    }

    /// Liefert das beste Modell für die Anforderungen; bei Cache-Treffer wird der Selector nicht aufgerufen.
    pub async fn select_best_model_cached(
        &self,
        requirements: ModelRequirements,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let key = requirements.cache_key();
        {
            let guard = self.cache.lock().map_err(|e| format!("cache lock: {}", e))?;
            if let Some(entry) = guard.get(&key) {
                let valid = match self.ttl_secs {
                    None => true,
                    Some(ttl) => entry.created_at.elapsed().as_secs() < ttl,
                };
                if valid {
                    return Ok(entry.model_id.clone());
                }
            }
        }
        let model_id = self.selector.select_best_model(requirements).await?;
        {
            let mut guard = self.cache.lock().map_err(|e| format!("cache lock: {}", e))?;
            guard.insert(
                key,
                CachedEntry {
                    model_id: model_id.clone(),
                    created_at: Instant::now(),
                },
            );
        }
        Ok(model_id)
    }

    /// Entfernt alle Einträge (z. B. nach Registry-Update).
    pub fn invalidate_all(&self) {
        let _ = self.cache.lock().map(|mut g| g.clear());
    }
}
