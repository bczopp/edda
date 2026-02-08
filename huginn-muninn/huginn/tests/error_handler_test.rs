//! Tests for Error Handler

use huginn::error_handler::ErrorHandler;
use tonic::Status;

#[tokio::test]
async fn test_error_handler_new() {
    let handler = ErrorHandler::new();
    assert!(handler.is_ok());
}

#[tokio::test]
async fn test_error_handler_handle_audio_device_error() {
    let handler = ErrorHandler::new().unwrap();
    let result = handler.handle_audio_device_error("Device not found").await;
    assert!(result.is_err());
    // Should return appropriate gRPC status
}

#[tokio::test]
async fn test_error_handler_handle_service_unavailable() {
    let handler = ErrorHandler::new().unwrap();
    let result = handler.handle_service_unavailable("STT service").await;
    assert!(result.is_err());
    // Should return UNAVAILABLE status
}

#[tokio::test]
async fn test_error_handler_handle_network_error() {
    let handler = ErrorHandler::new().unwrap();
    let result = handler.handle_network_error("Connection timeout").await;
    assert!(result.is_err());
    // Should return DEADLINE_EXCEEDED or UNAVAILABLE status
}

#[tokio::test]
async fn test_error_handler_handle_grpc_status() {
    let handler = ErrorHandler::new().unwrap();
    let status = Status::internal("Internal error");
    let result = handler.handle_grpc_status(status).await;
    assert!(result.is_err());
}
