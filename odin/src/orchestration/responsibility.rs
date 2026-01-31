use std::sync::Arc;
use crate::protocols::einherjar::CapabilityCache;
use crate::protocols::manager::ProtocolManager;
use crate::orchestration::UserRequest;
use crate::orchestration::error::OrchestrationError;
use crate::clients::manager::ClientManager;

/// Determines which service/plugin handles a request and routes it (Einherjar + Responsibility protocol).
pub struct ResponsibilityManager {
    capability_cache: Arc<CapabilityCache>,
    protocol_manager: Arc<ProtocolManager>,
    client_manager: Arc<ClientManager>,
}

impl ResponsibilityManager {
    pub fn new(
        capability_cache: Arc<CapabilityCache>,
        protocol_manager: Arc<ProtocolManager>,
        client_manager: Arc<ClientManager>,
    ) -> Self {
        Self {
            capability_cache,
            protocol_manager,
            client_manager,
        }
    }

    /// Determine which service should handle the request
    /// Returns service name and relevance score
    pub async fn determine_responsibility(
        &self,
        request: &UserRequest,
    ) -> Result<Option<(String, f64)>, Box<dyn std::error::Error + Send + Sync>> {
        // Get all cached capabilities
        let all_capabilities = self.capability_cache.get_all().await;
        
        if all_capabilities.is_empty() {
            // No capabilities discovered yet - try to discover
            self.protocol_manager.discover_all_capabilities().await?;
            let all_capabilities = self.capability_cache.get_all().await;
            if all_capabilities.is_empty() {
                return Ok(None);
            }
        }

        // Score each service based on relevance
        let mut scored_services: Vec<(String, f64)> = Vec::new();
        
        for cached in &all_capabilities {
            let score = self.calculate_relevance_score(request, &cached.capability).await;
            if score > 0.0 {
                scored_services.push((cached.service_name.clone(), score));
            }
        }

        // Sort by score (highest first)
        scored_services.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return the highest scoring service
        Ok(scored_services.first().map(|(name, score)| (name.clone(), *score)))
    }

    /// Calculate relevance score for a service based on request
    pub async fn calculate_relevance_score(
        &self,
        request: &UserRequest,
        capability: &crate::protocols::einherjar::einherjar::CapabilityResponse,
    ) -> f64 {
        let mut score = 0.0;
        
        // Check responsibility domains
        for domain in &capability.responsibility_domains {
            if self.matches_domain(&request.input, domain) {
                score += 10.0;
            }
        }

        // Check responsibility keywords
        let input_lower = request.input.to_lowercase();
        for keyword in &capability.responsibility_keywords {
            if input_lower.contains(&keyword.to_lowercase()) {
                score += 5.0;
            }
        }

        // Check function keywords
        for function in &capability.functions {
            for keyword in &function.responsibility_keywords {
                if input_lower.contains(&keyword.to_lowercase()) {
                    score += 3.0;
                }
            }
        }

        // Check input type match
        // Some services might be better for specific input types
        match request.input_type.as_str() {
            "image" | "video" => {
                // Services with vision capabilities get bonus
                if capability.purpose.to_lowercase().contains("vision") {
                    score += 15.0;
                }
            }
            "audio" => {
                // Services with audio capabilities get bonus
                if capability.purpose.to_lowercase().contains("audio") || 
                   capability.purpose.to_lowercase().contains("speech") {
                    score += 15.0;
                }
            }
            _ => {}
        }

        score
    }

    /// Check if request matches a responsibility domain
    fn matches_domain(&self, input: &str, domain: &str) -> bool {
        let input_lower = input.to_lowercase();
        let domain_lower = domain.to_lowercase();
        
        // Simple keyword matching - can be enhanced with LLM
        input_lower.contains(&domain_lower) || 
        domain_lower.contains(&input_lower)
    }

    /// Route request to responsible service
    pub async fn route_request(
        &self,
        request: &UserRequest,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Determine responsible service
        let (service_name, score) = match self.determine_responsibility(request).await? {
            Some((name, score)) => (name, score),
            None => {
                // No service found - try to discover capabilities first
                self.protocol_manager.discover_all_capabilities().await?;
                
                // Try again after discovery
                match self.determine_responsibility(request).await? {
                    Some((name, score)) => (name, score),
                    None => {
                        return Err(Box::new(OrchestrationError::NoServiceFound));
                    }
                }
            }
        };

        tracing::info!("Routing request to {} (score: {})", service_name, score);

        // Try to take responsibility
        let take_request = crate::protocols::responsibility::responsibility::TakeResponsibilityRequest {
            request_id: request.request_id.clone(),
            user_id: request.user_id.clone(),
            device_id: request.device_id.clone(),
            input: request.input.clone(),
            input_type: request.input_type.clone(),
            reason: format!("Relevance score: {}", score),
        };

        match self.protocol_manager.take_responsibility(&service_name, take_request).await {
            Ok(response) => {
                if response.accepted {
                    tracing::info!("Service {} accepted responsibility", service_name);
                    // Route to service based on service type
                    self.execute_service_request(&service_name, request).await
                } else {
                    // Service rejected - try next best service from cache
                    tracing::warn!("Service {} rejected responsibility: {}", service_name, response.message);
                    self.try_fallback_service(request, &service_name).await
                }
            }
            Err(e) => {
                tracing::error!("Failed to take responsibility: {}", e);
                Err(e.into())
            }
        }
    }

    /// Select model via Skuld
    async fn select_model(&self, prompt: &str) -> String {
        let request = crate::clients::skuld::skuld::SelectModelRequest {
            prompt: prompt.to_string(),
            max_size: 0,
            min_reliability: 0.0,
            max_latency_ms: 0,
        };
        
        match self.client_manager.select_skuld_model(request).await {
            Ok(response) => response.model_name,
            Err(e) => {
                tracing::warn!("Failed to select model via Skuld: {}, using default", e);
                String::new() // Fallback to default model
            }
        }
    }

    /// Get RAG context from Freki
    async fn get_rag_context(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Create embedding request for Freki
        // Note: In real implementation, we'd need to generate embeddings first
        // For now, we'll use a placeholder
        let freki_request = crate::clients::freki::freki::RetrieveContextRequest {
            query_embedding: vec![], // TODO: Generate embedding from query
            limit: 5,
            collection_name: "default".to_string(),
        };
        
        match self.client_manager.retrieve_freki_context(freki_request).await {
            Ok(response) => {
                // Combine retrieved documents into context
                let context: Vec<String> = response.documents
                    .iter()
                    .map(|doc| doc.content.clone())
                    .collect();
                Ok(context.join("\n\n"))
            }
            Err(_) => {
                // Freki not available or no context found - return empty
                Ok(String::new())
            }
        }
    }

    /// Try fallback service if primary service rejected
    async fn try_fallback_service(
        &self,
        request: &UserRequest,
        rejected_service: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Get all capabilities and find next best match
        let all_capabilities = self.capability_cache.get_all().await;
        
        let mut scored_services: Vec<(String, f64)> = Vec::new();
        for cached in &all_capabilities {
            if cached.service_name != rejected_service {
                let score = self.calculate_relevance_score(request, &cached.capability).await;
                if score > 0.0 {
                    scored_services.push((cached.service_name.clone(), score));
                }
            }
        }
        
        scored_services.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        if let Some((service_name, score)) = scored_services.first() {
            tracing::info!("Trying fallback service {} (score: {})", service_name, score);
            
            // Try to take responsibility with fallback service
            let take_request = crate::protocols::responsibility::responsibility::TakeResponsibilityRequest {
                request_id: request.request_id.clone(),
                user_id: request.user_id.clone(),
                device_id: request.device_id.clone(),
                input: request.input.clone(),
                input_type: request.input_type.clone(),
                reason: format!("Fallback after {} rejected (score: {})", rejected_service, score),
            };
            
            match self.protocol_manager.take_responsibility(service_name, take_request).await {
                Ok(response) => {
                    if response.accepted {
                        tracing::info!("Fallback service {} accepted responsibility", service_name);
                        self.execute_service_request(service_name, request).await
                    } else {
                        Err(Box::new(OrchestrationError::ServiceRejected(
                            format!("Fallback service {} also rejected: {}", service_name, response.message),
                        )))
                    }
                }
                Err(e) => {
                    Err(format!("Failed to take responsibility with fallback service: {}", e).into())
                }
            }
        } else {
            Err(Box::new(OrchestrationError::NoFallbackService))
        }
    }

    /// Execute request on a specific service
    async fn execute_service_request(
        &self,
        service_name: &str,
        request: &UserRequest,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match service_name {
            "geri" => {
                // Route to Geri for LLM processing
                // First, try to get context from Freki (RAG)
                let context = self.get_rag_context(&request.input).await.unwrap_or_default();
                
                // Use Skuld for model selection
                let model_name = self.select_model(&request.input).await.unwrap_or_default();
                
                let geri_request = crate::clients::geri::geri::ProcessPromptRequest {
                    prompt: request.input.clone(),
                    context,
                    model_name,
                    max_tokens: 1000,
                };
                
                match self.client_manager.process_geri_prompt(geri_request).await {
                    Ok(response) => Ok(response.text),
                    Err(e) => Err(Box::new(OrchestrationError::ActionFailed(format!("Geri: {}", e)))),
                }
            }
            "freki" => {
                // Route to Freki for RAG
                // This would typically be used in combination with Geri
                Err(Box::new(OrchestrationError::ServiceNotImplemented(
                    "Freki is typically used with Geri, not standalone".to_string(),
                )))
            }
            "thor" => {
                // Route to Thor for action execution via ActionOrchestrator
                // Create a simple action plan from the request
                let action_orchestrator = crate::orchestration::ActionOrchestrator::new_with_client(
                    self.client_manager.clone()
                );
                
                let action_plan = action_orchestrator.plan_actions(&request.input).await
                    .map_err(|e| Box::new(OrchestrationError::ActionFailed(format!("plan: {}", e))) as Box<dyn std::error::Error + Send + Sync>)?;
                
                let results = action_orchestrator.execute_actions(action_plan).await
                    .map_err(|e| Box::new(OrchestrationError::ActionFailed(format!("execute: {}", e))) as Box<dyn std::error::Error + Send + Sync>)?;
                
                // Combine results into response
                Ok(results.join("\n"))
            }
            _ => {
                Err(Box::new(OrchestrationError::ServiceNotImplemented(
                    service_name.to_string(),
                )))
            }
        }
    }
}
