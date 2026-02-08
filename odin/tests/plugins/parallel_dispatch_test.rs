//! Tests für parallele Plugin-Aufrufe (phase: parallel_agents für Valkyries/Plugins).

use odin::plugins::{PluginManager, OdinPlugin};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use async_trait::async_trait;

/// Mock-Plugin, das Aufrufe zählt (für Parallel-Test).
struct CountingPlugin {
    name: String,
    call_count: Arc<AtomicUsize>,
}

impl CountingPlugin {
    fn new(name: String, call_count: Arc<AtomicUsize>) -> Self {
        Self { name, call_count }
    }
}

#[async_trait]
impl OdinPlugin for CountingPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["test".to_string()]
    }

    async fn process_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        Ok(format!("Processed: {}", request))
    }
}

#[tokio::test]
async fn test_parallel_dispatch_spawns_multiple_calls() {
    let manager = PluginManager::new();
    let call_count = Arc::new(AtomicUsize::new(0));
    let plugin = Arc::new(CountingPlugin::new("test-plugin".to_string(), Arc::clone(&call_count)));
    manager.register(plugin).await;

    let results = manager
        .process_request_parallel("test-plugin", "test request", 3)
        .await
        .expect("parallel dispatch");

    assert_eq!(results.len(), 3, "should spawn 3 parallel calls");
    assert_eq!(call_count.load(Ordering::SeqCst), 3, "plugin called 3 times");
    for res in results {
        assert!(res.contains("Processed:"));
    }
}

#[tokio::test]
async fn test_parallel_dispatch_with_parallel_count_1() {
    let manager = PluginManager::new();
    let call_count = Arc::new(AtomicUsize::new(0));
    let plugin = Arc::new(CountingPlugin::new("test-plugin".to_string(), Arc::clone(&call_count)));
    manager.register(plugin).await;

    let results = manager
        .process_request_parallel("test-plugin", "test", 1)
        .await
        .expect("parallel dispatch");

    assert_eq!(results.len(), 1);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_parallel_dispatch_plugin_not_found() {
    let manager = PluginManager::new();
    let result = manager.process_request_parallel("nonexistent", "test", 2).await;
    assert!(result.is_err());
}
