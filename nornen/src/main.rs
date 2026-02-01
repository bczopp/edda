use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use nornen::utils::config::SettingsManager;
use nornen::urd::registry::ProviderRegistry;
use nornen::verdandi::router::RequestRouter;
use nornen::coordinator::NornenCoordinator;
use nornen::grpc::{start_grpc_server, GrpcServerDependencies};
use nornen::mimir_client::MimirClient;
use nornen::cache::ProviderCache;
use nornen::monitoring::collector::MetricsCollector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Nornen Decision Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/nornen.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}", settings.grpc_port);

    // Initialize Provider Cache first (if enabled)
    let cache = if let Some(cache_config) = &settings.cache {
        if cache_config.enabled {
            let cache = Arc::new(ProviderCache::new(cache_config.max_size, cache_config.ttl_seconds));
            info!("Provider Cache initialized: max_size={}, ttl={}s", cache_config.max_size, cache_config.ttl_seconds);
            Some(cache)
        } else {
            info!("Provider Cache disabled");
            None
        }
    } else {
        None
    };

    // Initialize Provider Registry (Urd) with cache if available
    let registry = if let Some(mimir_config) = &settings.mimir {
        // Use Mimir
        let mimir_client = Arc::new(MimirClient::new(mimir_config.url.clone()));
        mimir_client.connect().await?;
        info!("Connected to Mimir at {}", mimir_config.url);
        
        let registry = if let Some(cache) = cache.clone() {
            Arc::new(ProviderRegistry::new_with_mimir_and_cache(mimir_client, cache).await?)
        } else {
            Arc::new(ProviderRegistry::new_with_mimir(mimir_client).await?)
        };
        info!("Provider Registry (Urd) initialized with Mimir");
        registry
    } else {
        // Use PostgreSQL (legacy)
        let pool = sqlx::PgPool::connect(&settings.database.url).await?;
        info!("Database connection established");
        
        let registry = if let Some(cache) = cache.clone() {
            Arc::new(ProviderRegistry::new_with_cache(pool.clone(), cache).await?)
        } else {
            Arc::new(ProviderRegistry::new(pool.clone()).await?)
        };
        info!("Provider Registry (Urd) initialized with PostgreSQL");
        registry
    };

    // Initialize Request Router (Verdandi)
    let router = if let Some(cache) = cache.clone() {
        Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()))
    } else {
        Arc::new(RequestRouter::new(registry.clone()))
    };
    info!("Request Router (Verdandi) initialized");

    // Initialize Metrics Collector
    let metrics_collector = Arc::new(MetricsCollector::new(cache.clone()));
    info!("Metrics Collector initialized");

    // Initialize Nornen Coordinator
    let coordinator = Arc::new(NornenCoordinator::new(
        registry.clone(),
        router.clone(),
    ));
    info!("Nornen Coordinator initialized");

    // Initialize Access Control
    let mut access_control = crate::security::access_control::AccessControl::new();
    // Assign default roles (in production, this would come from configuration or database)
    access_control.assign_role("admin", crate::security::access_control::Role::Admin);
    access_control.assign_role("system", crate::security::access_control::Role::Admin);
    let access_control = Arc::new(access_control);
    info!("Access Control initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        coordinator: coordinator.clone(),
        registry: registry.clone(),
        router: router.clone(),
        access_control: access_control.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
