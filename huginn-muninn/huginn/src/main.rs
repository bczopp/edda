use tracing::info;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Huginn STT + Data Forwarding Service starting...");

    // Initialize STT engine
    let stt_engine = Arc::new(huginn::stt::STTEngine::new());
    
    // Initialize data forwarder
    let data_forwarder = Arc::new(huginn::forwarding::DataForwarder::new());
    
    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 50057));
    let deps = huginn::grpc::GrpcServerDependencies {
        stt_engine,
        data_forwarder,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = huginn::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Huginn STT + Data Forwarding Service started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Huginn STT + Data Forwarding Service...");

    Ok(())
}
