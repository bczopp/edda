use tracing::info;
use geri::utils::config::SettingsManager;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Geri LLM Service starting...");

    let config_path = PathBuf::from("config/geri.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded");

    // Initialize LLM provider
    let llm_provider: Arc<dyn geri::llm::LLMProvider> = Arc::new(geri::llm::LocalLLMProvider::new(
        settings.default_local_llm.clone(),
    ));

    // Initialize vision processor
    let vision_processor = Arc::new(geri::vision::VisionProcessor::new(
        settings.vision_model.clone(),
    ));

    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.grpc_port));
    let deps = geri::grpc::GrpcServerDependencies {
        llm_provider,
        vision_processor,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = geri::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Geri LLM Service started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Geri LLM Service...");

    Ok(())
}
