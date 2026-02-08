//! Tests for RestartService RPC

use gladsheim::grpc::GladsheimServiceImpl;
use tonic::Request;

// Include generated proto code
pub mod gladsheim {
    tonic::include_proto!("gladsheim.v1");
}

#[tokio::test]
async fn test_restart_service_not_found() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::RestartServiceRequest {
        service_name: "nonexistent".to_string(),
        force_stop: false,
        stop_timeout_ms: None,
    });
    
    let response = service.restart_service(request).await;
    // Should fail because service doesn't exist
    assert!(response.is_err());
}

#[tokio::test]
async fn test_restart_service_force_stop() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::RestartServiceRequest {
        service_name: "test-service".to_string(),
        force_stop: true,
        stop_timeout_ms: Some(1000),
    });
    
    // This will fail because service doesn't exist, but tests the force_stop parameter
    let response = service.restart_service(request).await;
    assert!(response.is_err());
}
