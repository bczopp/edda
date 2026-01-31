use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexingError {
    #[error("Indexing failed: {0}")]
    IndexingFailed(String),
}

pub struct IndexingService {
    index_path: String,
}

impl IndexingService {
    pub fn new(index_path: String) -> Self {
        Self { index_path }
    }

    pub async fn index(&self, data_id: &str, data: &[u8], metadata: &HashMap<String, String>) -> Result<String, IndexingError> {
        // TODO: Implement Tantivy indexing
        Ok(format!("index_{}", data_id))
    }
}
