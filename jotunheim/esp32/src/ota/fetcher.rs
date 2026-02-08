//! Update fetcher abstraction (Phase 7.1.1) – for OTA download and tests.

use async_trait::async_trait;

/// Fetches update metadata and payload (implemented by HTTP client or mock).
#[async_trait]
pub trait UpdateFetcher: Send + Sync {
    async fn check_available(&self, url: &str) -> bool;
    async fn download(&self, url: &str) -> Result<Vec<u8>, String>;
}
