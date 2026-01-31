use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Frigg Healthcare Plugin starting...");

    // Initialize plugin
    let plugin = frigg::FriggPlugin::new();
    info!("Frigg plugin initialized: {}", plugin.name());
    info!("Capabilities: {:?}", plugin.capabilities());

    // TODO: Register with Odin via gRPC
    // TODO: Start gRPC server for plugin communication

    info!("Frigg Healthcare Plugin started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Frigg Healthcare Plugin...");

    Ok(())
}
