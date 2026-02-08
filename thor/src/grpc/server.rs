use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

// Include generated protobuf code
pub mod thor {
    tonic::include_proto!("thor");
}

use thor::thor_service_server::{ThorService, ThorServiceServer};

pub struct ThorServiceImpl {
    dispatcher: Arc<crate::actions::ActionDispatcher>,
    xml_dispatcher: crate::actions::XmlDispatcher,
}

impl ThorServiceImpl {
    pub fn new(dispatcher: Arc<crate::actions::ActionDispatcher>) -> Self {
        let xml_dispatcher = crate::actions::XmlDispatcher::new(dispatcher.clone());
        Self { dispatcher, xml_dispatcher }
    }
}

#[tonic::async_trait]
impl ThorService for ThorServiceImpl {
    async fn execute_action(
        &self,
        request: Request<thor::ThorAction>,
    ) -> Result<Response<thor::ThorResult>, Status> {
        let action = request.into_inner();
        
        let context = crate::actions::ActionContext {
            device_id: action.device_id.clone(),
            user_id: action.user_id.clone(),
            action_id: action.action_id.clone(),
        };

        // Dispatch action (check for structural XML protocol)
        let result = if action.action_type == "XML_CALL" || action.action_type == "XML_TASK" {
            let xml_str = String::from_utf8_lossy(&action.action_data);
            self.xml_dispatcher.execute_xml(&context, &xml_str).await
        } else {
            self.dispatcher
                .dispatch(&action.action_type, &context, &action.action_data)
                .await
        }.map_err(|e| Status::internal(format!("Action execution failed: {}", e)))?;

        let response = thor::ThorResult {
            action_id: action.action_id,
            success: true,
            error_message: String::new(),
            result_data: result,
            metadata: std::collections::HashMap::new(),
        };

        Ok(Response::new(response))
    }

    type ExecuteActionStreamStream = tokio_stream::wrappers::ReceiverStream<Result<thor::ThorActionStreamResponse, Status>>;

    async fn execute_action_stream(
        &self,
        _request: Request<tonic::Streaming<thor::ThorActionStreamRequest>>,
    ) -> Result<Response<Self::ExecuteActionStreamStream>, Status> {
        // TODO: Implement streaming action execution (for interactive operations)
        Err(Status::unimplemented("Not yet implemented"))
    }
}

pub struct GrpcServerDependencies {
    pub dispatcher: Arc<crate::actions::ActionDispatcher>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Thor gRPC server on {}", addr);

    let thor_service = ThorServiceImpl::new(deps.dispatcher);

    Server::builder()
        .add_service(ThorServiceServer::new(thor_service))
        .serve(addr)
        .await?;

    Ok(())
}
