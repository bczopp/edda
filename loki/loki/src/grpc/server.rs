use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, warn, error};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod loki {
    tonic::include_proto!("loki");
}

use loki::loki_service_server::{LokiService, LokiServiceServer};
use crate::coordination::ServiceCoordinator;
use crate::error_handler::ErrorHandler;
use crate::tools::config_loader::ToolConfigLoader;
use crate::tools::config::{ToolConfig, ToolDefinition};
use crate::script_registry::ScriptRegistry;
use crate::script::manager::ScriptManager;

pub struct LokiServiceImpl {
    coordinator: Arc<ServiceCoordinator>,
    tool_config: Arc<RwLock<ToolConfigLoader>>,
    pub(crate) script_registry: Arc<ScriptRegistry>,
    script_manager: Arc<ScriptManager>,
}

impl LokiServiceImpl {
    pub fn new(
        coordinator: Arc<ServiceCoordinator>,
        tool_config: Arc<RwLock<ToolConfigLoader>>,
        script_registry: Arc<ScriptRegistry>,
        script_manager: Arc<ScriptManager>,
    ) -> Self {
        Self {
            coordinator,
            tool_config,
            script_registry,
            script_manager,
        }
    }
}

#[tonic::async_trait]
impl LokiService for LokiServiceImpl {
    async fn get_capabilities(
        &self,
        _request: Request<loki::GetCapabilitiesRequest>,
    ) -> Result<Response<loki::GetCapabilitiesResponse>, Status> {
        info!("GetCapabilities called");
        
        let config = self.tool_config.read().await.get_config().await;
        
        let capabilities: Vec<loki::ScriptCapability> = config.tools.iter().map(|tool| {
            loki::ScriptCapability {
                script_name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.iter().map(|p| {
                    loki::ParameterDefinition {
                        name: p.name.clone(),
                        r#type: format!("{:?}", p.param_type),
                        required: p.required,
                        description: p.description.clone().unwrap_or_default(),
                    }
                }).collect(),
                return_type: format!("{:?}", tool.return_type),
                supports_streaming: false, // TODO: Implement streaming support
            }
        }).collect();
        
        let children = vec![
            loki::ChildServiceInfo {
                name: "Fenrir".to_string(),
                purpose: "Hardware-Control Service".to_string(),
                enabled: false, // TODO: Check actual status
                address: "127.0.0.1:50071".to_string(),
            },
            loki::ChildServiceInfo {
                name: "Jörmungandr".to_string(),
                purpose: "Network/Communication Service".to_string(),
                enabled: false, // TODO: Check actual status
                address: "127.0.0.1:50072".to_string(),
            },
            loki::ChildServiceInfo {
                name: "Hel".to_string(),
                purpose: "Data/Storage Service".to_string(),
                enabled: false, // TODO: Check actual status
                address: "127.0.0.1:50073".to_string(),
            },
        ];
        
        Ok(Response::new(loki::GetCapabilitiesResponse {
            service_name: "Loki".to_string(),
            purpose: "Script Execution Service".to_string(),
            capabilities,
            children,
        }))
    }
    
    async fn get_children_status(
        &self,
        _request: Request<loki::GetChildrenStatusRequest>,
    ) -> Result<Response<loki::GetChildrenStatusResponse>, Status> {
        info!("GetChildrenStatus called");
        
        // TODO: Check actual status of child services
        let children = vec![
            loki::ChildStatus {
                name: "Fenrir".to_string(),
                available: false,
                status_message: "Not connected".to_string(),
                last_check_timestamp: chrono::Utc::now().timestamp(),
            },
            loki::ChildStatus {
                name: "Jörmungandr".to_string(),
                available: false,
                status_message: "Not connected".to_string(),
                last_check_timestamp: chrono::Utc::now().timestamp(),
            },
            loki::ChildStatus {
                name: "Hel".to_string(),
                available: false,
                status_message: "Not connected".to_string(),
                last_check_timestamp: chrono::Utc::now().timestamp(),
            },
        ];
        
        Ok(Response::new(loki::GetChildrenStatusResponse {
            children,
        }))
    }
    
    async fn list_scripts(
        &self,
        request: Request<loki::ListScriptsRequest>,
    ) -> Result<Response<loki::ListScriptsResponse>, Status> {
        let req = request.into_inner();
        info!("ListScripts called with pattern: {:?}", req.name_pattern);
        
        // Get all script names from registry
        let script_names = self.script_registry.list_scripts().await;
        
        let scripts: Vec<loki::ScriptInfo> = script_names.iter()
            .filter_map(|name| {
                // Get tool from registry
                if let Some(tool) = self.script_registry.get_tool(name).await {
                    if let Some(pattern) = &req.name_pattern {
                        if !tool.name.contains(pattern) {
                            return None;
                        }
                    }
                    Some(tool)
                } else {
                    None
                }
            })
            .map(|tool| {
                let (has_inline_script, script_path) = match &tool.script {
                    crate::tools::config::ScriptSource { inline: Some(_), path: None } => (true, String::new()),
                    crate::tools::config::ScriptSource { inline: None, path: Some(path) } => (false, path.clone()),
                    _ => (false, String::new()),
                };
                
                loki::ScriptInfo {
                    name: tool.name.clone(),
                    description: tool.description.clone(),
                    language: "lua".to_string(), // TODO: Get from tool definition
                    parameters: tool.parameters.iter().map(|p| {
                        loki::ParameterDefinition {
                            name: p.name.clone(),
                            r#type: format!("{:?}", p.param_type),
                            required: p.required,
                            description: p.description.clone().unwrap_or_default(),
                        }
                    }).collect(),
                    return_type: format!("{:?}", tool.return_type),
                    has_inline_script,
                    script_path,
                }
            })
            .collect();
        
        Ok(Response::new(loki::ListScriptsResponse {
            scripts,
        }))
    }
    
    async fn register_script(
        &self,
        request: Request<loki::RegisterScriptRequest>,
    ) -> Result<Response<loki::RegisterScriptResponse>, Status> {
        let req = request.into_inner();
        info!("RegisterScript called for: {}", req.name);
        
        // Check if script already exists
        if self.script_registry.has_tool(&req.name).await {
            return Ok(Response::new(loki::RegisterScriptResponse {
                success: false,
                script_name: req.name.clone(),
                error_message: format!("Script '{}' already exists", req.name),
            }));
        }
        
        // Convert protobuf parameters to ToolParameter
        use crate::tools::config::{ToolDefinition, ToolParameter, ParameterType, ReturnType, ScriptSource};
        let parameters: Result<Vec<ToolParameter>, Status> = req.parameters.into_iter()
            .map(|p| {
                let param_type = match p.r#type.as_str() {
                    "String" => ParameterType::String,
                    "Number" | "Integer" | "Float" => ParameterType::Number,
                    "Boolean" => ParameterType::Boolean,
                    "Object" => ParameterType::Object,
                    "Array" => ParameterType::Array,
                    _ => return Err(Status::invalid_argument(format!("Invalid parameter type: {}", p.r#type))),
                };
                Ok(ToolParameter {
                    name: p.name,
                    param_type,
                    required: p.required,
                    description: if p.description.is_empty() { None } else { Some(p.description) },
                })
            })
            .collect();
        let parameters = parameters?;
        
        // Parse return type
        let return_type = match req.return_type.as_str() {
            "String" => ReturnType::String,
            "Number" | "Integer" | "Float" => ReturnType::Number,
            "Boolean" => ReturnType::Boolean,
            "Object" => ReturnType::Object,
            "Array" => ReturnType::Array,
            "Void" => ReturnType::Void,
            _ => return Err(Status::invalid_argument(format!("Invalid return type: {}", req.return_type))),
        };
        
        // Determine script source (protobuf oneof)
        let script_source = if !req.inline_script.is_empty() {
            if !req.script_path.is_empty() {
                return Err(Status::invalid_argument("Cannot specify both inline_script and script_path"));
            }
            ScriptSource::inline(req.inline_script)
        } else if !req.script_path.is_empty() {
            ScriptSource::path(req.script_path)
        } else {
            return Err(Status::invalid_argument("Must specify either inline_script or script_path"));
        };
        
        // Create tool definition
        let tool = ToolDefinition {
            name: req.name.clone(),
            description: req.description.clone(),
            parameters,
            return_type,
            script: script_source,
        };
        
        // Register tool in registry first (needed for validation)
        self.script_registry.register_tool(tool.clone()).await;
        
        // Validate script syntax
        if let Err(e) = self.script_manager.validate_script(&req.name).await {
            // Remove from registry if validation fails
            self.script_registry.unregister_tool(&req.name).await;
            return Ok(Response::new(loki::RegisterScriptResponse {
                success: false,
                script_name: req.name.clone(),
                error_message: format!("Script validation failed: {}", e),
            }));
        }

        // Invalidate cache so next execution loads the (possibly updated) script
        self.script_manager.invalidate_script(&req.name).await;

        info!("Successfully registered script: {}", req.name);
        
        Ok(Response::new(loki::RegisterScriptResponse {
            success: true,
            script_name: req.name,
            error_message: String::new(),
        }))
    }
    
    async fn execute_script(
        &self,
        request: Request<loki::ExecuteScriptRequest>,
    ) -> Result<Response<loki::ExecuteScriptResponse>, Status> {
        let req = request.into_inner();
        info!("ExecuteScript called for script_id: {}", req.script_id);
        
        // Execute script via coordinator
        let result = self.coordinator.execute_script(&req.script_content).await
            .map_err(|e| ErrorHandler::to_grpc_status(&e))?;

        Ok(Response::new(loki::ExecuteScriptResponse {
            success: true,
            output: result,
            error: String::new(),
        }))
    }
    
    type StreamScriptExecutionStream = tokio_stream::wrappers::ReceiverStream<Result<loki::ScriptChunk, Status>>;
    
    async fn stream_script_execution(
        &self,
        request: Request<loki::StreamScriptExecutionRequest>,
    ) -> Result<Response<Self::StreamScriptExecutionStream>, Status> {
        let req = request.into_inner();
        info!("StreamScriptExecution called for script: {}", req.script_name);
        
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        
        // Get script from registry
        let tool = match self.script_registry.get_tool(&req.script_name).await {
            Some(tool) => tool,
            None => {
                let _ = tx.send(Ok(loki::ScriptChunk {
                    chunk_type: Some(loki::script_chunk::ChunkType::Error(
                        loki::ScriptError {
                            error_message: format!("Script '{}' not found", req.script_name),
                            error_type: "script_not_found".to_string(),
                        }
                    )),
                    timestamp: chrono::Utc::now().timestamp(),
                })).await;
                return Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)));
            }
        };
        
        // Build script context from parameters
        use crate::script::engine::ScriptContext;
        let mut context = ScriptContext::new();
        if let Some(input) = req.input {
            for (key, value) in input.parameters {
                context.set(&key, serde_json::Value::String(value));
            }
        }
        
        // Execute script in background task and stream results
        let script_manager = Arc::clone(&self.script_manager);
        let script_name = req.script_name.clone();
        tokio::spawn(async move {
            // Send initial output chunk
            let _ = tx.send(Ok(loki::ScriptChunk {
                chunk_type: Some(loki::script_chunk::ChunkType::Output(
                    loki::ScriptOutput {
                        data: format!("Executing script: {}\n", script_name),
                        is_final: false,
                    }
                )),
                timestamp: chrono::Utc::now().timestamp(),
            })).await;
            
            // Execute script
            match script_manager.execute_script(&script_name, context).await {
                Ok(output) => {
                    // Send output chunks (split into lines for streaming)
                    let lines: Vec<&str> = output.lines().collect();
                    for (i, line) in lines.iter().enumerate() {
                        let is_final = i == lines.len() - 1;
                        let _ = tx.send(Ok(loki::ScriptChunk {
                            chunk_type: Some(loki::script_chunk::ChunkType::Output(
                                loki::ScriptOutput {
                                    data: format!("{}\n", line),
                                    is_final,
                                }
                            )),
                            timestamp: chrono::Utc::now().timestamp(),
                        })).await;
                    }
                    
                    // Send final result chunk
                    let _ = tx.send(Ok(loki::ScriptChunk {
                        chunk_type: Some(loki::script_chunk::ChunkType::Result(
                            loki::ScriptResult {
                                success: true,
                                output,
                                metadata: std::collections::HashMap::new(),
                            }
                        )),
                        timestamp: chrono::Utc::now().timestamp(),
                    })).await;
                }
                Err(e) => {
                    // Send error chunk
                    let _ = tx.send(Ok(loki::ScriptChunk {
                        chunk_type: Some(loki::script_chunk::ChunkType::Error(
                            loki::ScriptError {
                                error_message: e.to_string(),
                                error_type: "execution_error".to_string(),
                            }
                        )),
                        timestamp: chrono::Utc::now().timestamp(),
                    })).await;
                }
            }
        });
        
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}

pub struct GrpcServerDependencies {
    pub coordinator: Arc<ServiceCoordinator>,
    pub tool_config: Arc<RwLock<ToolConfigLoader>>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Loki gRPC server on {}", addr);

    let loki_service = LokiServiceImpl::new(
        deps.coordinator,
        deps.tool_config,
        deps.script_registry,
        deps.script_manager,
    );

    Server::builder()
        .add_service(LokiServiceServer::new(loki_service))
        .serve(addr)
        .await?;

    Ok(())
}
