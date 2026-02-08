// WiFiManager tests (Phase 3.1.1, TDD).

use jotunheim_esp32::network::{WiFiManager, WiFiStatus};

#[tokio::test]
async fn new_creates_manager_with_credentials() {
    let mgr = WiFiManager::new("MySSID".into(), "secret".into());
    assert_eq!(mgr.status(), WiFiStatus::Disconnected);
}

#[tokio::test]
async fn connect_returns_ok_and_status_becomes_connected() {
    let mgr = WiFiManager::new("TestSSID".into(), "pass".into());
    let res = mgr.connect().await;
    assert!(res.is_ok());
    assert_eq!(mgr.status(), WiFiStatus::Connected);
}

#[tokio::test]
async fn reconnect_after_connect_succeeds() {
    let mgr = WiFiManager::new("SSID".into(), "pwd".into());
    mgr.connect().await.unwrap();
    assert_eq!(mgr.status(), WiFiStatus::Connected);
    let res = mgr.reconnect().await;
    assert!(res.is_ok());
    assert_eq!(mgr.status(), WiFiStatus::Connected);
}

#[tokio::test]
async fn status_initially_disconnected() {
    let mgr = WiFiManager::new("x".into(), "y".into());
    assert_eq!(mgr.status(), WiFiStatus::Disconnected);
}
