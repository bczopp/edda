//! Error Handler for Huginn

use tonic::Status;
use tracing::{error, warn, info};
use std::fmt;

/// Error types for Huginn
#[derive(Debug, Clone)]
pub enum HuginnError {
    AudioDeviceError(String),
    ServiceUnavailable(String),
    NetworkError(String),
    InvalidInput(String),
    InternalError(String),
}

impl fmt::Display for HuginnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HuginnError::AudioDeviceError(msg) => write!(f, "Audio device error: {}", msg),
            HuginnError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
            HuginnError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            HuginnError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            HuginnError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for HuginnError {}

/// Error Handler for Huginn services
pub struct ErrorHandler {
    // Future: Add retry configuration, fallback services, etc.
}

impl ErrorHandler {
    pub fn new() -> Result<Self, HuginnError> {
        info!("Creating ErrorHandler");
        Ok(Self {})
    }
    
    /// Handle audio device errors
    pub async fn handle_audio_device_error(&self, message: &str) -> Result<(), Status> {
        error!("Audio device error: {}", message);
        Err(Status::failed_precondition(format!("Audio device error: {}", message)))
    }
    
    /// Handle service unavailability
    pub async fn handle_service_unavailable(&self, service_name: &str) -> Result<(), Status> {
        warn!("Service unavailable: {}", service_name);
        Err(Status::unavailable(format!("Service {} is currently unavailable", service_name)))
    }
    
    /// Handle network errors
    pub async fn handle_network_error(&self, message: &str) -> Result<(), Status> {
        warn!("Network error: {}", message);
        // Determine if it's a timeout or connection error
        if message.contains("timeout") || message.contains("deadline") {
            Err(Status::deadline_exceeded(format!("Network timeout: {}", message)))
        } else {
            Err(Status::unavailable(format!("Network error: {}", message)))
        }
    }
    
    /// Handle invalid input errors
    pub async fn handle_invalid_input(&self, message: &str) -> Result<(), Status> {
        warn!("Invalid input: {}", message);
        Err(Status::invalid_argument(message))
    }
    
    /// Handle internal errors
    pub async fn handle_internal_error(&self, message: &str) -> Result<(), Status> {
        error!("Internal error: {}", message);
        Err(Status::internal(format!("Internal error: {}", message)))
    }
    
    /// Handle gRPC status errors
    pub async fn handle_grpc_status(&self, status: Status) -> Result<(), Status> {
        match status.code() {
            tonic::Code::Ok => Ok(()),
            tonic::Code::Cancelled => {
                warn!("Request cancelled");
                Err(status)
            }
            tonic::Code::Unknown => {
                error!("Unknown error: {}", status.message());
                Err(status)
            }
            tonic::Code::InvalidArgument => {
                warn!("Invalid argument: {}", status.message());
                Err(status)
            }
            tonic::Code::DeadlineExceeded => {
                warn!("Deadline exceeded: {}", status.message());
                Err(status)
            }
            tonic::Code::NotFound => {
                warn!("Not found: {}", status.message());
                Err(status)
            }
            tonic::Code::AlreadyExists => {
                warn!("Already exists: {}", status.message());
                Err(status)
            }
            tonic::Code::PermissionDenied => {
                warn!("Permission denied: {}", status.message());
                Err(status)
            }
            tonic::Code::ResourceExhausted => {
                warn!("Resource exhausted: {}", status.message());
                Err(status)
            }
            tonic::Code::FailedPrecondition => {
                warn!("Failed precondition: {}", status.message());
                Err(status)
            }
            tonic::Code::Aborted => {
                warn!("Aborted: {}", status.message());
                Err(status)
            }
            tonic::Code::OutOfRange => {
                warn!("Out of range: {}", status.message());
                Err(status)
            }
            tonic::Code::Unimplemented => {
                warn!("Unimplemented: {}", status.message());
                Err(status)
            }
            tonic::Code::Internal => {
                error!("Internal error: {}", status.message());
                Err(status)
            }
            tonic::Code::Unavailable => {
                warn!("Unavailable: {}", status.message());
                Err(status)
            }
            tonic::Code::DataLoss => {
                error!("Data loss: {}", status.message());
                Err(status)
            }
            tonic::Code::Unauthenticated => {
                warn!("Unauthenticated: {}", status.message());
                Err(status)
            }
        }
    }
    
    /// Convert HuginnError to gRPC Status
    pub fn huginn_error_to_status(&self, error: HuginnError) -> Status {
        match error {
            HuginnError::AudioDeviceError(msg) => {
                Status::failed_precondition(format!("Audio device error: {}", msg))
            }
            HuginnError::ServiceUnavailable(msg) => {
                Status::unavailable(format!("Service unavailable: {}", msg))
            }
            HuginnError::NetworkError(msg) => {
                if msg.contains("timeout") || msg.contains("deadline") {
                    Status::deadline_exceeded(format!("Network timeout: {}", msg))
                } else {
                    Status::unavailable(format!("Network error: {}", msg))
                }
            }
            HuginnError::InvalidInput(msg) => {
                Status::invalid_argument(msg)
            }
            HuginnError::InternalError(msg) => {
                Status::internal(format!("Internal error: {}", msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_huginn_error_to_status() {
        let handler = ErrorHandler::new().unwrap();
        
        let audio_error = HuginnError::AudioDeviceError("Device not found".to_string());
        let status = handler.huginn_error_to_status(audio_error);
        assert_eq!(status.code(), tonic::Code::FailedPrecondition);
        
        let service_error = HuginnError::ServiceUnavailable("STT service".to_string());
        let status = handler.huginn_error_to_status(service_error);
        assert_eq!(status.code(), tonic::Code::Unavailable);
        
        let network_error = HuginnError::NetworkError("Connection timeout".to_string());
        let status = handler.huginn_error_to_status(network_error);
        assert_eq!(status.code(), tonic::Code::DeadlineExceeded);
        
        let input_error = HuginnError::InvalidInput("Empty text".to_string());
        let status = handler.huginn_error_to_status(input_error);
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        
        let internal_error = HuginnError::InternalError("Unexpected error".to_string());
        let status = handler.huginn_error_to_status(internal_error);
        assert_eq!(status.code(), tonic::Code::Internal);
    }
}
