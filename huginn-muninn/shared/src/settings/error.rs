//! Settings error type for Huginn & Muninn

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Invalid settings: {0}")]
    Validation(String),

    #[error("Failed to load settings from {path}: {cause}")]
    Load { path: String, cause: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Watch error: {0}")]
    Watch(String),
}
