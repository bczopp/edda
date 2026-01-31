use crate::embedding::{EmbeddingModel, EmbeddingError, SentenceTransformersModel};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Model registry for managing embedding models
pub struct ModelRegistry {
    models: Arc<RwLock<std::collections::HashMap<String, Arc<dyn EmbeddingModel>>>>,
    default_model: String,
}

impl ModelRegistry {
    pub fn new(default_model: String) -> Self {
        Self {
            models: Arc::new(RwLock::new(std::collections::HashMap::new())),
            default_model,
        }
    }

    /// Register an embedding model
    pub async fn register(&self, model: Arc<dyn EmbeddingModel>) {
        let mut models = self.models.write().await;
        models.insert(model.get_model_name().to_string(), model);
    }

    /// Get a model by name, or default if not found
    pub async fn get_model(&self, model_name: Option<&str>) -> Result<Arc<dyn EmbeddingModel>, EmbeddingError> {
        let models = self.models.read().await;
        let name = model_name.unwrap_or(&self.default_model);
        
        models.get(name)
            .cloned()
            .ok_or_else(|| EmbeddingError::ModelError(format!("Model not found: {}", name)))
    }

    /// Initialize default model
    pub async fn initialize_default(&self) -> Result<(), EmbeddingError> {
        let model = Arc::new(SentenceTransformersModel::new(&self.default_model).await?);
        self.register(model).await;
        Ok(())
    }

    /// Check if model is available
    pub async fn is_model_available(&self, model_name: &str) -> bool {
        let models = self.models.read().await;
        models.contains_key(model_name)
    }

    /// List all available models
    pub async fn list_models(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }
}
