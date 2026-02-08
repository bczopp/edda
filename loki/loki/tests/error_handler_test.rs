//! Tests for ErrorHandler (TDD – Phase 10.1.1).

use loki::error_handler::ErrorHandler;
use shared::LokiError;
use tonic::Code;

#[test]
fn error_handler_execution_error_maps_to_internal() {
    let e = LokiError::ExecutionError("lua failed".into());
    let status = ErrorHandler::to_grpc_status(&e);
    assert_eq!(status.code(), Code::Internal);
    assert!(status.message().contains("lua failed"));
}

#[test]
fn error_handler_script_not_found_maps_to_not_found() {
    let e = LokiError::ScriptNotFound("x".into());
    let status = ErrorHandler::to_grpc_status(&e);
    assert_eq!(status.code(), Code::NotFound);
}

#[test]
fn error_handler_resource_limit_maps_to_resource_exhausted() {
    let e = LokiError::ResourceLimitExceeded("timeout".into());
    let status = ErrorHandler::to_grpc_status(&e);
    assert_eq!(status.code(), Code::ResourceExhausted);
}

#[test]
fn error_handler_invalid_script_maps_to_invalid_argument() {
    let e = LokiError::InvalidScript("bad".into());
    let status = ErrorHandler::to_grpc_status(&e);
    assert_eq!(status.code(), Code::InvalidArgument);
}

#[test]
fn error_handler_service_unavailable_maps_to_unavailable() {
    let e = LokiError::ServiceUnavailable("down".into());
    let status = ErrorHandler::to_grpc_status(&e);
    assert_eq!(status.code(), Code::Unavailable);
}

#[test]
fn error_handler_from_dyn_error_maps_to_internal() {
    let e: Box<dyn std::error::Error + Send + Sync> = Box::new(std::io::Error::new(std::io::ErrorKind::Other, "io"));
    let status = ErrorHandler::from_dyn_error(&*e);
    assert_eq!(status.code(), Code::Internal);
}
