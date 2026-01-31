use async_trait::async_trait;

/// Interface for Odin plugins (compile-time, per IMPLEMENTATION_PLAN).
#[async_trait]
pub trait OdinPlugin: Send + Sync {
    /// Plugin identifier; used as key in [`PluginManager`](crate::plugins::PluginManager).
    fn name(&self) -> &str;
    /// Capability labels exposed to the orchestrator.
    fn capabilities(&self) -> Vec<String>;
    /// Process a request string; returns response or error.
    async fn process_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>>;
}
