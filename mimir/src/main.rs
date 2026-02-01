use tracing::{info, error};
use std::path::PathBuf;
use std::sync::Arc;
use std::net::SocketAddr;
use mimir::utils::config::SettingsManager;
use mimir::storage::EncryptedDatabase;
use mimir::encryption::EncryptionManager;
use mimir::gdpr::GDPRCompliance;
use mimir::grpc::{start_grpc_server, GrpcServerDependencies};
use mimir::monitoring::PerformanceMonitor;
use mimir::cache::CacheManager;
use std::fs;
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration first so we can init logging (format, optional file + rotation)
    let config_path = PathBuf::from("config/mimir.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    let settings = settings_manager.get().await;

    // Init tracing: JSON for production, optional file with daily rotation
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let use_json = matches!(settings.logging.log_format, mimir::utils::config::LogFormat::Json);
    let log_dir = settings.logging.log_directory.as_deref();

    let (writer, _guard) = if let Some(dir) = log_dir {
        std::fs::create_dir_all(dir).ok();
        let file_appender = tracing_appender::rolling::daily(dir, "mimir.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        (tracing_subscriber::fmt::writer::MakeWriterExt::make_writer(non_blocking), Some(guard))
    } else {
        (tracing_subscriber::fmt::writer::MakeWriterExt::make_writer(std::io::stderr), None)
    };

    let fmt = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(writer);
    if use_json {
        fmt.json().with_current_span(false).with_span_list(false).init();
    } else {
        fmt.init();
    }

    info!("Mimir Privacy Database Service starting...");
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

    // Initialize performance monitor
    let performance_monitor = Arc::new(PerformanceMonitor::new());
    info!("Performance monitor initialized");
    
    // Initialize cache (optional - can be enabled via settings in future)
    let cache = Arc::new(CacheManager::new(1000, Duration::from_secs(300))); // 1000 entries, 5 min TTL
    info!("Cache initialized (max_size: 1000, ttl: 300s)");

    // Initialize Access Control and Audit Logging (optional - can be enabled via settings)
    let access_control = if settings.security.enable_access_control {
        Some(Arc::new(mimir::access_control::AccessControlManager::new()))
    } else {
        None
    };

    let audit_logger = if settings.security.enable_audit_logging {
        // Create a pool for audit logging with connection pool configuration
        let audit_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(settings.database.max_connections)
            .min_connections(settings.database.min_connections)
            .connect(&settings.database.url)
            .await?;
        Some(Arc::new(mimir::audit::AuditLogManager::new(audit_pool)))
    } else {
        None
    };

    // Create database pool with connection pool configuration
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .min_connections(settings.database.min_connections)
        .connect(&settings.database.url)
        .await?;
    info!("Database connection established (max_connections: {}, min_connections: {})", 
          settings.database.max_connections, settings.database.min_connections);

    // Initialize database with all features
    let database = Arc::new(
        EncryptedDatabase::new_with_all_features(
            &pool,
            encryption_manager,
            access_control.clone(),
            audit_logger.clone(),
            Some(cache.clone()),
            Some(performance_monitor.clone()),
        ).await?
    );

    // Initialize GDPR compliance
    let gdpr = Arc::new(GDPRCompliance::new_with_database(database.clone()));

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies::new(database.clone(), gdpr);
    let deps = GrpcServerDependencies {
        database: deps.database,
        gdpr: deps.gdpr,
        access_control,
        audit_logger,
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
