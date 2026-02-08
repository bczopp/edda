//! Gladsheim binary - Service Manager & Runtime Manager

use tracing::info;
use std::path::PathBuf;
use std::net::SocketAddr;
use gladsheim::grpc::run_server;
use gladsheim::utils::config::GladsheimConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Gladsheim Service Manager starting...");

    let config_path = PathBuf::from("config/gladsheim.json");
    let config = if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        GladsheimConfig::from_json(&content)?
    } else {
        info!("Config not found, using defaults");
        GladsheimConfig::default()
    };

    let addr: SocketAddr = format!("{}:{}", config.grpc_host, config.grpc_port).parse()?;
    info!("Starting gRPC server on {}", addr);
    run_server(addr).await?;

    Ok(())
}
