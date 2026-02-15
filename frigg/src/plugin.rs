use async_trait::async_trait;
use crate::plugin::OdinPlugin;

/// Frigg Plugin - Implements OdinPlugin trait for healthcare tasks
pub struct FriggPlugin {
    name: String,
    capabilities: Vec<String>,
}

impl FriggPlugin {
    pub fn new() -> Self {
        Self {
            name: "frigg".to_string(),
            capabilities: vec![
                "health_questions".to_string(),
                "mental_health".to_string(),
                "physical_health".to_string(),
                "certified_courses".to_string(),
            ],
        }
    }
}

#[async_trait]
impl OdinPlugin for FriggPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }

    async fn process_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Basic implementation - in a real version, this would:
        // 1. Parse the request to determine the healthcare task type
        // 2. Route to appropriate healthcare handler
        // 3. Process the task with healthcare-specific logic
        // 4. Return the result
        
        let request_lower = request.to_lowercase();
        
        if request_lower.contains("mental") || request_lower.contains("depression") || request_lower.contains("anxiety") {
            Ok("Mental health guidance provided".to_string())
        } else if request_lower.contains("physical") || request_lower.contains("pain") || request_lower.contains("symptom") {
            Ok("Physical health guidance provided".to_string())
        } else if request_lower.contains("course") || request_lower.contains("certified") {
            Ok("Certified course information provided".to_string())
        } else {
            Ok(format!("Frigg processed healthcare request: {}", request))
        }
    }
}

impl Default for FriggPlugin {
    fn default() -> Self {
        Self::new()
    }
}
