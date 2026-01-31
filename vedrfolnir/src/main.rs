use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use vedrfolnir::utils::config::VedrfolnirSettings;
use vedrfolnir::connection::{ConnectionBuilder, ConnectionManager};
use vedrfolnir::auth::AuthManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Vedrfolnir Connection Builder Client starting...");

    // Load configuration
    let config_path = PathBuf::from("config/vedrfolnir.json");
    let settings = if config_path.exists() {
        let config = config::Config::builder()
            .add_source(config::File::new(
                config_path.to_str().unwrap(),
                config::FileFormat::Json,
            ))
            .build()?;
        config.try_deserialize::<VedrfolnirSettings>()?
    } else {
        info!("Config file not found, using defaults");
        VedrfolnirSettings::default()
    };

    info!("Configuration loaded: Ratatoskr URL={}, gRPC URL={}", 
          settings.yggdrasil_ratatoskr_url, settings.yggdrasil_grpc_url);

    // Initialize auth manager
    let auth_manager = Arc::new(AuthManager::new());
    
    // Initialize connection builder
    let connection_builder = Arc::new(ConnectionBuilder::new(
        settings.yggdrasil_ratatoskr_url.clone(),
    ));
    
    // Initialize connection manager
    let connection_manager = Arc::new(ConnectionManager::new(30)); // 30 second heartbeat
    
    // For demonstration, we'll use placeholder credentials
    // In a real implementation, these would come from device configuration or user input
    let device_id = "device-123".to_string();
    let user_id = "user-123".to_string();
    let device_identity = "device-identity-123".to_string();
    
    // Authenticate to get token
    let auth_token = auth_manager.authenticate(&device_identity).await
        .map_err(|e| format!("Authentication failed: {}", e))?;
    info!("Authentication successful");
    
    // Build connection
    info!("Building connection to Yggdrasil...");
    let connection = connection_builder.build_connection(
        device_id.clone(),
        user_id.clone(),
        device_identity.clone(),
        auth_token,
    ).await
    .map_err(|e| format!("Failed to build connection: {}", e))?;
    
    info!("Connection established with session_id: {}", connection.session_id());
    
    // Set connection in manager
    connection_manager.set_connection(connection).await;
    
    // Start heartbeat
    connection_manager.start_heartbeat().await;
    info!("Heartbeat started");
    
    info!("Vedrfolnir Connection Builder Client started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Vedrfolnir Connection Builder Client...");
    
    // Disconnect
    connection_manager.disconnect().await
        .map_err(|e| format!("Failed to disconnect: {}", e))?;
    
    info!("Disconnected successfully");

    Ok(())
}
