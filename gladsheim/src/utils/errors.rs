//! Error types for Gladsheim

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GladsheimError {
    #[error("Service error: {0}")]
    ServiceError(String),
    
    #[error("Process error: {0}")]
    ProcessError(String),
    
    #[error("Resource error: {0}")]
    ResourceError(String),
    
    #[error("Health check error: {0}")]
    HealthCheckError(String),
    
    #[error("Registry error: {0}")]
    RegistryError(String),
    
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

pub type Result<T> = std::result::Result<T, GladsheimError>;
