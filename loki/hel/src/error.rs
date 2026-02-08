//! Hel error types (Phase 8).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HelError {
    #[error("Filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Cache error: {0}")]
    Cache(String),
    #[error("Not available: {0}")]
    NotAvailable(String),
}

pub type Result<T> = std::result::Result<T, HelError>;
