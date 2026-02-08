//! Fenrir binary: Hardware-Control Service entrypoint.

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Fenrir Hardware-Control Service starting (stub)");
    // TODO: gRPC server, hardware init
    Ok(())
}
