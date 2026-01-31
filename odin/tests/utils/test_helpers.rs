use std::time::Duration;
use tokio::time::sleep;

/// Wait for a service to be ready
/// Parses URL to extract host and port for TCP connection check
pub async fn wait_for_service(url: &str, max_retries: u32) -> bool {
    // Parse URL to get host and port
    let (host, port) = parse_url(url);
    
    for _ in 0..max_retries {
        if let Ok(_) = tokio::net::TcpStream::connect((host.as_str(), port)).await {
            return true;
        }
        sleep(Duration::from_millis(500)).await;
    }
    false
}

/// Parse URL to extract host and port
fn parse_url(url: &str) -> (String, u16) {
    // Remove http:// or https:// prefix
    let url = url.trim_start_matches("http://").trim_start_matches("https://");
    
    // Split by colon to get host and port
    if let Some(colon_pos) = url.find(':') {
        let host = url[..colon_pos].to_string();
        let port_str = &url[colon_pos + 1..];
        if let Ok(port) = port_str.parse::<u16>() {
            return (host, port);
        }
    }
    
    // Default to localhost:port if parsing fails
    (url.to_string(), 50052)
}

/// Get service URL from environment or use default
pub fn get_service_url(service_name: &str, default_port: u16) -> String {
    let env_var = format!("{}_URL", service_name.to_uppercase());
    std::env::var(&env_var).unwrap_or_else(|_| {
        format!("http://localhost:{}", default_port)
    })
}
