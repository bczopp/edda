//! Fenrir error type

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FenrirError {
    #[error("GPIO error: {0}")]
    Gpio(String),

    #[error("Sensor error: {0}")]
    Sensor(String),

    #[error("Actuator error: {0}")]
    Actuator(String),

    #[error("Hardware not available: {0}")]
    NotAvailable(String),
}

pub type Result<T> = std::result::Result<T, FenrirError>;
