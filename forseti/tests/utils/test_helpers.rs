use std::time::Duration;
use tokio::time::sleep;

/// Wait for a service to be ready
pub async fn wait_for_service(url: &str, max_retries: u32) -> bool {
    for _ in 0..max_retries {
        if let Ok(_) = tokio::net::TcpStream::connect(url).await {
            return true;
        }
        sleep(Duration::from_millis(500)).await;
    }
    false
}

/// Get service URL from environment or use default
pub fn get_service_url(service_name: &str, default_port: u16) -> String {
    let env_var = format!("{}_URL", service_name.to_uppercase());
    std::env::var(&env_var).unwrap_or_else(|_| {
        format!("http://localhost:{}", default_port)
    })
}
