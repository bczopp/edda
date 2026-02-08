//! Error types for Loki services

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LokiError {
    #[error("Script execution error: {0}")]
    ExecutionError(String),
    
    #[error("Script not found: {0}")]
    ScriptNotFound(String),
    
    #[error("Script compilation error: {0}")]
    CompilationError(String),
    
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    
    #[error("Invalid script: {0}")]
    InvalidScript(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

pub type Result<T> = std::result::Result<T, LokiError>;
