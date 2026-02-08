//! Gladsheim gRPC Service Implementation

use tonic::{Request, Response, Status};
use tracing::info;
use crate::thjalfi::Thjalfi;
use crate::byggvir::{Byggvir, ResourceManager, ResourceLimits as InternalResourceLimits};
use crate::roskva::Roskva;
use crate::skirnir::Skirnir;
use std::net::SocketAddr;
use std::time::Duration;

// Include generated proto code
pub mod gladsheim {
    tonic::include_proto!("gladsheim.v1");
}

use gladsheim::gladsheim_service_server::GladsheimService;
use gladsheim::*;

pub struct GladsheimServiceImpl {
    thjalfi: Thjalfi,
    #[allow(dead_code)]
    byggvir: Byggvir,
    resource_manager: ResourceManager,
    roskva: Arc<Roskva>,
    skirnir: Skirnir,
    _resource_handle: crate::byggvir::MonitoringHandle,
    _health_handle: crate::roskva::MonitoringLoopHandle,
}

impl GladsheimServiceImpl {
    pub async fn new() -> Self {
        info!("Creating GladsheimServiceImpl");
        
        let skirnir = Skirnir::new().expect("Failed to create Skirnir");
        
        let mut resource_manager = ResourceManager::new();
        resource_manager.set_skirnir(skirnir.clone());
        let _resource_handle = resource_manager.start_monitoring_loop(Duration::from_secs(5));

        let mut roskva_inner = Roskva::new().expect("Failed to create Roskva");
        roskva_inner.set_skirnir(skirnir.clone());
        let roskva = Arc::new(roskva_inner);
        let _health_handle = roskva.clone().start_monitoring_loop(Duration::from_secs(10), None).await;

        Self {
            thjalfi: Thjalfi::new().expect("Failed to create Thjalfi"),
            byggvir: Byggvir::new().expect("Failed to create Byggvir"),
            resource_manager,
            roskva,
            skirnir,
            _resource_handle,
            _health_handle,
        }
    }
    
    pub fn is_ready(&self) -> bool {
        true
    }
}

#[tonic::async_trait]
impl GladsheimService for GladsheimServiceImpl {
    async fn start_service(
        &self,
        request: Request<StartServiceRequest>,
    ) -> Result<Response<ServiceStatus>, Status> {
        let req = request.into_inner();
        
        info!("StartService request for: {}", req.service_name);
        
        // 1. Skirnir prüft ob Service bereits läuft
        if self.skirnir.get_service(&req.service_name).await.is_some() {
            return Err(Status::already_exists(format!("Service '{}' is already running", req.service_name)));
        }
        
        // 2. TODO: Heimdall-Authorization (Phase 6)
        
        // 3. Thjalfi lädt Service
        let config = crate::thjalfi::ServiceConfig {
            name: req.service_name.clone(),
            command: "".to_string(), // TODO: Resolve service path from config
            args: req.args,
            working_directory: if req.working_directory.is_empty() {
                None
            } else {
                Some(std::path::PathBuf::from(req.working_directory))
            },
            environment_vars: req.environment_vars,
        };
        
        self.thjalfi.start_service(config, Duration::from_secs(5))
            .await
            .map_err(|e| Status::internal(format!("Failed to start service: {}", e)))?;
        
        let process_id = self.thjalfi.get_service_pid(&req.service_name).await.unwrap_or(0);
        
        // 4. Byggvir alloziert Ressourcen (Phase 5)
        
        // 5. Roskva registriert Health-Monitoring
        self.roskva.health_tracker().register_service(req.service_name.clone(), crate::roskva::HealthCheckStrategy::Process).await;
        
        // 6. Skirnir registriert Service
        self.skirnir.register_service(req.service_name.clone()).await
            .map_err(|e| Status::internal(format!("Failed to register service: {}", e)))?;
        
        // Update PID and status in Skirnir
        self.skirnir.update_pid(&req.service_name, process_id).await
            .map_err(|e| Status::internal(format!("Failed to update PID: {}", e)))?;
        self.skirnir.update_status(&req.service_name, crate::skirnir::ServiceStatus::Running).await
            .map_err(|e| Status::internal(format!("Failed to update status: {}", e)))?;

        // 7. Register with ResourceManager for usage tracking and limits
        let limits = req.resource_limits.as_ref().map(|l| InternalResourceLimits::new(
            (l.max_memory_bytes / (1024 * 1024)) as u64,
            l.max_cpu_percent,
        )).unwrap_or_else(|| InternalResourceLimits::default_server());
        self.resource_manager.register_service(req.service_name.clone(), process_id, limits).await;
        
        Ok(Response::new(ServiceStatus {
            service_name: req.service_name,
            state: ServiceState::Running as i32,
            process_id: process_id as i32,
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
        request: Request<StopServiceRequest>,
    ) -> Result<Response<ServiceStatus>, Status> {
        let req = request.into_inner();
        
        info!("StopService request for: {} (force: {})", req.service_name, req.force);
        
        let service_info = self.skirnir.get_service(&req.service_name).await
            .ok_or_else(|| Status::not_found(format!("Service '{}' not found", req.service_name)))?;

        let process_id = service_info.process_id.unwrap_or(0) as i32;
        let start_time_unix = service_info.start_time.map(|t| t.timestamp()).unwrap_or(0);

        self.skirnir.update_status(&req.service_name, crate::skirnir::ServiceStatus::Stopping).await
            .map_err(|e| Status::internal(format!("Failed to update status: {}", e)))?;

        // 1. Unregister from monitoring
        self.resource_manager.unregister_service(&req.service_name).await;
        self.roskva.health_tracker().unregister_service(&req.service_name).await; // Unregister from Roskva
        
        // 2. Thjalfi stoppt Service
        let timeout = req.timeout_ms.and_then(|ms| if ms > 0 { Some(Duration::from_millis(ms as u64)) } else { None });
        if let Err(e) = self.thjalfi.stop_service(&req.service_name, req.force, timeout).await {
            self.skirnir.update_status(&req.service_name, crate::skirnir::ServiceStatus::Running).await.ok();
            return Err(Status::internal(format!("Failed to stop service: {}", e)));
        }

        // 3. Skirnir aktualisiert Status und deregistriert Service
        self.skirnir.update_status(&req.service_name, crate::skirnir::ServiceStatus::Stopped).await
            .map_err(|e| Status::internal(format!("Failed to update status: {}", e)))?;

        self.skirnir.unregister_service(&req.service_name).await
            .map_err(|e| Status::internal(format!("Failed to unregister: {}", e)))?;

        info!("Service '{}' successfully stopped via gRPC", req.service_name);

        Ok(Response::new(ServiceStatus {
            service_name: req.service_name,
            state: ServiceState::Stopped as i32,
            process_id,
            start_time_unix,
            stop_time_unix: chrono::Utc::now().timestamp(),
            resource_usage: None,
            health_status: None,
            error_message: None,
            restart_count: 0,
        }))
    }
    
    async fn restart_service(
        &self,
        request: Request<RestartServiceRequest>,
    ) -> Result<Response<ServiceStatus>, Status> {
        let req = request.into_inner();
        
        // 1. Stop service first
        let stop_request = StopServiceRequest {
            service_name: req.service_name.clone(),
            force: req.force_stop,
            timeout_ms: req.stop_timeout_ms,
        };
        
        let _ = self.stop_service(Request::new(stop_request)).await;
        
        // Wait a bit for cleanup
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // 2. Start service again
        // TODO: Get original service config from somewhere (service registry or config)
        // For now, use minimal config
        let start_request = StartServiceRequest {
            service_name: req.service_name.clone(),
            environment_vars: std::collections::HashMap::new(),
            resource_limits: None,
            working_directory: String::new(),
            args: vec![],
        };
        
        self.start_service(Request::new(start_request)).await
    }
    
    async fn get_service_status(
        &self,
        request: Request<ServiceStatusRequest>,
    ) -> Result<Response<ServiceStatus>, Status> {
        let req = request.into_inner();
        
        info!("GetServiceStatus request for: {}", req.service_name);
        
        // Check if service exists in registry
        let service_info = self.skirnir.get_service(&req.service_name).await;
        
        if let Some(service) = service_info {
            let state = match service.status {
                crate::skirnir::ServiceStatus::Starting => ServiceState::Starting,
                crate::skirnir::ServiceStatus::Running => ServiceState::Running,
                crate::skirnir::ServiceStatus::Stopping => ServiceState::Stopping,
                crate::skirnir::ServiceStatus::Stopped => ServiceState::Stopped,
                crate::skirnir::ServiceStatus::Crashed => ServiceState::Crashed,
            };
            
            Ok(Response::new(ServiceStatus {
                service_name: service.name,
                state: state as i32,
                process_id: service.process_id.unwrap_or(0) as i32,
                start_time_unix: service.start_time.map(|t| t.timestamp()).unwrap_or(0),
                stop_time_unix: 0,
                resource_usage: Some(ResourceUsage {
                    memory_bytes: service.resource_usage.memory_bytes,
                    cpu_percent: service.resource_usage.cpu_percent,
                    memory_mb: (service.resource_usage.memory_bytes as f32 / (1024.0 * 1024.0)),
                    measured_at_unix: chrono::Utc::now().timestamp(),
                }),
                health_status: Some(ServiceHealth {
                    service_name: service.name.clone(),
                    status: (if service.health.is_healthy { HealthStatus::Healthy } else { HealthStatus::Unhealthy }) as i32,
                    message: service.health.error_message.unwrap_or_default(),
                    last_check_unix: chrono::Utc::now().timestamp(),
                    next_check_unix: 0,
                    consecutive_failures: 0,
                    check_strategy: String::new(),
                }),
                error_message: None,
                restart_count: 0,
            }))
        } else {
            Err(Status::not_found(format!("Service '{}' not found", req.service_name)))
        }
    }
    
    async fn list_services(
        &self,
        request: Request<ListServicesRequest>,
    ) -> Result<Response<ServiceList>, Status> {
        let req = request.into_inner();
        
        info!("ListServices request");
        
        let all_services = self.skirnir.list_services().await;
        
        // Filter by states if requested
        let filtered_services: Vec<_> = if req.filter_states.is_empty() {
            all_services.clone()
        } else {
            all_services.iter()
                .filter(|s| {
                    let state = match s.status {
                        crate::skirnir::ServiceStatus::Starting => ServiceState::Starting,
                        crate::skirnir::ServiceStatus::Running => ServiceState::Running,
                        crate::skirnir::ServiceStatus::Stopping => ServiceState::Stopping,
                        crate::skirnir::ServiceStatus::Stopped => ServiceState::Stopped,
                        crate::skirnir::ServiceStatus::Crashed => ServiceState::Crashed,
                    };
                    req.filter_states.contains(&(state as i32))
                })
                .cloned()
                .collect()
        };
        
        let proto_services: Vec<ServiceStatus> = filtered_services.iter().map(|s| {
            let state = match s.status {
                crate::skirnir::ServiceStatus::Starting => ServiceState::Starting,
                crate::skirnir::ServiceStatus::Running => ServiceState::Running,
                crate::skirnir::ServiceStatus::Stopping => ServiceState::Stopping,
                crate::skirnir::ServiceStatus::Stopped => ServiceState::Stopped,
                crate::skirnir::ServiceStatus::Crashed => ServiceState::Crashed,
            };
            
            ServiceStatus {
                service_name: s.name.clone(),
                state: state as i32,
                process_id: s.process_id.unwrap_or(0) as i32,
                start_time_unix: s.start_time.map(|t| t.timestamp()).unwrap_or(0),
                stop_time_unix: 0,
                resource_usage: Some(ResourceUsage {
                    memory_bytes: s.resource_usage.memory_bytes,
                    cpu_percent: s.resource_usage.cpu_percent,
                    memory_mb: (s.resource_usage.memory_bytes as f32 / (1024.0 * 1024.0)),
                    measured_at_unix: chrono::Utc::now().timestamp(),
                }),
                health_status: Some(ServiceHealth {
                    service_name: s.name.clone(),
                    status: (if s.health.is_healthy { HealthStatus::Healthy } else { HealthStatus::Unhealthy }) as i32,
                    message: s.health.error_message.unwrap_or_default(),
                    last_check_unix: chrono::Utc::now().timestamp(),
                    next_check_unix: 0,
                    consecutive_failures: 0,
                    check_strategy: String::new(),
                }),
                error_message: None,
                restart_count: 0,
            }
        }).collect();
        
        let running_count = proto_services.iter()
            .filter(|s| s.state == ServiceState::Running as i32)
            .count() as u32;
        
        let stopped_count = proto_services.iter()
            .filter(|s| s.state == ServiceState::Stopped as i32)
            .count() as u32;
        
        let crashed_count = proto_services.iter()
            .filter(|s| s.state == ServiceState::Crashed as i32)
            .count() as u32;
        
        let total_count = proto_services.len() as u32;
        Ok(Response::new(ServiceList {
            services: proto_services,
            total_count,
            running_count,
            stopped_count,
            crashed_count,
        }))
    }
    
    async fn get_service_health(
        &self,
        request: Request<ServiceHealthRequest>,
    ) -> Result<Response<ServiceHealth>, Status> {
        let req = request.into_inner();
        
        // Get health from Roskva
        let health_data = self.roskva.health_tracker().get_health(&req.service_name).await
            .ok_or_else(|| Status::not_found(format!("Health data for '{}' not found", req.service_name)))?;
        
        let status = if health_data.is_healthy {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        };
        
        Ok(Response::new(ServiceHealth {
            service_name: req.service_name,
            status: status as i32,
            message: health_data.error_message.unwrap_or_default(),
            last_check_unix: health_data.last_check.timestamp(),
            next_check_unix: health_data.last_check.timestamp() + self.roskva.health_monitor().check_interval().as_secs() as i64,
            consecutive_failures: health_data.consecutive_failures,
            check_strategy: format!("{:?}", health_data.check_strategy),
        }))
    }
    
    type SubscribeServiceHealthStream = std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<HealthUpdate, Status>> + Send + 'static>
    >;
    
    async fn subscribe_service_health(
        &self,
        request: Request<HealthSubscribeRequest>,
    ) -> Result<Response<Self::SubscribeServiceHealthStream>, Status> {
        let req = request.into_inner();
        
        info!("SubscribeServiceHealth request for: {} (interval: {:?} ms)", 
            req.service_name, req.update_interval_ms);
        
        // Check if service exists
        let _service_info = self.skirnir.get_service(&req.service_name).await
            .ok_or_else(|| Status::not_found(format!("Service '{}' not found", req.service_name)))?;
        
        // Get update interval (default: config interval)
        let update_interval = req.update_interval_ms
            .map(|ms| Duration::from_millis(ms as u64))
            .unwrap_or_else(|| self.roskva.health_monitor().check_interval());
        
        // Create streaming channel
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        
        // Spawn background task for health updates (clone so we can move into 'static task)
        let service_name = req.service_name.clone();
        let roskva_tracker = self.roskva.health_tracker().clone();
        let roskva_monitor = self.roskva.health_monitor().clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(update_interval);
            
            loop {
                interval.tick().await;
                
                // Check health
                let strategy = roskva_tracker.get_strategy(&service_name).await;
                let is_healthy = if let Some(strategy) = strategy {
                    match strategy {
                        crate::roskva::monitoring::HealthCheckStrategy::Http { url } => {
                            roskva_monitor.check_http_health(&url).await.unwrap_or(false)
                        }
                        crate::roskva::monitoring::HealthCheckStrategy::Grpc { service } => {
                            roskva_monitor.check_grpc_health(&service).await.unwrap_or(false)
                        }
                        crate::roskva::monitoring::HealthCheckStrategy::Process => {
                            true // TODO: Check if process is running
                        }
                    }
                } else {
                    true // No strategy, assume healthy
                };
                
                // Update tracker
                roskva_tracker.update_health(&service_name, is_healthy, None).await;
                
                // Get current health
                let health_data = roskva_tracker.get_health(&service_name).await;
                
                if let Some(health) = health_data {
                    let status = if health.is_healthy {
                        HealthStatus::Healthy
                    } else {
                        HealthStatus::Unhealthy
                    };
                    
                    let health_update = HealthUpdate {
                        health: Some(ServiceHealth {
                            service_name: service_name.clone(),
                            status: status as i32,
                            message: health.error_message.unwrap_or_default(),
                            last_check_unix: health.last_check.timestamp(),
                            next_check_unix: health.last_check.timestamp() + update_interval.as_secs() as i64,
                            consecutive_failures: health.consecutive_failures,
                            check_strategy: format!("{:?}", health.check_strategy),
                        }),
                        update_time_unix: chrono::Utc::now().timestamp(),
                        state_changed: false, // TODO: Track state changes
                    };
                    
                    // Send update (ignore errors if receiver dropped)
                    if tx.send(Ok(health_update)).await.is_err() {
                        break; // Receiver dropped, stop streaming
                    }
                }
            }
        });
        
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx)
        )))
    }
    
    async fn get_resource_usage(
        &self,
        request: Request<ResourceUsageRequest>,
    ) -> Result<Response<ResourceUsage>, Status> {
        let req = request.into_inner();
        
        info!("GetResourceUsage request for: {}", req.service_name);
        
        let _service_info = self.skirnir.get_service(&req.service_name).await
            .ok_or_else(|| Status::not_found(format!("Service '{}' not found", req.service_name)))?;

        let usage = self.resource_manager.get_resource_usage(&req.service_name).await
            .ok_or_else(|| Status::failed_precondition(format!("No resource usage for '{}'", req.service_name)))?;

        Ok(Response::new(ResourceUsage {
            memory_bytes: usage.memory_bytes,
            cpu_percent: usage.cpu_percent,
            memory_mb: (usage.memory_bytes as f64 / (1024.0 * 1024.0)) as f32,
            measured_at_unix: usage.measured_at.timestamp(),
        }))
    }
    
    async fn set_resource_limits(
        &self,
        request: Request<ResourceLimitsRequest>,
    ) -> Result<Response<ResourceLimits>, Status> {
        let req = request.into_inner();
        
        info!("SetResourceLimits request for: {}", req.service_name);
        
        // Check if service exists
        let _service_info = self.skirnir.get_service(&req.service_name).await
            .ok_or_else(|| Status::not_found(format!("Service '{}' not found", req.service_name)))?;
        
        let limits = req
            .limits
            .ok_or_else(|| Status::invalid_argument("Resource limits not provided"))?;

        // Convert proto limits to internal representation (MB + percent)
        let internal_limits = InternalResourceLimits::new(
            (limits.max_memory_bytes / (1024 * 1024)) as u64,
            limits.max_cpu_percent,
        );

        // Store in ResourceManager
        self.resource_manager
            .set_limits(req.service_name.clone(), internal_limits)
            .await;

        Ok(Response::new(limits))
    }
    
    async fn get_resource_limits(
        &self,
        request: Request<ServiceRequest>,
    ) -> Result<Response<ResourceLimits>, Status> {
        let req = request.into_inner();
        
        info!("GetResourceLimits request for: {}", req.service_name);
        
        // Check if service exists
        let _service_info = self.skirnir.get_service(&req.service_name).await
            .ok_or_else(|| Status::not_found(format!("Service '{}' not found", req.service_name)))?;
        
        // Try to get stored resource limits from ResourceManager; fall back to defaults
        let internal_limits = self
            .resource_manager
            .get_limits(&req.service_name)
            .await
            .unwrap_or_else(|| InternalResourceLimits::default_server());
        
        Ok(Response::new(ResourceLimits {
            max_memory_bytes: internal_limits.max_memory_mb * 1024 * 1024,
            max_cpu_percent: internal_limits.max_cpu_percent,
            max_memory_mb: internal_limits.max_memory_mb as f32,
            memory_warning_percent: Some(80.0),
            cpu_warning_percent: Some(80.0),
        }))
    }
}

/// Run the gRPC server on the given address.
pub async fn run_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let service = GladsheimServiceImpl::new().await;
    tonic::transport::Server::builder()
        .add_service(gladsheim::gladsheim_service_server::GladsheimServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
