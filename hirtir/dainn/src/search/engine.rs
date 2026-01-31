use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Search failed: {0}")]
    SearchFailed(String),
}

pub struct SearchEngine;

impl SearchEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn search(&self, query: &str) -> Result<Vec<String>, SearchError> {
        // TODO: Implement Tantivy search
        Ok(vec![])
    }
}
