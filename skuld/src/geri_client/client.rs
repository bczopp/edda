//! Geri gRPC Client for Model Registry access

use tonic::transport::Channel;
use tonic::{Request, Response, Status};

pub mod geri {
    tonic::include_proto!("geri");
}

use geri::geri_service_client::GeriServiceClient;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub model_type: String,
    pub parameter_count: u64,
    pub hardware_requirements: String,
    pub context_window: u32,
}

impl From<geri::ModelInfo> for ModelInfo {
    fn from(proto: geri::ModelInfo) -> Self {
        Self {
            id: proto.id,
            name: proto.name,
            provider: proto.provider,
            model_type: proto.model_type,
            parameter_count: proto.parameter_count,
            hardware_requirements: proto.hardware_requirements,
            context_window: proto.context_window,
        }
    }
}

pub struct GeriClient {
    client: GeriServiceClient<Channel>,
}

impl GeriClient {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = GeriServiceClient::connect(addr).await?;
        Ok(Self { client })
    }

    /// List all models, optionally filtered by type and/or provider
    pub async fn list_models(
        &mut self,
        model_type: Option<&str>,
        provider: Option<&str>,
    ) -> Result<Vec<ModelInfo>, Status> {
        let request = Request::new(geri::ListModelsRequest {
            model_type: model_type.unwrap_or("").to_string(),
            provider: provider.unwrap_or("").to_string(),
        });

        let response = self.client.list_models(request).await?;
        let models = response.into_inner().models.into_iter()
            .map(ModelInfo::from)
            .collect();
        
        Ok(models)
    }

    /// Get detailed information about a specific model
    pub async fn get_model_info(&mut self, model_id: &str) -> Result<ModelInfo, Status> {
        let request = Request::new(geri::GetModelInfoRequest {
            model_id: model_id.to_string(),
        });

        let response = self.client.get_model_info(request).await?;
        let model = response.into_inner().model
            .ok_or_else(|| Status::not_found("Model not found"))?;
        
        Ok(ModelInfo::from(model))
    }
}
