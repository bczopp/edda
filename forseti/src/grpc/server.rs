use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod forseti {
    tonic::include_proto!("forseti");
}

use forseti::forseti_service_server::{ForsetiService, ForsetiServiceServer};

pub struct ForsetiServiceImpl {
    training_manager: Arc<crate::training::TrainingManager>,
    rl_trainer: Arc<crate::rl::RLAgentTrainer>,
    inference_engine: Arc<crate::inference::InferenceEngine>,
    model_registry: Arc<crate::models::ModelRegistry>,
}

impl ForsetiServiceImpl {
    pub fn new(
        training_manager: Arc<crate::training::TrainingManager>,
        rl_trainer: Arc<crate::rl::RLAgentTrainer>,
        inference_engine: Arc<crate::inference::InferenceEngine>,
        model_registry: Arc<crate::models::ModelRegistry>,
    ) -> Self {
        Self {
            training_manager,
            rl_trainer,
            inference_engine,
            model_registry,
        }
    }
}

#[tonic::async_trait]
impl ForsetiService for ForsetiServiceImpl {
    async fn train_model(
        &self,
        request: Request<forseti::TrainModelRequest>,
    ) -> Result<Response<forseti::TrainModelResponse>, Status> {
        let req = request.into_inner();
        
        let model_id = self.training_manager.train_model(&req.model_type, &req.training_data).await
            .map_err(|e| Status::internal(format!("Training failed: {}", e)))?;

        // Register model
        self.model_registry.register_model(&model_id, &req.model_type, "pytorch").await
            .map_err(|e| Status::internal(format!("Model registration failed: {}", e)))?;

        let training_job_id = uuid::Uuid::new_v4().to_string();

        Ok(Response::new(forseti::TrainModelResponse {
            model_id,
            status: "training".to_string(),
            training_job_id,
        }))
    }

    async fn fine_tune_model(
        &self,
        request: Request<forseti::FineTuneModelRequest>,
    ) -> Result<Response<forseti::FineTuneModelResponse>, Status> {
        let req = request.into_inner();
        
        let fine_tuned_model_id = uuid::Uuid::new_v4().to_string();
        
        // Register fine-tuned model
        self.model_registry.register_model(&fine_tuned_model_id, "fine_tuned", "pytorch").await
            .map_err(|e| Status::internal(format!("Model registration failed: {}", e)))?;

        Ok(Response::new(forseti::FineTuneModelResponse {
            fine_tuned_model_id,
            status: "fine_tuning".to_string(),
        }))
    }

    async fn train_rl_agent(
        &self,
        request: Request<forseti::TrainRLAgentRequest>,
    ) -> Result<Response<forseti::TrainRLAgentResponse>, Status> {
        let req = request.into_inner();
        
        let config_bytes = serde_json::to_vec(&req.environment_config).unwrap_or_default();
        
        let agent_id = self.rl_trainer.train_agent(&req.algorithm, &config_bytes).await
            .map_err(|e| Status::internal(format!("RL training failed: {}", e)))?;

        let training_job_id = uuid::Uuid::new_v4().to_string();

        Ok(Response::new(forseti::TrainRLAgentResponse {
            agent_id,
            status: "training".to_string(),
            training_job_id,
        }))
    }

    async fn run_inference(
        &self,
        request: Request<forseti::RunInferenceRequest>,
    ) -> Result<Response<forseti::RunInferenceResponse>, Status> {
        let req = request.into_inner();
        
        let output = self.inference_engine.run_inference(&req.model_id, &req.input).await
            .map_err(|e| Status::internal(format!("Inference failed: {}", e)))?;

        Ok(Response::new(forseti::RunInferenceResponse {
            output,
        }))
    }

    async fn export_model(
        &self,
        request: Request<forseti::ExportModelRequest>,
    ) -> Result<Response<forseti::ExportModelResponse>, Status> {
        let req = request.into_inner();
        
        let export_id = uuid::Uuid::new_v4().to_string();
        let export_path = format!("exports/{}.{}", req.model_id, req.export_format);

        Ok(Response::new(forseti::ExportModelResponse {
            success: true,
            export_path,
            export_id,
        }))
    }

    async fn get_model_status(
        &self,
        request: Request<forseti::GetModelStatusRequest>,
    ) -> Result<Response<forseti::GetModelStatusResponse>, Status> {
        let req = request.into_inner();
        
        let model_type = self.model_registry.get_model(&req.model_id).await
            .map_err(|e| Status::internal(format!("Failed to get model: {}", e)))?;

        Ok(Response::new(forseti::GetModelStatusResponse {
            status: "ready".to_string(),
            progress: 100.0,
            metadata: std::collections::HashMap::from([
                ("model_type".to_string(), model_type),
            ]),
        }))
    }
}

pub struct GrpcServerDependencies {
    pub training_manager: Arc<crate::training::TrainingManager>,
    pub rl_trainer: Arc<crate::rl::RLAgentTrainer>,
    pub inference_engine: Arc<crate::inference::InferenceEngine>,
    pub model_registry: Arc<crate::models::ModelRegistry>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Forseti gRPC server on {}", addr);

    let forseti_service = ForsetiServiceImpl::new(
        deps.training_manager,
        deps.rl_trainer,
        deps.inference_engine,
        deps.model_registry,
    );

    Server::builder()
        .add_service(ForsetiServiceServer::new(forseti_service))
        .serve(addr)
        .await?;

    Ok(())
}
