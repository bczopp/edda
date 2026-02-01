use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, info_span};
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

pub mod freki {
    tonic::include_proto!("freki");
}

use freki::freki_service_server::{FrekiService, FrekiServiceServer};

pub struct FrekiServiceImpl {
    vector_db: Arc<crate::vector_db::VectorDbClient>,
    document_indexer: Arc<crate::indexing::DocumentIndexer>,
    context_retriever: Arc<crate::retrieval::ContextRetriever>,
    collection_name: String,
    audit_logger: Arc<crate::utils::AuditLogger>,
}

impl FrekiServiceImpl {
    pub fn new(
        vector_db: Arc<crate::vector_db::VectorDbClient>,
        collection_name: String,
        audit_logger: Arc<crate::utils::AuditLogger>,
    ) -> Self {
        let document_indexer = Arc::new(crate::indexing::DocumentIndexer::new(
            (*vector_db).clone(),
            collection_name.clone(),
        ));
        let context_retriever = Arc::new(crate::retrieval::ContextRetriever::new(
            (*vector_db).clone(),
            collection_name.clone(),
        ));
        
        Self {
            vector_db,
            document_indexer,
            context_retriever,
            collection_name,
            audit_logger,
        }
    }
}

#[tonic::async_trait]
impl FrekiService for FrekiServiceImpl {
    async fn index_document(
        &self,
        request: Request<freki::IndexDocumentRequest>,
    ) -> Result<Response<freki::IndexDocumentResponse>, Status> {
        let request_id = Uuid::new_v4().to_string();
        let _guard = info_span!("index_document", request_id = %request_id).entered();
        let req = request.into_inner();

        // Parse embedding from bytes
        let embedding: Vec<f32> = bincode::deserialize(&req.embedding)
            .map_err(|e| Status::invalid_argument(format!("Invalid embedding format: {}", e)))?;

        // Create document
        let document = crate::indexing::Document {
            id: req.document_id.clone(),
            content: req.content,
            metadata: serde_json::to_value(req.metadata).unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
        };

        // Index document
        self.document_indexer.index_document(document, embedding).await
            .map_err(|e| Status::internal(format!("Failed to index document: {}", e)))?;
        self.audit_logger.log_document_indexed(&req.document_id);

        Ok(Response::new(freki::IndexDocumentResponse {
            success: true,
            message: format!("Document {} indexed successfully", req.document_id),
        }))
    }

    async fn retrieve_context(
        &self,
        request: Request<freki::RetrieveContextRequest>,
    ) -> Result<Response<freki::RetrieveContextResponse>, Status> {
        let request_id = Uuid::new_v4().to_string();
        let _guard = info_span!("retrieve_context", request_id = %request_id).entered();
        let req = request.into_inner();
        self.audit_logger.log_query(&request_id, req.limit as u32);

        // Parse query embedding from bytes
        let query_embedding: Vec<f32> = bincode::deserialize(&req.query_embedding)
            .map_err(|e| Status::invalid_argument(format!("Invalid query embedding format: {}", e)))?;

        // Use collection from request or default
        let collection = if req.collection_name.is_empty() {
            &self.collection_name
        } else {
            &req.collection_name
        };

        // Retrieve context
        let context = self.context_retriever.retrieve(query_embedding, req.limit).await
            .map_err(|e| Status::internal(format!("Failed to retrieve context: {}", e)))?;

        // Convert to protobuf response and audit document access
        let documents: Vec<freki::RetrievedDocument> = context.documents.into_iter().map(|doc| {
            self.audit_logger.log_document_accessed(&doc.id);
            freki::RetrievedDocument {
                id: doc.id,
                content: doc.content,
                metadata: doc.metadata.as_object()
                    .map(|m| m.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect())
                    .unwrap_or_default(),
                score: doc.score,
            }
        }).collect();

        Ok(Response::new(freki::RetrieveContextResponse {
            documents,
            relevance_scores: context.relevance_scores,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub vector_db: Arc<crate::vector_db::VectorDbClient>,
    pub collection_name: String,
    pub audit_logger: Arc<crate::utils::AuditLogger>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Freki gRPC server on {}", addr);

    let freki_service = FrekiServiceImpl::new(
        deps.vector_db,
        deps.collection_name,
        deps.audit_logger,
    );

    Server::builder()
        .add_service(FrekiServiceServer::new(freki_service))
        .serve(addr)
        .await?;

    Ok(())
}
