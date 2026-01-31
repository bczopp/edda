use crate::actions::executor::ActionExecutor;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ActionRegistry {
    executors: Arc<RwLock<HashMap<String, Arc<dyn ActionExecutor>>>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            executors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, executor: Arc<dyn ActionExecutor>) {
        let mut executors = self.executors.write().await;
        executors.insert(executor.action_type().to_string(), executor);
    }

    pub async fn get(&self, action_type: &str) -> Option<Arc<dyn ActionExecutor>> {
        let executors = self.executors.read().await;
        executors.get(action_type).cloned()
    }

    pub async fn list_types(&self) -> Vec<String> {
        let executors = self.executors.read().await;
        executors.keys().cloned().collect()
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
