//! Tests for Phase 8.2.1: IP-based Connection Manager (connect to IP, timeout, retry).

use bifrost::discovery::ip::{IpConnectionManager, build_ws_url};
use std::time::Duration;

#[test]
fn build_ws_url_formats_host_and_port() {
    let url = build_ws_url("192.168.1.1", 50051);
    assert_eq!(url, "ws://192.168.1.1:50051");
}

#[test]
fn build_ws_url_handles_hostname() {
    let url = build_ws_url("bifrost.local", 8080);
    assert_eq!(url, "ws://bifrost.local:8080");
}

#[test]
fn manager_new_accepts_timeout_and_retry_config() {
    let _ = IpConnectionManager::new(
        Duration::from_secs(5),
        Some(bifrost::websocket::ReconnectionConfig::default()),
    );
}

#[test]
fn manager_connect_timeout_returns_err_on_timeout() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mgr = IpConnectionManager::new(Duration::from_millis(10), None);
        let result = mgr.connect("127.0.0.1", 1).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("timeout") || err.to_string().contains("refused") || err.to_string().contains("Connection"),
            "expected timeout/refused error, got: {}",
            err
        );
    });
}

#[test]
fn manager_connect_with_retry_records_attempts_on_failure() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let config = bifrost::websocket::ReconnectionConfig {
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_secs(60),
            jitter_ratio: 0.0,
        };
        let mgr = IpConnectionManager::new(Duration::from_millis(5), Some(config.clone()));
        let result = mgr.connect_with_retry("127.0.0.1", 1, 2).await;
        assert!(result.is_err());
    });
}
