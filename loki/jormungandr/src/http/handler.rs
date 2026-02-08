//! HTTP request handler – GET, POST, PUT, DELETE (Phase 7.2.1).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Invalid URL: {0}")]
    Url(String),
}

pub type Result<T> = std::result::Result<T, HttpError>;

/// HTTP request handler (GET, POST, PUT, DELETE).
pub struct HTTPRequestHandler {
    client: reqwest::Client,
}

impl HTTPRequestHandler {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self { client })
    }

    /// GET request; returns response body as string.
    pub async fn get(&self, url: &str) -> Result<String> {
        let resp = self.client.get(url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// POST request with body; returns response body as string.
    pub async fn post(&self, url: &str, body: &str) -> Result<String> {
        let resp = self
            .client
            .post(url)
            .body(body.to_string())
            .send()
            .await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// PUT request with body (optional).
    pub async fn put(&self, url: &str, body: &str) -> Result<String> {
        let resp = self
            .client
            .put(url)
            .body(body.to_string())
            .send()
            .await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// DELETE request (optional).
    pub async fn delete(&self, url: &str) -> Result<()> {
        self.client.delete(url).send().await?;
        Ok(())
    }
}
