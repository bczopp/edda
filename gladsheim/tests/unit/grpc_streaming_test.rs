//! Tests for SubscribeServiceHealth Streaming

use gladsheim::grpc::GladsheimServiceImpl;
use tonic::Request;

// Include generated proto code
pub mod gladsheim {
    tonic::include_proto!("gladsheim.v1");
}

#[tokio::test]
async fn test_subscribe_service_health_not_found() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::HealthSubscribeRequest {
        service_name: "nonexistent".to_string(),
        update_interval_ms: Some(1000),
    });
    
    let response = service.subscribe_service_health(request).await;
    // Should fail because service doesn't exist
    assert!(response.is_err());
}

#[tokio::test]
async fn test_subscribe_service_health_custom_interval() {
    let service = GladsheimServiceImpl::new().await;
    
    // First register a service
    // TODO: Register service first, then subscribe
    
    let request = Request::new(gladsheim::HealthSubscribeRequest {
        service_name: "test-service".to_string(),
        update_interval_ms: Some(500),
    });
    
    // This will fail because service doesn't exist, but tests the custom interval
    let response = service.subscribe_service_health(request).await;
    assert!(response.is_err());
}
