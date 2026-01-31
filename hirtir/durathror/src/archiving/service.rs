use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArchivingError {
    #[error("Archiving failed: {0}")]
    ArchivingFailed(String),
}

pub struct ArchivingService {
    s3_endpoint: String,
    s3_bucket: String,
}

impl ArchivingService {
    pub fn new(s3_endpoint: String, s3_bucket: String) -> Self {
        Self {
            s3_endpoint,
            s3_bucket,
        }
    }

    pub async fn archive(&self, data_id: &str, archive_location: &str) -> Result<String, ArchivingError> {
        // TODO: Implement S3 archiving
        Ok(format!("archive_{}", data_id))
    }
}
