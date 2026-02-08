//! OTAUpdateClient (Phase 7.1.1, TDD).

use super::fetcher::UpdateFetcher;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OtaUpdateError {
    #[error("Update not available")]
    NotAvailable,
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("Install failed: {0}")]
    InstallFailed(String),
}

/// Checks availability, downloads, installs, and triggers restart.
pub struct OtaUpdateClient<F> {
    base_url: String,
    fetcher: F,
    on_restart: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl<F> OtaUpdateClient<F>
where
    F: UpdateFetcher,
{
    pub fn new(base_url: String, fetcher: F) -> Self {
        Self {
            base_url,
            fetcher,
            on_restart: None,
        }
    }

    pub fn on_restart(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_restart = Some(Arc::new(f));
        self
    }

    pub async fn check_available(&self) -> bool {
        self.fetcher.check_available(&self.base_url).await
    }

    pub async fn download(&self) -> Result<Vec<u8>, OtaUpdateError> {
        if !self.check_available().await {
            return Err(OtaUpdateError::NotAvailable);
        }
        self.fetcher
            .download(&self.base_url)
            .await
            .map_err(OtaUpdateError::DownloadFailed)
    }

    pub async fn install(&self, _data: &[u8]) -> Result<(), OtaUpdateError> {
        // On host: no-op; on device: write to partition
        Ok(())
    }

    pub async fn restart(&self) {
        if let Some(ref f) = self.on_restart {
            f();
        }
    }
}
