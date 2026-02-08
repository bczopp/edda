//! Script Manager for coordinating script execution, caching, and resource management

use crate::script_registry::ScriptRegistry;
use super::engine::{ScriptEngine, ScriptContext};
use crate::tools::config::ToolDefinition;
use shared::{LokiError, Result, ScriptDefinition};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tracing::{info, warn, error};

/// Cache entry for compiled/loaded scripts
struct ScriptCacheEntry {
    script_def: ScriptDefinition,
    last_used: std::time::Instant,
}

/// Script Manager coordinates script execution, caching, and resource management
pub struct ScriptManager {
    registry: Arc<ScriptRegistry>,
    engine: Arc<ScriptEngine>,
    cache: Arc<RwLock<HashMap<String, ScriptCacheEntry>>>,
    max_cache_size: usize,
}

impl ScriptManager {
    pub fn new(
        registry: Arc<ScriptRegistry>,
        engine: Arc<ScriptEngine>,
    ) -> Result<Self> {
        Ok(Self {
            registry,
            engine,
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_cache_size: 100, // Default cache size
        })
    }
    
    /// Execute a script by name
    pub async fn execute_script(
        &self,
        script_name: &str,
        context: ScriptContext,
    ) -> Result<String> {
        info!("Executing script: {}", script_name);
        
        // Get tool from registry
        let tool = self.registry.get_tool(script_name).await
            .ok_or_else(|| LokiError::ScriptNotFound(script_name.to_string()))?;
        
        // Check cache or load script
        let script_def = self.get_or_load_script(&tool).await?;
        
        // Execute script with resource limits
        let result = self.engine.execute(&script_def, context).await?;
        
        // Update cache last_used
        self.update_cache_usage(script_name).await;
        
        Ok(result)
    }
    
    /// Validate a script by name
    pub async fn validate_script(&self, script_name: &str) -> Result<()> {
        info!("Validating script: {}", script_name);
        
        let tool = self.registry.get_tool(script_name).await
            .ok_or_else(|| LokiError::ScriptNotFound(script_name.to_string()))?;
        
        self.engine.validate_script(&tool).await
    }
    
    /// Get or load script (with caching)
    async fn get_or_load_script(&self, tool: &ToolDefinition) -> Result<ScriptDefinition> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(&tool.name) {
                info!("Using cached script: {}", tool.name);
                return Ok(entry.script_def.clone());
            }
        }
        
        // Load script
        info!("Loading script: {}", tool.name);
        let script_def = self.engine.load_script(tool).await?;
        
        // Add to cache
        {
            let mut cache = self.cache.write().await;
            
            // Evict old entries if cache is full
            if cache.len() >= self.max_cache_size {
                self.evict_oldest(&mut cache).await;
            }
            
            cache.insert(tool.name.clone(), ScriptCacheEntry {
                script_def: script_def.clone(),
                last_used: std::time::Instant::now(),
            });
        }
        
        Ok(script_def)
    }
    
    /// Update cache usage timestamp
    async fn update_cache_usage(&self, script_name: &str) {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(script_name) {
            entry.last_used = std::time::Instant::now();
        }
    }
    
    /// Evict oldest cache entry
    async fn evict_oldest(&self, cache: &mut HashMap<String, ScriptCacheEntry>) {
        if let Some((oldest_key, _)) = cache.iter()
            .min_by_key(|(_, entry)| entry.last_used)
        {
            let key = oldest_key.clone();
            cache.remove(&key);
            info!("Evicted cached script: {}", key);
        }
    }
    
    /// Clear script cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("Script cache cleared");
    }

    /// Invalidate cache entry for a script (e.g. when script is updated).
    pub async fn invalidate_script(&self, name: &str) {
        let mut cache = self.cache.write().await;
        if cache.remove(name).is_some() {
            info!("Invalidated cached script: {}", name);
        }
    }
    
    /// Get cache statistics
    pub async fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.read().await;
        (cache.len(), self.max_cache_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_script_manager_cache_stats() {
        let registry = Arc::new(ScriptRegistry::new());
        let engine = Arc::new(ScriptEngine::new());
        
        let manager = ScriptManager::new(registry, engine).unwrap();
        let (current, max) = manager.cache_stats().await;
        
        assert_eq!(current, 0);
        assert_eq!(max, 100);
    }
}
