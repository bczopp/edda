use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::audit::{AuditEvent, AuditLogger};
use super::responsibility;
use super::ActionOrchestrator;
use crate::utils::{MonitoringService, ParallelProcessor, QueuedRequest, RequestQueue, ResponseCache};

/// User input as received by the orchestrator (from platform or Huginn).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
    pub request_id: String,
    pub user_id: String,
    pub device_id: String,
    pub input: String,
    /// Input kind: `"text"`, `"audio"`, `"image"`, `"video"`.
    pub input_type: String,
}

impl From<QueuedRequest> for UserRequest {
    fn from(q: QueuedRequest) -> Self {
        Self {
            request_id: q.request_id,
            user_id: q.user_id,
            device_id: q.device_id,
            input: q.input,
            input_type: q.input_type,
        }
    }
}

/// Processes user requests: routes via ResponsibilityManager or fallback (type/keyword).
pub struct RequestProcessor {
    responsibility_manager: Option<Arc<responsibility::ResponsibilityManager>>,
    action_orchestrator: Option<Arc<ActionOrchestrator>>,
    audit_logger: Option<Arc<dyn AuditLogger>>,
    monitoring: Option<Arc<MonitoringService>>,
    response_cache: Option<Arc<ResponseCache>>,
}

impl RequestProcessor {
    /// Processor with no routing; returns placeholder messages.
    pub fn new() -> Self {
        Self {
            responsibility_manager: None,
            action_orchestrator: None,
            audit_logger: None,
            monitoring: None,
            response_cache: None,
        }
    }

    /// Full flow: Parse (determine_responsibility) → Route (route_request) → Coordinate (execute_service_request).
    pub fn new_with_responsibility(responsibility_manager: Arc<responsibility::ResponsibilityManager>) -> Self {
        Self {
            responsibility_manager: Some(responsibility_manager),
            action_orchestrator: None,
            audit_logger: None,
            monitoring: None,
            response_cache: None,
        }
    }

    /// Fallback path: when no ResponsibilityManager, use ActionOrchestrator for action-like text.
    pub fn new_with_action_fallback(action_orchestrator: ActionOrchestrator) -> Self {
        Self {
            responsibility_manager: None,
            action_orchestrator: Some(Arc::new(action_orchestrator)),
            audit_logger: None,
            monitoring: None,
            response_cache: None,
        }
    }

    /// Attach an audit logger; call after construction (e.g. `processor.with_audit_logger(logger)`).
    pub fn with_audit_logger(mut self, logger: Arc<dyn AuditLogger>) -> Self {
        self.audit_logger = Some(logger);
        self
    }

    /// Attach monitoring; process() updates active_requests (1 during, 0 after).
    pub fn with_monitoring(mut self, monitoring: Arc<MonitoringService>) -> Self {
        self.monitoring = Some(monitoring);
        self
    }

    /// Phase 8 Caching: use ResponseCache by request_id; cache hit returns stored response.
    pub fn with_response_cache(mut self, cache: Arc<ResponseCache>) -> Self {
        self.response_cache = Some(cache);
        self
    }

    /// Process a user request; returns response string or error (e.g. [`OrchestrationError`](crate::orchestration::OrchestrationError)).
    pub async fn process(&self, request: UserRequest) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref m) = self.monitoring {
            m.update_active_requests(1).await;
        }
        if let Some(ref c) = self.response_cache {
            if let Some(cached) = c.get(&request.request_id).await {
                if let Some(ref m) = self.monitoring {
                    m.update_active_requests(0).await;
                }
                return Ok(cached);
            }
        }
        let result = self.process_inner(request.clone()).await;
        if let (Some(ref c), Ok(ref s)) = (self.response_cache.as_ref(), &result) {
            c.set(request.request_id.clone(), s.clone()).await;
        }
        if let Some(ref m) = self.monitoring {
            m.update_active_requests(0).await;
        }
        result
    }

    /// Phase 8 Request-Queuing: dequeue one request, process it, return result or None if queue empty.
    pub async fn process_one_from_queue(
        self: Arc<Self>,
        queue: Arc<RequestQueue>,
    ) -> Option<Result<String, Box<dyn std::error::Error + Send + Sync>>> {
        let q = queue.dequeue().await?;
        Some(self.process(UserRequest::from(q)).await)
    }

    /// Phase 8 Parallel-Processing: process multiple requests concurrently via utils::ParallelProcessor.
    pub async fn process_parallel(
        self: Arc<Self>,
        requests: Vec<QueuedRequest>,
    ) -> Vec<Result<String, Box<dyn std::error::Error + Send + Sync>>> {
        ParallelProcessor::process_parallel(requests, move |q| {
            let me = Arc::clone(&self);
            Box::pin(async move { me.process(UserRequest::from(q)).await })
        })
        .await
    }

    async fn process_inner(&self, request: UserRequest) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref log) = self.audit_logger {
            log.log(&AuditEvent::RequestReceived {
                request_id: request.request_id.clone(),
                user_id: request.user_id.clone(),
                device_id: request.device_id.clone(),
                input_type: request.input_type.clone(),
            });
        }
        if let Some(ref resp_manager) = self.responsibility_manager {
            return resp_manager.route_request(&request).await;
        }

        let input_lower = request.input.to_lowercase();
        if request.input_type == "audio" || input_lower.contains("transcribe") || input_lower.contains("speech") {
            return Ok("Audio processing requires Huginn-Muninn service (not available without responsibility manager)".to_string());
        }
        if request.input_type == "image" || request.input_type == "video" {
            return Ok("Vision processing requires Geri service (not available without responsibility manager)".to_string());
        }
        if request.input_type == "text" {
            if let Some(ref ao) = self.action_orchestrator {
                if let Ok(plan) = ao.plan_actions(&request.input).await {
                    if !plan.actions.is_empty() {
                        let first_type = &plan.actions[0].action_type;
                        return Ok(format!("Planned {} action(s): {}", plan.actions.len(), first_type));
                    }
                }
            }
            return Ok("Text processing requires Geri service (not available without responsibility manager)".to_string());
        }
        Ok(format!("Request received: {} (type: {}). Responsibility manager required for full processing.", request.input, request.input_type))
    }
}
