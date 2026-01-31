use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use nornen::utils::config::SettingsManager;
use nornen::urd::registry::ProviderRegistry;
use nornen::verdandi::router::RequestRouter;
use nornen::coordinator::NornenCoordinator;
use nornen::grpc::{start_grpc_server, GrpcServerDependencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Nornen Decision Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/nornen.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}", settings.grpc_port);

    // Initialize database connection pool
    let pool = sqlx::PgPool::connect(&settings.database.url).await?;
    info!("Database connection established");

    // Initialize Provider Registry (Urd)
    let registry = Arc::new(ProviderRegistry::new(pool.clone()).await?);
    info!("Provider Registry (Urd) initialized");

    // Initialize Request Router (Verdandi)
    let router = Arc::new(RequestRouter::new(registry.clone()));
    info!("Request Router (Verdandi) initialized");

    // Initialize Nornen Coordinator
    let coordinator = Arc::new(NornenCoordinator::new(registry.clone(), router.clone()));
    info!("Nornen Coordinator initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        coordinator: coordinator.clone(),
        registry: registry.clone(),
        router: router.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
