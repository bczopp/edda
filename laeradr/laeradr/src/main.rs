use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use laeradr::utils::config::SettingsManager;
use laeradr::coordinator::ServiceCoordinator;
use laeradr::grpc::{start_grpc_server, GrpcServerDependencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Laeradr Data Management Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/laeradr.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}", settings.grpc_port);

    // Initialize Dainn (Indexing Service)
    let dainn = Arc::new(dainn::IndexingService::new(settings.dainn.index_path.clone()));
    info!("Dainn (Indexing) initialized");

    // Initialize Dvalinn (Validation Service)
    let dvalinn = Arc::new(dvalinn::ValidationService::new(settings.dvalinn.schema_directory.clone()));
    info!("Dvalinn (Validation) initialized");

    // Initialize Duneyrr (Aggregation Service)
    let duneyrr = Arc::new(duneyrr::AggregationService::new(settings.duneyrr.batch_size));
    info!("Duneyrr (Aggregation) initialized");

    // Initialize Durathror (Retention Service)
    let durathror = Arc::new(durathror::ArchivingService::new(
        settings.durathror.s3_endpoint.clone(),
        settings.durathror.s3_bucket.clone(),
    ));
    info!("Durathror (Retention) initialized");

    // Initialize Coordinator
    let coordinator = Arc::new(ServiceCoordinator::new(
        dainn.clone(),
        dvalinn.clone(),
        duneyrr.clone(),
        durathror.clone(),
    ));
    info!("Service Coordinator initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        coordinator: coordinator.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
