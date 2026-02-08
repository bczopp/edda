use std::time::Duration;
use tokio::time::sleep;

/// Wait for a service to be ready (TCP connect to host:port or "host:port" string).
pub async fn wait_for_service(url: &str, max_retries: u32) -> bool {
    for _ in 0..max_retries {
        if tokio::net::TcpStream::connect(url).await.is_ok() {
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

/// Assert that a service is listening on the given address (e.g. "127.0.0.1:50051").
/// Fails the test after timeout if not reachable.
pub async fn assert_service_listens(addr: &str, timeout: Duration) {
    let deadline = tokio::time::Instant::now() + timeout;
    while tokio::time::Instant::now() < deadline {
        if tokio::net::TcpStream::connect(addr).await.is_ok() {
            return;
        }
        sleep(Duration::from_millis(100)).await;
    }
    panic!("service at {} did not become reachable within {:?}", addr, timeout);
}

/// Assert resource usage is within bounds (for tests that check Byggvir/limits).
#[allow(dead_code)]
pub fn assert_resource_within_bounds(
    memory_mb: u64,
    max_memory_mb: u64,
    cpu_percent: f32,
    max_cpu_percent: f32,
) {
    assert!(
        memory_mb <= max_memory_mb,
        "memory {} MB exceeds max {} MB",
        memory_mb,
        max_memory_mb
    );
    assert!(
        cpu_percent <= max_cpu_percent,
        "cpu {}% exceeds max {}%",
        cpu_percent,
        max_cpu_percent
    );
}

/// Get test gRPC address from env GLADSHEIM_TEST_GRPC_ADDR or default.
#[allow(dead_code)]
pub fn test_grpc_addr() -> String {
    std::env::var("GLADSHEIM_TEST_GRPC_ADDR").unwrap_or_else(|_| "127.0.0.1:50051".to_string())
}
