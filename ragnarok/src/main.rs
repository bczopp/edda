use tracing::info;
use std::path::PathBuf;
use ragnarok::utils::config::SettingsManager;
use ragnarok::grpc_client::OdinClient;
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

    // Initialize Odin gRPC client
    let mut odin_client = OdinClient::new(&settings.odin.address, settings.odin.port).await?;
    info!("Odin gRPC client initialized");

    // Execute command
    if let Some(command) = cli.command {
        match command {
            ragnarok::cli::Commands::Chat { message } => {
                let request = odin::ProcessRequest {
                    text: message,
                    ..Default::default()
                };
                let response = odin_client.process_request(request).await?;
                println!("{}", response.text);
            }
            ragnarok::cli::Commands::Action { action } => {
                println!("Executing action: {}", action);
            }
            ragnarok::cli::Commands::Status => {
                println!("Ragnarok Terminal Platform - Status: Running");
            }
        }
    } else {
        println!("Ragnarok Terminal Platform - Use --help for usage");
    }

    Ok(())
}

mod odin {
    tonic::include_proto!("odin");
}
