use std::sync::Arc;
use tracing::info;
use geri::utils::config::SettingsManager;
use geri::model::ModelRegistry;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    geri::logging::init_logging(Some("info"))?;

    info!("Geri LLM Service starting...");

    let config_path = PathBuf::from("config/geri.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager
        .load()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })?;
    settings_manager
        .start_hot_reload()
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded");

    // Initialize LLM provider using LocalLLMManager
    use geri::llm::local_manager::LocalLLMManager;
    
    let local_manager = LocalLLMManager::new()
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })?;
    
    let llm_provider: Arc<dyn geri::llm::LLMProvider> = if settings.local_provider.auto_select {
        info!("Auto-selecting local LLM provider based on hardware...");
        let provider = local_manager.create_provider_auto().await
            .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            })?;
        Arc::from(provider)
    } else {
        // Use explicit provider type from config
        info!("Using configured provider type: {}", settings.local_provider.provider_type);
        
        let hardware = local_manager.detect_hardware();
        let config = local_manager.generate_model_config(&hardware);
        
        let provider = local_manager.create_provider(
            &settings.local_provider.provider_type,
            config
        ).await
            .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            })?;
        Arc::from(provider)
    };

    info!("Local LLM provider initialized: {}", llm_provider.model_name());

    // Initialize vision processor
    let vision_processor = Arc::new(geri::vision::VisionProcessor::new(
        settings.vision_model.clone(),
    ));

    // Start gRPC server
    let model_registry = Arc::new(ModelRegistry::default());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.grpc_port));
    let deps = geri::grpc::GrpcServerDependencies {
        model_registry,
        llm_provider,
        vision_processor,
    };
    let _server_handle = tokio::spawn(async move {
        if let Err(e) = geri::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Geri LLM Service started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Geri LLM Service...");

    Ok(())
}
