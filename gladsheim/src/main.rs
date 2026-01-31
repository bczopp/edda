use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use gladsheim::utils::config::SettingsManager;
use gladsheim::thjalfi::loader::ServiceLoader;
use gladsheim::byggvir::manager::ResourceManager;
use gladsheim::roskva::monitor::HealthMonitor;
use gladsheim::skirnir::registry::ServiceRegistry;
use gladsheim::grpc::{start_grpc_server, GrpcServerDependencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Gladsheim Service Manager starting...");

    // Load configuration
    let config_path = PathBuf::from("config/gladsheim.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}", settings.grpc_port);

    // Initialize Thjalfi (Service Loader)
    let service_loader = Arc::new(ServiceLoader::new(settings.service_directory.clone()));
    info!("Thjalfi (Service Loader) initialized");

    // Initialize Byggvir (Resource Manager)
    let resource_manager = Arc::new(ResourceManager::new(
        settings.max_memory_mb,
        settings.max_cpu_percent,
    ));
    info!("Byggvir (Resource Manager) initialized");

    // Initialize Roskva (Health Monitor)
    let health_monitor = Arc::new(HealthMonitor::new());
    info!("Roskva (Health Monitor) initialized");

    // Initialize Skirnir (Service Registry)
    let service_registry = Arc::new(ServiceRegistry::new());
    info!("Skirnir (Service Registry) initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        service_loader: service_loader.clone(),
        resource_manager: resource_manager.clone(),
        health_monitor: health_monitor.clone(),
        service_registry: service_registry.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
