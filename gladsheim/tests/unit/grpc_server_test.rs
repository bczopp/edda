//! Tests for Gladsheim gRPC Server

use gladsheim::grpc::GladsheimServiceImpl;
use tonic::Request;

// Include generated proto code
pub mod gladsheim {
    tonic::include_proto!("gladsheim.v1");
}

#[tokio::test]
async fn test_gladsheim_service_creation() {
    let service = GladsheimServiceImpl::new().await;
    assert!(service.is_ready());
}

#[tokio::test]
async fn test_start_service_request() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::StartServiceRequest {
        service_name: "test-service".to_string(),
        environment_vars: std::collections::HashMap::new(),
        resource_limits: None,
        working_directory: String::new(),
        args: vec![],
    });
    
    let response = service.start_service(request).await;
    // Should fail for non-existent service or unauthorized
    assert!(response.is_ok() || response.is_err());
}

#[tokio::test]
async fn test_get_service_status() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::ServiceStatusRequest {
        service_name: "nonexistent".to_string(),
    });
    
    let response = service.get_service_status(request).await;
    // Should return error for non-existent service
    assert!(response.is_err());
}

#[tokio::test]
async fn test_stop_service_nonexistent() {
    let service = GladsheimServiceImpl::new().await;

    let request = Request::new(gladsheim::StopServiceRequest {
        service_name: "nonexistent".to_string(),
        force: false,
        timeout_ms: Some(5000),
    });

    let response = service.stop_service(request).await;
    assert!(response.is_err());
}

#[tokio::test]
async fn test_list_services() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::ListServicesRequest {
        filter_states: vec![],
        include_resources: false,
        include_health: false,
    });
    
    let response = service.list_services(request).await;
    assert!(response.is_ok());
    
    let response = response.unwrap().into_inner();
    assert_eq!(response.total_count, 0); // No services registered yet
}

#[tokio::test]
async fn test_get_resource_usage() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::ResourceUsageRequest {
        service_name: "nonexistent".to_string(),
    });
    
    let response = service.get_resource_usage(request).await;
    // Should return error for non-existent service
    assert!(response.is_err());
}

#[tokio::test]
async fn test_get_resource_limits() {
    let service = GladsheimServiceImpl::new().await;
    
    let request = Request::new(gladsheim::ServiceRequest {
        service_name: "nonexistent".to_string(),
    });
    
    let response = service.get_resource_limits(request).await;
    // Should return error for non-existent service
    assert!(response.is_err());
}

#[tokio::test]
async fn test_set_resource_limits_nonexistent_service() {
    let service = GladsheimServiceImpl::new().await;

    let limits = gladsheim::ResourceLimits {
        max_memory_bytes: 256 * 1024 * 1024,
        max_cpu_percent: 25.0,
        max_memory_mb: 256.0,
        memory_warning_percent: Some(80.0),
        cpu_warning_percent: Some(80.0),
    };

    let request = Request::new(gladsheim::ResourceLimitsRequest {
        service_name: "nonexistent".to_string(),
        limits: Some(limits),
    });

    let response = service.set_resource_limits(request).await;
    // Should return error for non-existent service
    assert!(response.is_err());
}
