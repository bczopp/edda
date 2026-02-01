use tracing::info;
use freki::utils::config::SettingsManager;
use freki::utils::logging;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging()?;

    info!("Freki RAG Service starting...");

    let config_path = PathBuf::from("config/freki.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded");

    // Initialize Qdrant client
    let vector_db = Arc::new(freki::vector_db::VectorDbClient::new(&settings.qdrant_url).await?);
    
    // Ensure collection exists
    let collection_name = "documents".to_string();
    let vector_size = 384; // all-MiniLM-L6-v2 dimension
    if let Err(_) = vector_db.create_collection(&collection_name, vector_size).await {
        // Collection might already exist, which is fine
        info!("Collection '{}' already exists or creation failed", collection_name);
    }

    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.grpc_port));
    let audit_logger = Arc::new(freki::utils::AuditLogger::with_tracing());
    let deps = freki::grpc::GrpcServerDependencies {
        vector_db,
        collection_name,
        audit_logger,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = freki::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Freki RAG Service started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Freki RAG Service...");

    Ok(())
}
