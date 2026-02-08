//! Shared error types for Jotunheim.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum JotunheimError {
    #[error("Config error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
