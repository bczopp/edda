use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoordinatorError {
    #[error("Coordination failed: {0}")]
    CoordinationFailed(String),
}

pub struct ServiceCoordinator {
    dainn: Arc<dainn::IndexingService>,
    dvalinn: Arc<dvalinn::ValidationService>,
    duneyrr: Arc<duneyrr::AggregationService>,
    durathror: Arc<durathror::ArchivingService>,
}

impl ServiceCoordinator {
    pub fn new(
        dainn: Arc<dainn::IndexingService>,
        dvalinn: Arc<dvalinn::ValidationService>,
        duneyrr: Arc<duneyrr::AggregationService>,
        durathror: Arc<durathror::ArchivingService>,
    ) -> Self {
        Self {
            dainn,
            dvalinn,
            duneyrr,
            durathror,
        }
    }

    pub async fn index_data(&self, data_id: &str, data: &[u8], metadata: &std::collections::HashMap<String, String>) -> Result<String, CoordinatorError> {
        self.dainn.index(data_id, data, metadata).await
            .map_err(|e| CoordinatorError::CoordinationFailed(format!("{}", e)))
    }

    pub async fn validate_data(&self, schema_id: &str, data: &[u8]) -> Result<bool, CoordinatorError> {
        self.dvalinn.validate(schema_id, data).await
            .map_err(|e| CoordinatorError::CoordinationFailed(format!("{}", e)))
    }

    pub async fn aggregate_data(&self, aggregation_type: &str, data_ids: &[String]) -> Result<Vec<u8>, CoordinatorError> {
        self.duneyrr.aggregate(aggregation_type, data_ids).await
            .map_err(|e| CoordinatorError::CoordinationFailed(format!("{}", e)))
    }

    pub async fn archive_data(&self, data_id: &str, archive_location: &str) -> Result<String, CoordinatorError> {
        self.durathror.archive(data_id, archive_location).await
            .map_err(|e| CoordinatorError::CoordinationFailed(format!("{}", e)))
    }
}
