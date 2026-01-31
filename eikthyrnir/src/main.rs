use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use eikthyrnir::utils::config::SettingsManager;
use eikthyrnir::assessment::engine::QualityAssessor;
use eikthyrnir::aggregation::processor::QualityAggregator;
use eikthyrnir::metrics::tracker::MetricsTracker;
use eikthyrnir::grpc::{start_grpc_server, GrpcServerDependencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Eikthyrnir Quality Assessment Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/eikthyrnir.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}", settings.grpc_port);

    // Initialize database connection pool
    let pool = sqlx::PgPool::connect(&settings.database.url).await?;
    info!("Database connection established");

    // Initialize Quality Assessor
    let quality_assessor = Arc::new(QualityAssessor::new(pool.clone()));
    info!("Quality Assessor initialized");

    // Initialize Quality Aggregator
    let quality_aggregator = Arc::new(QualityAggregator::new(pool.clone()));
    info!("Quality Aggregator initialized");

    // Initialize Metrics Tracker
    let metrics_tracker = Arc::new(MetricsTracker::new(pool.clone()));
    info!("Metrics Tracker initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        quality_assessor: quality_assessor.clone(),
        quality_aggregator: quality_aggregator.clone(),
        metrics_tracker: metrics_tracker.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
