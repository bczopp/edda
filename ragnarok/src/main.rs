use tracing::info;
use std::path::PathBuf;
use ragnarok::utils::config::SettingsManager;
use ragnarok::services::OdinServiceIntegration;
use ragnarok::cli::parse_args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Ragnarok Terminal Platform starting...");

    // Load configuration
    let config_path = PathBuf::from("config/ragnarok.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: Odin={}:{}", settings.odin.address, settings.odin.port);

    // Parse CLI arguments
    let cli = parse_args();

    // Odin service integration (Phase 4)
    let mut odin_integration = OdinServiceIntegration::new(&settings.odin.address, settings.odin.port).await?;
    info!("Odin service integration initialized");

    // Execute command
    if let Some(command) = cli.command {
        match command {
            ragnarok::cli::Commands::Chat { message } => {
                let response = odin_integration.send_chat(&message).await?;
                println!("{}", response);
            }
            ragnarok::cli::Commands::Action { action } => {
                println!("Executing action: {}", action);
            }
            ragnarok::cli::Commands::Status => {
                println!("Ragnarok Terminal Platform - Status: Running");
            }
            ragnarok::cli::Commands::Settings => {
                println!("Config path: {:?}", config_path);
                println!("Odin: {}:{}", settings.odin.address, settings.odin.port);
            }
        }
    } else {
        println!("Ragnarok Terminal Platform - Use --help for usage");
    }

    Ok(())
}
