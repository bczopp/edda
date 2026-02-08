//! Jörmungandr binary: Network/Communication Service entrypoint.

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Jörmungandr Network/Communication Service starting (stub)");
    // TODO: gRPC server, HTTP/WebSocket/MQTT handlers
    Ok(())
}
