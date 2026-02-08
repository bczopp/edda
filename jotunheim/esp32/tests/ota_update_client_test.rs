// OTAUpdateClient tests (Phase 7.1.1, TDD).

use jotunheim_esp32::ota::{OtaUpdateClient, UpdateFetcher};
use std::sync::atomic::{AtomicBool, Ordering};

struct MockFetcher {
    available: bool,
    payload: Vec<u8>,
}

#[async_trait::async_trait]
impl UpdateFetcher for MockFetcher {
    async fn check_available(&self, _url: &str) -> bool {
        self.available
    }
    async fn download(&self, _url: &str) -> Result<Vec<u8>, String> {
        if self.available {
            Ok(self.payload.clone())
        } else {
            Err("not available".into())
        }
    }
}

#[tokio::test]
async fn check_available_returns_fetcher_result() {
    let fetcher = MockFetcher { available: true, payload: vec![] };
    let client = OtaUpdateClient::new("http://test".into(), fetcher);
    assert!(client.check_available().await);
}

#[tokio::test]
async fn download_returns_payload_when_available() {
    let fetcher = MockFetcher {
        available: true,
        payload: b"firmware v2".to_vec(),
    };
    let client = OtaUpdateClient::new("http://test".into(), fetcher);
    let data = client.download().await.unwrap();
    assert_eq!(data, b"firmware v2");
}

#[tokio::test]
async fn install_stores_pending_then_restart_callback_called() {
    let fetcher = MockFetcher {
        available: true,
        payload: b"fw".to_vec(),
    };
    let restart_called = std::sync::Arc::new(AtomicBool::new(false));
    let r = restart_called.clone();
    let client = OtaUpdateClient::new("http://t".into(), fetcher)
        .on_restart(move || r.store(true, Ordering::SeqCst));
    let data = client.download().await.unwrap();
    client.install(&data).await.unwrap();
    client.restart().await;
    assert!(restart_called.load(Ordering::SeqCst));
}
