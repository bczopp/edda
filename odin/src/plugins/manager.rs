use crate::plugins::{GrpcPluginProxy, OdinGrpcProcessClient, OdinPlugin};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Registry of [`OdinPlugin`] instances; register/get/list by plugin name.
pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<String, Arc<dyn OdinPlugin>>>>,
}

impl PluginManager {
    /// Empty registry.
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a plugin; overwrites existing entry with same name.
    pub async fn register(&self, plugin: Arc<dyn OdinPlugin>) {
        let mut plugins = self.plugins.write().await;
        plugins.insert(plugin.name().to_string(), plugin);
    }

    /// Register a remote plugin by URL and capability labels.
    /// Capabilities are typically obtained via Einherjar Protocol (gRPC GetCapabilities).
    /// Fails if the URL is unreachable (e.g. connection refused).
    pub async fn register_remote_plugin(
        &self,
        name: String,
        base_url: &str,
        capabilities: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = OdinGrpcProcessClient::new(base_url).await?;
        let proxy = Arc::new(GrpcPluginProxy::new(name.clone(), capabilities, Arc::new(client)));
        self.register(proxy).await;
        Ok(())
    }

    /// Look up plugin by name.
    pub async fn get(&self, plugin_name: &str) -> Option<Arc<dyn OdinPlugin>> {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_name).cloned()
    }

    /// List registered plugin names.
    pub async fn list(&self) -> Vec<String> {
        let plugins = self.plugins.read().await;
        plugins.keys().cloned().collect()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
