use tracing::info;
use skuld::utils::config::SettingsManager;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Skuld LLM Selection Service starting...");

    let config_path = PathBuf::from("config/skuld.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded");

    // Initialize database connection
    let pool = sqlx::PgPool::connect(&settings.database_url).await?;
    
    // Run migrations if migrations directory exists
    if std::path::Path::new("./migrations").exists() {
        sqlx::migrate!("./migrations").run(&pool).await?;
    }
    
    // Initialize model registry
    let registry = Arc::new(skuld::registry::ModelRegistry::new(pool));
    
    // Initialize evaluator
    let evaluator = Arc::new(skuld::evaluation::ModelEvaluator);
    
    // Initialize model selector
    let selector = Arc::new(skuld::selection::ModelSelector::new(
        registry.clone(),
        evaluator.clone(),
    ));
    
    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.grpc_port));
    let deps = skuld::grpc::GrpcServerDependencies {
        selector,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = skuld::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Skuld LLM Selection Service started successfully on port {}", settings.grpc_port);

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Skuld LLM Selection Service...");

    Ok(())
}
