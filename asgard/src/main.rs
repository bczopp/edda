use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Asgard Homeserver Platform starting...");

    // Load configuration
    let config_path = PathBuf::from("config/asgard.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: Odin={}:{}, Server={}:{}", 
        settings.odin.address, settings.odin.port,
        settings.server.host, settings.server.port);

    // Initialize Odin gRPC client
    let odin_client = Arc::new(tokio::sync::Mutex::new(
        OdinClient::new(&settings.odin.address, settings.odin.port).await?
    ));
    info!("Odin gRPC client initialized");

    // Initialize Server Manager
    let server_manager = Arc::new(ServerManager::new(odin_client.clone()));
    server_manager.initialize().await?;
    info!("Server Manager initialized");

    info!("Asgard Homeserver Platform started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Asgard Homeserver Platform...");
    
    server_manager.shutdown().await?;

    Ok(())
}
