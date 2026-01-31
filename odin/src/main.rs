use tracing::info;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Odin Main Orchestrator starting...");

    // Load settings
    let config_path = std::env::var("ODIN_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            if let Some(home) = dirs::home_dir() {
                home.join(".edda").join("odin").join("settings.json")
            } else {
                PathBuf::from("config").join("settings.json")
            }
        });
    
    let settings_manager = Arc::new(odin::utils::config::SettingsManager::new(config_path.clone()));
    settings_manager.load().await?;
    
    // Start hot-reload watcher
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    let grpc_port = settings.grpc_port;
    info!("Settings loaded, gRPC port: {}", grpc_port);
    
    // Wrap settings in Arc<RwLock> for sharing
    let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));

    // Initialize service registry
    let service_registry = Arc::new(odin::services::ServiceRegistry::new());
    
    // Initialize protocol manager
    let protocol_manager = Arc::new(odin::protocols::manager::ProtocolManager::new(settings_arc.clone()));
    
    // Initialize client manager
    let client_manager = Arc::new(odin::clients::manager::ClientManager::new(settings_arc.clone()));
    client_manager.initialize().await?;
    
    // Initialize responsibility manager
    let capability_cache = protocol_manager.get_cache();
    let responsibility_manager = Arc::new(odin::orchestration::responsibility::ResponsibilityManager::new(
        capability_cache,
        protocol_manager.clone(),
        client_manager.clone(),
    ));
    
    // Discover capabilities from all services and enabled plugins (Frigg, Valkyries)
    protocol_manager.discover_all_capabilities().await?;

    // Bootstrap Frigg/Valkyries as remote plugins when enabled and URL set
    let plugin_manager = Arc::new(odin::plugins::PluginManager::new());
    odin::bootstrap::bootstrap_frigg_valkyries_plugins(plugin_manager.as_ref(), protocol_manager.as_ref(), &settings_arc).await?;
    
    // Initialize request processor with responsibility manager
    let request_processor = Arc::new(odin::orchestration::RequestProcessor::new_with_responsibility(
        responsibility_manager,
    ));
    
    // Initialize action orchestrator with client manager
    let action_orchestrator = Arc::new(odin::orchestration::ActionOrchestrator::new_with_client(
        client_manager.clone(),
    ));
    
    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], grpc_port));
    let deps = odin::grpc::GrpcServerDependencies {
        request_processor,
        action_orchestrator,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = odin::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Odin Main Orchestrator started successfully on port {}", grpc_port);

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Odin Main Orchestrator...");

    Ok(())
}
