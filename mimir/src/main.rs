use tracing::{info, error};
use std::path::PathBuf;
use std::sync::Arc;
use std::net::SocketAddr;
use mimir::utils::config::SettingsManager;
use mimir::storage::EncryptedDatabase;
use mimir::encryption::EncryptionManager;
use mimir::gdpr::GDPRCompliance;
use mimir::grpc::{start_grpc_server, GrpcServerDependencies};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Mimir Privacy Database Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/mimir.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}", settings.grpc_port);

    // Load or generate encryption key
    let key = if PathBuf::from(&settings.encryption_key_path).exists() {
        fs::read(&settings.encryption_key_path)?
    } else {
        // Generate new key
        use ring::rand::{SecureRandom, SystemRandom};
        let mut key = vec![0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut key)?;
        
        // Create directory if it doesn't exist
        if let Some(parent) = PathBuf::from(&settings.encryption_key_path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&settings.encryption_key_path, &key)?;
        info!("Generated new encryption key at {}", settings.encryption_key_path);
        key
    };

    // Initialize encryption manager
    let encryption_manager = EncryptionManager::new(&key)?;
    info!("Encryption manager initialized");

    // Initialize database with encryption
    let database = Arc::new(
        EncryptedDatabase::new_with_encryption_manager(
            &settings.database.url,
            encryption_manager,
        ).await?
    );
    info!("Database connection established");

    // Initialize GDPR compliance
    // Note: GDPRCompliance needs to be updated to accept Arc<EncryptedDatabase>
    // For now, we'll create it with a reference
    let gdpr = Arc::new(GDPRCompliance::new_with_database(database.clone()));

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        database: database.clone(),
        gdpr,
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
