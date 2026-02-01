//! gRPC-Client für Eikthyrnir (Quality-Daten).

use std::time::Duration;
use thiserror::Error;
use tonic::transport::Endpoint;

pub mod eikthyrnir {
    tonic::include_proto!("eikthyrnir");
}

use eikthyrnir::eikthyrnir_service_client::EikthyrnirServiceClient;

#[derive(Debug, Error)]
pub enum EikthyrnirClientError {
    #[error("Transport: {0}")]
    Transport(String),
    #[error("gRPC: {0}")]
    Grpc(#[from] tonic::Status),
}

/// Domain-Typ für eine Quality-Metrik (von Eikthyrnir).
#[derive(Debug, Clone)]
pub struct QualityMetric {
    pub metric_id: String,
    pub provider_id: String,
    pub value: f64,
    pub timestamp: String,
}

/// Client für Eikthyrnir-Service (Quality-Daten abrufen).
pub struct EikthyrnirClient {
    endpoint: String,
}

impl EikthyrnirClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    /// Quality-Metriken für einen Provider abrufen.
    pub async fn get_quality_metrics(
        &self,
        provider_id: &str,
        limit: i32,
    ) -> Result<Vec<QualityMetric>, EikthyrnirClientError> {
        let channel = Endpoint::from_shared(self.endpoint.clone())
            .map_err(|e| EikthyrnirClientError::Transport(e.to_string()))?
            .connect_timeout(Duration::from_secs(5))
            .connect()
            .await
            .map_err(|e| EikthyrnirClientError::Transport(e.to_string()))?;

        let mut client = EikthyrnirServiceClient::new(channel);
        let req = eikthyrnir::GetQualityMetricsRequest {
            provider_id: provider_id.to_string(),
            limit: limit.max(0),
        };
        let res = client.get_quality_metrics(tonic::Request::new(req)).await?;
        let metrics = res
            .into_inner()
            .metrics
            .into_iter()
            .map(|m| QualityMetric {
                metric_id: m.metric_id,
                provider_id: m.provider_id,
                value: m.value,
                timestamp: m.timestamp,
            })
            .collect();
        Ok(metrics)
    }
}
