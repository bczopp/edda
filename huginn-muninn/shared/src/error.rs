//! Error types for audio processing

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("Audio processing error: {0}")]
    ProcessingError(String),
    
    #[error("Format conversion error: {0}")]
    FormatConversionError(String),
    
    #[error("Audio device error: {0}")]
    DeviceError(String),
    
    #[error("Audio I/O error: {0}")]
    IoError(String),
    
    #[error("Unsupported audio format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Invalid audio data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, AudioError>;
