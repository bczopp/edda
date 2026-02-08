use tracing::{info, error};
use thor::utils::config::SettingsManager;
use thor::grpc::start_grpc_server;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Thor Action Executor Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/thor.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded");

    // Initialize permission checker
    let permission_checker = Arc::new(thor::permissions::PermissionChecker::new(
        settings.heimdall_url.clone(),
    ));

    // Initialize action registry
    let registry = Arc::new(thor::actions::ActionRegistry::new());
    
    // Register action executors
    registry.register(Arc::new(thor::file::FileActionExecutor)).await;
    registry.register(Arc::new(thor::system::SystemCommandExecutor)).await;
    registry.register(Arc::new(thor::network::NetworkActionExecutor)).await;
    registry.register(Arc::new(thor::app::AppControlExecutor)).await;
    
    // Register new action handlers
    registry.register(Arc::new(thor::terminal::TerminalActionHandler::new())).await;
    registry.register(Arc::new(thor::ui_automation::UIAutomationHandler::new())).await;
    registry.register(Arc::new(thor::scheduler::SchedulerActionHandler::new())).await;
    
    // Register Jotunheim handler (requires URL from settings)
    if let Some(jotunheim_url) = &settings.jotunheim_url {
        registry.register(Arc::new(thor::jotunheim::JotunheimActionHandler::new(jotunheim_url.clone()))).await;
    }

    // Register sandbox handler
    registry.register(Arc::new(thor::sandbox::SandboxActionExecutor)).await;

    // Initialize action dispatcher (with optional audit logging)
    let dispatcher = if settings.enable_audit_logging {
        Arc::new(thor::actions::ActionDispatcher::new_with_audit(
            registry.clone(),
            permission_checker.clone(),
            thor::audit::TracingAuditLogger::new(),
            settings.enable_sandboxing,
        ))
    } else {
        Arc::new(thor::actions::ActionDispatcher::new(
            registry.clone(),
            permission_checker.clone(),
            settings.enable_sandboxing,
        ))
    };

    // Start gRPC server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.grpc_port));
    let deps = thor::grpc::GrpcServerDependencies {
        dispatcher,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = start_grpc_server(addr, deps).await {
            error!("gRPC server error: {}", e);
        }
    });

    info!("Thor Action Executor Service started successfully on port {}", settings.grpc_port);

    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Thor Action Executor Service...");
    
    server_handle.abort();

    Ok(())
}
