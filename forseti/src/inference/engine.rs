use thiserror::Error;

#[derive(Debug, Error)]
pub enum InferenceError {
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
}

pub struct InferenceEngine;

impl InferenceEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_inference(&self, model_id: &str, input: &[u8]) -> Result<Vec<u8>, InferenceError> {
        // TODO: Run model inference
        Ok(vec![])
    }
}
