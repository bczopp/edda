use tracing::info;
use std::sync::Arc;
use nidhoggr::utils::config::{NidhoggrSettings, SettingsManager};
use nidhoggr::connection::ConnectionManager;
use nidhoggr::routing::MessageRouter;
use nidhoggr::ratelimiter::RateLimiter;
use nidhoggr::websocket::WebSocketServer;
use nidhoggr::clients::ClientManager;
use nidhoggr::security::SecurityMonitor;
use nidhoggr::security::audit::AuditLogger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Nidhöggr Connection Endpoint Service starting...");

    // Load settings
    let settings = NidhoggrSettings::load(None)?;
    let settings_manager = SettingsManager::new(settings.clone())?;

    // Initialize connection manager
    let connection_manager = Arc::new(ConnectionManager::new());
    
    // Initialize client manager
    let client_manager = Arc::new(
        ClientManager::new(settings.service_endpoints.clone())
            .await
            .map_err(|e| format!("Failed to initialize client manager: {}", e))?
    );
    info!("Client manager initialized");
    
    // Initialize router
    let router = Arc::new(MessageRouter::new(connection_manager.clone(), client_manager));
    
    // Initialize rate limiter
    let rate_limiter = Arc::new(RateLimiter::new(
        settings.rate_limit_per_minute,
        settings.rate_limit_per_hour,
    ));
    
    // Initialize security monitor
    let security_monitor = Arc::new(SecurityMonitor::new(10000)); // Keep last 10000 events
    
    // Initialize audit logger
    let audit_logger = Arc::new(AuditLogger::new(10000)); // Keep last 10000 logs
    
    // Start WebSocket server
    let websocket_server = WebSocketServer::new_with_tls(
        settings.websocket_port,
        connection_manager.clone(),
        router,
        rate_limiter,
        settings.tls_cert_path.clone(),
        settings.tls_key_path.clone(),
        security_monitor,
        audit_logger,
    );
    
    let websocket_handle = tokio::spawn(async move {
        if let Err(e) = websocket_server.start().await {
            tracing::error!("WebSocket server error: {}", e);
        }
    });
    
    // Initialize endpoint handler for gRPC
    let endpoint_handler = Arc::new(nidhoggr::endpoint::EndpointHandler::new(connection_manager));
    
    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.grpc_port));
    let deps = nidhoggr::grpc::GrpcServerDependencies {
        endpoint_handler,
    };
    let grpc_handle = tokio::spawn(async move {
        if let Err(e) = nidhoggr::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Nidhöggr Connection Endpoint Service started successfully");
    info!("WebSocket server listening on port {}", settings.websocket_port);
    info!("gRPC server listening on port {}", settings.grpc_port);

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Nidhöggr Connection Endpoint Service...");
    
    websocket_handle.abort();
    grpc_handle.abort();

    Ok(())
}
