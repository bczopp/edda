use tracing::info;
use std::sync::Arc;
use std::path::PathBuf;
use std::net::SocketAddr;
use forseti::utils::config::SettingsManager;
use forseti::python::runtime::PythonRuntime;
use forseti::models::registry::ModelRegistry;
use forseti::training::manager::TrainingManager;
use forseti::rl::agent::RLAgentTrainer;
use forseti::inference::engine::InferenceEngine;
use forseti::grpc::{start_grpc_server, GrpcServerDependencies};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Forseti ML/DL/RL Service starting...");

    // Load configuration
    let config_path = PathBuf::from("config/forseti.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: gRPC port={}", settings.grpc_port);

    // Initialize database connection pool
    let pool = sqlx::PgPool::connect(&settings.database.url).await?;
    info!("Database connection established");

    // Initialize Python Runtime
    let python_runtime = Arc::new(PythonRuntime::new(settings.python.python_path.clone())?);
    info!("Python Runtime initialized");

    // Initialize Model Registry
    let model_registry = Arc::new(ModelRegistry::new(pool.clone()));
    info!("Model Registry initialized");

    // Initialize Training Manager
    let training_manager = Arc::new(TrainingManager::new(python_runtime.clone()));
    info!("Training Manager initialized");

    // Initialize RL Agent Trainer
    let rl_trainer = Arc::new(RLAgentTrainer::new(python_runtime.clone()));
    info!("RL Agent Trainer initialized");

    // Initialize Inference Engine
    let inference_engine = Arc::new(InferenceEngine::new());
    info!("Inference Engine initialized");

    // Start gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let deps = GrpcServerDependencies {
        training_manager: training_manager.clone(),
        rl_trainer: rl_trainer.clone(),
        inference_engine: inference_engine.clone(),
        model_registry: model_registry.clone(),
    };

    info!("Starting gRPC server on {}", addr);
    start_grpc_server(addr, deps).await?;

    Ok(())
}
