//! Fehlertypen für Key-Storage.

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum KeyStorageError {
    #[error("Storage backend error: {0}")]
    Backend(String),
    #[error("Invalid UTF-8 in stored key")]
    InvalidUtf8,
}
