// General test helpers (service readiness, URLs).

use std::time::Duration;
use tokio::time::sleep;

/// Wait for a service to be ready (TCP connect). URL may be http(s)://host:port or host:port.
pub async fn wait_for_service(url: &str, max_retries: u32) -> bool {
    let (host, port) = parse_host_port(url).unwrap_or(("localhost", 0));
    if port == 0 {
        return false;
    }
    let addr = format!("{}:{}", host, port);
    for _ in 0..max_retries {
        if tokio::net::TcpStream::connect(&addr).await.is_ok() {
            return true;
        }
        sleep(Duration::from_millis(500)).await;
    }
    false
}

fn parse_host_port(url: &str) -> Option<(&str, u16)> {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);
    let (host, port_str) = rest.split_once(':')?;
    let port: u16 = port_str.parse().ok()?;
    Some((host, port))
}

/// Get service URL from environment or use default.
pub fn get_service_url(service_name: &str, default_port: u16) -> String {
    let env_var = format!("{}_URL", service_name.to_uppercase().replace('-', "_"));
    std::env::var(&env_var).unwrap_or_else(|_| format!("http://localhost:{}", default_port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_service_url_uses_default_when_env_unset() {
        let url = get_service_url("postgres", 5432);
        assert!(url.contains("5432"));
    }

    #[test]
    fn parse_host_port_parses_http_url() {
        let (h, p) = parse_host_port("http://localhost:8080").unwrap();
        assert_eq!(h, "localhost");
        assert_eq!(p, 8080);
    }
}
