//! Hel binary: Data/Storage Service entrypoint.

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Hel Data/Storage Service starting (stub)");
    // TODO: gRPC server, filesystem/storage/cache init
    Ok(())
}
