use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod gladsheim {
    tonic::include_proto!("gladsheim.v1");
}

use gladsheim::gladsheim_service_server::{GladsheimService, GladsheimServiceServer};

pub struct GladsheimServiceImpl {
    service_loader: Arc<crate::thjalfi::ServiceLoader>,
    resource_manager: Arc<crate::byggvir::ResourceManager>,
    health_monitor: Arc<crate::roskva::HealthMonitor>,
    service_registry: Arc<crate::skirnir::ServiceRegistry>,
}

impl GladsheimServiceImpl {
    pub fn new(
        service_loader: Arc<crate::thjalfi::ServiceLoader>,
        resource_manager: Arc<crate::byggvir::ResourceManager>,
        health_monitor: Arc<crate::roskva::HealthMonitor>,
        service_registry: Arc<crate::skirnir::ServiceRegistry>,
    ) -> Self {
        Self {
            service_loader,
            resource_manager,
            health_monitor,
            service_registry,
        }
    }
}

#[tonic::async_trait]
impl GladsheimService for GladsheimServiceImpl {
    async fn start_service(
        &self,
        request: Request<gladsheim::StartServiceRequest>,
    ) -> Result<Response<gladsheim::ServiceStatus>, Status> {
        let req = request.into_inner();
        
        // Load and start service
        self.service_loader.load_service(&req.service_name).await
            .map_err(|e| Status::internal(format!("Service loading failed: {}", e)))?;
        
        // Allocate resources
        self.resource_manager.allocate_resources(&req.service_name).await
            .map_err(|e| Status::internal(format!("Resource allocation failed: {}", e)))?;
        
        // Register service
        self.service_registry.register(crate::skirnir::ServiceInfo {
            service_name: req.service_name.clone(),
            status: "running".to_string(),
            health: "healthy".to_string(),
        }).await;
        
        Ok(Response::new(gladsheim::ServiceStatus {
            service_name: req.service_name,
            state: gladsheim::ServiceState::SERVICE_STATE_RUNNING as i32,
            process_id: 12345, // TODO: Get actual PID
            start_time_unix: chrono::Utc::now().timestamp(),
            stop_time_unix: 0,
            resource_usage: None,
            health_status: None,
            error_message: None,
            restart_count: 0,
        }))
    }

    async fn stop_service(
        &self,
        request: Request<gladsheim::StopServiceRequest>,
    ) -> Result<Response<gladsheim::ServiceStatus>, Status> {
        let req = request.into_inner();
        
        // TODO: Stop service gracefully or force
        // Update registry
        if let Some(mut service) = self.service_registry.get(&req.service_name).await {
            service.status = "stopped".to_string();
            self.service_registry.register(service).await;
        }
        
        Ok(Response::new(gladsheim::ServiceStatus {
            service_name: req.service_name,
            state: gladsheim::ServiceState::SERVICE_STATE_STOPPED as i32,
            process_id: 0,
            start_time_unix: 0,
            stop_time_unix: chrono::Utc::now().timestamp(),
            resource_usage: None,
            health_status: None,
            error_message: None,
            restart_count: 0,
        }))
    }

    async fn restart_service(
        &self,
        request: Request<gladsheim::RestartServiceRequest>,
    ) -> Result<Response<gladsheim::ServiceStatus>, Status> {
        let req = request.into_inner();
        
        // Stop first
        let stop_req = gladsheim::StopServiceRequest {
            service_name: req.service_name.clone(),
            force: req.force_stop,
            timeout_ms: req.stop_timeout_ms,
        };
        self.stop_service(Request::new(stop_req)).await?;
        
        // Start again
        let start_req = gladsheim::StartServiceRequest {
            service_name: req.service_name.clone(),
            environment_vars: std::collections::HashMap::new(),
            resource_limits: None,
            working_directory: String::new(),
            args: vec![],
        };
        self.start_service(Request::new(start_req)).await
    }

    async fn get_service_status(
        &self,
        request: Request<gladsheim::ServiceStatusRequest>,
    ) -> Result<Response<gladsheim::ServiceStatus>, Status> {
        let req = request.into_inner();
        
        let service = self.service_registry.get(&req.service_name).await
            .ok_or_else(|| Status::not_found(format!("Service {} not found", req.service_name)))?;
        
        let state = match service.status.as_str() {
            "running" => gladsheim::ServiceState::SERVICE_STATE_RUNNING,
            "stopped" => gladsheim::ServiceState::SERVICE_STATE_STOPPED,
            _ => gladsheim::ServiceState::SERVICE_STATE_UNKNOWN,
        };
        
        Ok(Response::new(gladsheim::ServiceStatus {
            service_name: service.service_name,
            state: state as i32,
            process_id: 0,
            start_time_unix: 0,
            stop_time_unix: 0,
            resource_usage: None,
            health_status: None,
            error_message: None,
            restart_count: 0,
        }))
    }

    async fn list_services(
        &self,
        _request: Request<gladsheim::ListServicesRequest>,
    ) -> Result<Response<gladsheim::ServiceList>, Status> {
        let services = self.service_registry.list().await;
        
        let proto_services: Vec<gladsheim::ServiceStatus> = services.into_iter().map(|s| {
            let state = match s.status.as_str() {
                "running" => gladsheim::ServiceState::SERVICE_STATE_RUNNING,
                "stopped" => gladsheim::ServiceState::SERVICE_STATE_STOPPED,
                _ => gladsheim::ServiceState::SERVICE_STATE_UNKNOWN,
            };
            gladsheim::ServiceStatus {
                service_name: s.service_name,
                state: state as i32,
                process_id: 0,
                start_time_unix: 0,
                stop_time_unix: 0,
                resource_usage: None,
                health_status: None,
                error_message: None,
                restart_count: 0,
            }
        }).collect();
        
        let running_count = proto_services.iter().filter(|s| s.state == gladsheim::ServiceState::SERVICE_STATE_RUNNING as i32).count() as u32;
        let stopped_count = proto_services.iter().filter(|s| s.state == gladsheim::ServiceState::SERVICE_STATE_STOPPED as i32).count() as u32;
        
        Ok(Response::new(gladsheim::ServiceList {
            services: proto_services,
            total_count: proto_services.len() as u32,
            running_count,
            stopped_count,
            crashed_count: 0,
        }))
    }

    async fn get_service_health(
        &self,
        request: Request<gladsheim::ServiceHealthRequest>,
    ) -> Result<Response<gladsheim::ServiceHealth>, Status> {
        let req = request.into_inner();
        
        let health = self.health_monitor.check_health(&req.service_name).await
            .map_err(|e| Status::internal(format!("Health check failed: {}", e)))?;
        
        let status = if health == "healthy" {
            gladsheim::HealthStatus::HEALTH_STATUS_HEALTHY
        } else {
            gladsheim::HealthStatus::HEALTH_STATUS_UNHEALTHY
        };
        
        Ok(Response::new(gladsheim::ServiceHealth {
            service_name: req.service_name,
            status: status as i32,
            message: health,
            last_check_unix: chrono::Utc::now().timestamp(),
            next_check_unix: chrono::Utc::now().timestamp() + 60,
            consecutive_failures: 0,
            check_strategy: "grpc".to_string(),
        }))
    }

    type SubscribeServiceHealthStream = tokio_stream::wrappers::ReceiverStream<Result<gladsheim::HealthUpdate, Status>>;

    async fn subscribe_service_health(
        &self,
        _request: Request<gladsheim::HealthSubscribeRequest>,
    ) -> Result<Response<Self::SubscribeServiceHealthStream>, Status> {
        // TODO: Implement health streaming
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    async fn get_resource_usage(
        &self,
        request: Request<gladsheim::ResourceUsageRequest>,
    ) -> Result<Response<gladsheim::ResourceUsage>, Status> {
        let _req = request.into_inner();
        
        // TODO: Get actual resource usage
        Ok(Response::new(gladsheim::ResourceUsage {
            memory_bytes: 0,
            cpu_percent: 0.0,
            memory_mb: 0.0,
            measured_at_unix: chrono::Utc::now().timestamp(),
        }))
    }

    async fn set_resource_limits(
        &self,
        request: Request<gladsheim::ResourceLimitsRequest>,
    ) -> Result<Response<gladsheim::ResourceLimits>, Status> {
        let req = request.into_inner();
        
        // TODO: Set resource limits
        Ok(Response::new(req.limits.unwrap_or_default()))
    }

    async fn get_resource_limits(
        &self,
        _request: Request<gladsheim::ServiceRequest>,
    ) -> Result<Response<gladsheim::ResourceLimits>, Status> {
        // TODO: Get resource limits
        Ok(Response::new(gladsheim::ResourceLimits {
            max_memory_bytes: 0,
            max_cpu_percent: 0.0,
            max_memory_mb: 0.0,
            memory_warning_percent: None,
            cpu_warning_percent: None,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub service_loader: Arc<crate::thjalfi::ServiceLoader>,
    pub resource_manager: Arc<crate::byggvir::ResourceManager>,
    pub health_monitor: Arc<crate::roskva::HealthMonitor>,
    pub service_registry: Arc<crate::skirnir::ServiceRegistry>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Gladsheim gRPC server on {}", addr);

    let gladsheim_service = GladsheimServiceImpl::new(
        deps.service_loader,
        deps.resource_manager,
        deps.health_monitor,
        deps.service_registry,
    );

    Server::builder()
        .add_service(GladsheimServiceServer::new(gladsheim_service))
        .serve(addr)
        .await?;

    Ok(())
}
