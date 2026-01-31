use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use heidrun::utils::config::SettingsManager;
use heidrun::token::counter::TokenCounter;
use heidrun::pricing::calculator::PricingCalculator;
use heidrun::settlement::processor::SettlementProcessor;
use heidrun::preauth::manager::PreAuthManager;
use heidrun::grpc::{start_grpc_server, GrpcServerDependencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Heidrun Token & Pricing Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/heidrun.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}, commission_rate={}", settings.grpc_port, settings.commission_rate);

    // Initialize database connection pool
    let pool = sqlx::PgPool::connect(&settings.database.url).await?;
    info!("Database connection established");

    // Initialize Token Counter
    let token_counter = Arc::new(TokenCounter::new());
    info!("Token Counter initialized");

    // Initialize Pricing Calculator
    let pricing_calculator = Arc::new(PricingCalculator::new(pool.clone(), settings.commission_rate));
    info!("Pricing Calculator initialized");

    // Initialize Settlement Processor
    let settlement_processor = Arc::new(SettlementProcessor::new(pool.clone()));
    info!("Settlement Processor initialized");

    // Initialize Pre-Auth Manager
    let preauth_manager = Arc::new(PreAuthManager::new(pool.clone()));
    info!("Pre-Auth Manager initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        token_counter: token_counter.clone(),
        pricing_calculator: pricing_calculator.clone(),
        settlement_processor: settlement_processor.clone(),
        preauth_manager: preauth_manager.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
