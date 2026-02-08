//! ErrorHandler – map LokiError and dyn Error to gRPC Status (Phase 10.1.1).

use shared::LokiError;
use tonic::{Code, Status};

/// Maps Loki and generic errors to gRPC Status for consistent API responses.
pub struct ErrorHandler;

impl ErrorHandler {
    /// Map LokiError to tonic Status with appropriate gRPC code.
    pub fn to_grpc_status(e: &LokiError) -> Status {
        let (code, msg) = match e {
            LokiError::ExecutionError(s) => (Code::Internal, s.clone()),
            LokiError::ScriptNotFound(s) => (Code::NotFound, s.clone()),
            LokiError::CompilationError(s) => (Code::InvalidArgument, s.clone()),
            LokiError::ResourceLimitExceeded(s) => (Code::ResourceExhausted, s.clone()),
            LokiError::InvalidScript(s) => (Code::InvalidArgument, s.clone()),
            LokiError::IoError(s) => (Code::Internal, s.clone()),
            LokiError::ServiceUnavailable(s) => (Code::Unavailable, s.clone()),
            LokiError::ConfigurationError(s) => (Code::FailedPrecondition, s.clone()),
        };
        Status::new(code, msg)
    }

    /// Map generic error to tonic Status (Internal).
    pub fn from_dyn_error(e: &(dyn std::error::Error + Send + Sync)) -> Status {
        Status::new(Code::Internal, e.to_string())
    }
}
