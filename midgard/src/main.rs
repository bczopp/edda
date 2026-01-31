use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use midgard::utils::config::SettingsManager;
use midgard::grpc_client::OdinClient;
use midgard::audio::manager::AudioManager;
use midgard::platform::manager::PlatformManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Midgard Desktop Platform starting...");

    // Load configuration
    let config_path = PathBuf::from("config/midgard.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: Odin={}:{}", settings.odin.address, settings.odin.port);

    // Initialize Odin gRPC client
    let odin_client = Arc::new(tokio::sync::Mutex::new(
        OdinClient::new(&settings.odin.address, settings.odin.port).await?
    ));
    info!("Odin gRPC client initialized");

    // Initialize Audio Manager
    let audio_manager = Arc::new(AudioManager::new(settings.audio.sample_rate));
    audio_manager.initialize().await?;
    info!("Audio Manager initialized");

    // Initialize Platform Manager
    let platform_manager = Arc::new(PlatformManager::new(
        odin_client.clone(),
        audio_manager.clone(),
    ));
    platform_manager.initialize().await?;
    info!("Platform Manager initialized");

    info!("Midgard Desktop Platform started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Midgard Desktop Platform...");
    
    platform_manager.shutdown().await?;

    Ok(())
}
