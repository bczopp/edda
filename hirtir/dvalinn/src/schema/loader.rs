use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Schema loading failed: {0}")]
    LoadingFailed(String),
}

pub struct SchemaLoader;

impl SchemaLoader {
    pub fn new() -> Self {
        Self
    }

    pub async fn load_schema(&self, schema_id: &str) -> Result<String, SchemaError> {
        // TODO: Load schema from directory
        Ok("{}".to_string())
    }
}
