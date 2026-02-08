use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "unknown".to_string());
    let port: u16 = env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "50050".to_string())
        .parse()
        .unwrap_or(50050);

    info!("Starting mock service {} on port {}", service_name, port);

    // Explizit als SocketAddr parsen, um Typ-Inferenz-Probleme zu vermeiden.
    let addr: std::net::SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Mock service {} listening on {}", service_name, addr);

    loop {
        match listener.accept().await {
            Ok((_stream, _addr)) => {
                info!("Mock service {} received connection", service_name);
            }
            Err(e) => {
                error!("Error accepting connection: {}", e);
            }
        }
    }
}
