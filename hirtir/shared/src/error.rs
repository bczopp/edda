use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedError {
    #[error("Data processing error: {0}")]
    DataProcessingError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}
