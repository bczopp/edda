use tracing::info;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Muninn TTS Service starting...");

    // Initialize TTS engine
    let tts_engine = Arc::new(muninn::tts::TTSEngine::new());
    
    // Start gRPC server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 50058));
    let deps = muninn::grpc::GrpcServerDependencies {
        tts_engine,
    };
    let server_handle = tokio::spawn(async move {
        if let Err(e) = muninn::grpc::start_grpc_server(addr, deps).await {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    info!("Muninn TTS Service started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down Muninn TTS Service...");

    Ok(())
}
