use tracing::{info, error};
use heimdall::utils::config::SettingsManager;
use heimdall::grpc::start_grpc_server;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Heimdall Security Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/heimdall.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded");

    // Initialize database
    let db_manager = heimdall::utils::database::DatabaseManager::new(&settings.database_url).await?;
    let pool = db_manager.pool();
    
    // Initialize key management
    let keys_dir = PathBuf::from("keys");
    std::fs::create_dir_all(&keys_dir)?;
    
    // Generate or load Heimdall signing keypair
    let key_generator = heimdall::keys::KeyGenerator::new();
    let signing_keypair = if keys_dir.join("heimdall.key").exists() {
        let key_storage = heimdall::keys::SecureKeyStorage::new(keys_dir.clone());
        Arc::new(key_storage.load_keypair("heimdall")?)
    } else {
        let (keypair, pkcs8) = key_generator.generate_ed25519_keypair()?;
        let key_storage = heimdall::keys::SecureKeyStorage::new(keys_dir.clone());
        key_storage.store_keypair("heimdall", &pkcs8)?;
        Arc::new(keypair)
    };
    
    // Initialize repositories
    let device_repo = Arc::new(heimdall::utils::DeviceRepository::new(pool.clone()));
    let token_repo = Arc::new(heimdall::utils::TokenRepository::new(pool.clone()));
    
    // Initialize authentication components
    let challenge_generator = Arc::new(heimdall::auth::ChallengeGenerator::new(
        keys_dir.clone(),
        device_repo.clone(),
    ));
    
    let token_generator = Arc::new(heimdall::token::TokenGenerator::new(
        signing_keypair.clone(),
        settings.token_configuration.clone(),
    ));
    
    let auth_manager = Arc::new(heimdall::auth::AuthenticationManager::new(
        challenge_generator,
        token_generator.clone(),
        device_repo.clone(),
        token_repo.clone(),
        pool.clone(),
    ));

    // Initialize caches
    let token_cache = Arc::new(heimdall::utils::TokenValidationCache::new(300)); // 5 minutes
    let permission_cache = Arc::new(heimdall::utils::PermissionCheckCache::new(300)); // 5 minutes

    // Initialize authorization components with cache
    let permission_manager = Arc::new(heimdall::authz::PermissionManager::with_cache(
        pool.clone(),
        device_repo.clone(),
        permission_cache.clone(),
    ));

    // Initialize token validation with cache
    let token_validator = Arc::new(heimdall::token::TokenValidator::with_cache(
        keys_dir.clone(),
        token_cache.clone(),
    ));

    // Initialize guest network manager
    let guest_network_manager = Arc::new(heimdall::guest::GuestNetworkManager::new(
        pool.clone(),
        device_repo.clone(),
    ));

    // Initialize Bifrost validation
    let connection_validator = Arc::new(heimdall::bifrost::ConnectionValidator::with_guest_network(
        device_repo.clone(),
        permission_manager.clone(),
        guest_network_manager.clone(),
    ));

    // Initialize mesh registry
    let mesh_registry = Arc::new(heimdall::mesh::MeshDeviceRegistry::new(
        pool.clone(),
        device_repo.clone(),
    ));

    // Initialize session management
    let session_repo = Arc::new(heimdall::utils::SessionRepository::new(pool.clone()));
    let session_manager = Arc::new(heimdall::utils::SessionManager::new(
        session_repo.clone(),
        token_repo.clone(),
        settings.session_management.session_timeout_hours,
    ));
    session_manager.start_cleanup_task();

    // Initialize audit logging
    let _audit_logger = Arc::new(heimdall::security::AuditLogger::new(pool.clone()));

    // Start gRPC server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.grpc_port));
    let deps = heimdall::grpc::GrpcServerDependencies {
        auth_manager,
        permission_manager,
        token_validator,
        token_repo: token_repo.clone(),
        token_generator,
        connection_validator,
        mesh_registry,
        signing_keypair,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = start_grpc_server(addr, deps).await {
            error!("gRPC server error: {}", e);
        }
    });

    info!("Heimdall Security Service started successfully on port {}", settings.grpc_port);

    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Heimdall Security Service...");
    
    server_handle.abort();

    Ok(())
}
