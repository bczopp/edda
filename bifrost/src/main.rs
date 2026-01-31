use tracing::{error, info};
use bifrost::utils::config::SettingsManager;
use bifrost::utils::init_logging;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging();

    info!("Bifrost Communication Service starting...");

    let config_path = PathBuf::from("config/bifrost.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded");

    // Start WebSocket server
    let ws_server = bifrost::websocket::WebSocketServer::new(settings.websocket_port);
    let server_handle = tokio::spawn(async move {
        if let Err(e) = ws_server.start().await {
            error!("WebSocket server error: {}", e);
        }
    });

    info!("Bifrost Communication Service started successfully on port {}", settings.websocket_port);

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Bifrost Communication Service...");
    
    server_handle.abort();

    Ok(())
}
