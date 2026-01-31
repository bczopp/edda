use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use njordr::utils::config::SettingsManager;
use njordr::earnings::manager::EarningsManager;
use njordr::settlements::processor::SettlementProcessor;
use njordr::trade::manager::TradeManager;
use njordr::payment::gateway::PaymentGateway;
use njordr::grpc::{start_grpc_server, GrpcServerDependencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Njörðr Marketplace Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/njordr.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}, commission_rate={}", settings.grpc_port, settings.commission_rate);

    // Initialize database connection pool
    let pool = sqlx::PgPool::connect(&settings.database.url).await?;
    info!("Database connection established");

    // Initialize Earnings Manager
    let earnings_manager = Arc::new(EarningsManager::new(pool.clone(), settings.commission_rate));
    info!("Earnings Manager initialized");

    // Initialize Settlement Processor
    let settlement_processor = Arc::new(SettlementProcessor::new(
        pool.clone(),
        EarningsManager::new(pool.clone(), settings.commission_rate),
    ));
    info!("Settlement Processor initialized");

    // Initialize Trade Manager
    let trade_manager = Arc::new(TradeManager::new(pool.clone()));
    info!("Trade Manager initialized");

    // Initialize Payment Gateway
    let payment_gateway = Arc::new(PaymentGateway::new(
        settings.payment.stripe_api_key.clone(),
        settings.payment.paypal_client_id.clone(),
    ));
    info!("Payment Gateway initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        earnings_manager: earnings_manager.clone(),
        settlement_processor: settlement_processor.clone(),
        trade_manager: trade_manager.clone(),
        payment_gateway: payment_gateway.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
