use tracing::info;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Loki Script Execution Service starting...");

    // Initialize service coordinator
    let coordinator = Arc::new(loki::coordination::ServiceCoordinator::new()?);
    
    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 50056));
    let deps = loki::grpc::GrpcServerDependencies {
        coordinator,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = loki::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Loki Script Execution Service started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Loki Script Execution Service...");

    Ok(())
}
