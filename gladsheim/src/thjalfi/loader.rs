use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("Service loading failed: {0}")]
    LoadingFailed(String),
}

pub struct ServiceLoader {
    service_directory: String,
}

impl ServiceLoader {
    pub fn new(service_directory: String) -> Self {
        Self { service_directory }
    }

    pub async fn load_service(&self, service_name: &str) -> Result<(), LoaderError> {
        // TODO: Load service from directory
        Ok(())
    }
}
