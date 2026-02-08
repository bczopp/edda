//! Tool configuration system for Loki

use serde::{Deserialize, Serialize};
use crate::utils::{LokiError, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolConfig {
    pub tools: Vec<ToolDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
    #[serde(rename = "return_type")]
    pub return_type: ReturnType,
    pub script: ScriptSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: ParameterType,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReturnType {
    String,
    Number,
    Boolean,
    Object,
    Array,
    Void,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl ScriptSource {
    pub fn inline(code: String) -> Self {
        Self {
            inline: Some(code),
            path: None,
        }
    }
    
    pub fn path(path: String) -> Self {
        Self {
            inline: None,
            path: Some(path),
        }
    }
    
    pub fn is_inline(&self) -> bool {
        self.inline.is_some()
    }
    
    pub fn is_path(&self) -> bool {
        self.path.is_some()
    }
    
    pub fn get_inline(&self) -> Option<&String> {
        self.inline.as_ref()
    }
    
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

impl ToolDefinition {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            parameters: Vec::new(),
            return_type: ReturnType::String,
            script: ScriptSource::Inline(String::new()),
        }
    }
}

impl ToolParameter {
    pub fn new(name: String, param_type: ParameterType, required: bool) -> Self {
        Self {
            name,
            param_type,
            required,
            description: None,
        }
    }
}

impl ToolConfig {
    pub fn validate(&self) -> Result<()> {
        for tool in &self.tools {
            if tool.name.is_empty() {
                return Err(LokiError::ConfigurationError(
                    "Tool name cannot be empty".to_string()
                ));
            }
            
            // Validate script source: must have either inline or path, not both
            let has_inline = tool.script.inline.is_some();
            let has_path = tool.script.path.is_some();
            
            if !has_inline && !has_path {
                return Err(LokiError::ConfigurationError(
                    format!("Tool '{}' must have either inline script or path", tool.name)
                ));
            }
            
            if has_inline && has_path {
                return Err(LokiError::ConfigurationError(
                    format!("Tool '{}' cannot have both inline script and path", tool.name)
                ));
            }
            
            if let Some(code) = &tool.script.inline {
                if code.is_empty() {
                    return Err(LokiError::ConfigurationError(
                        format!("Tool '{}' has empty inline script", tool.name)
                    ));
                }
            }
            
            if let Some(path) = &tool.script.path {
                if path.is_empty() {
                    return Err(LokiError::ConfigurationError(
                        format!("Tool '{}' has empty script path", tool.name)
                    ));
                }
            }
            
            // Validate parameters
            for param in &tool.parameters {
                if param.name.is_empty() {
                    return Err(LokiError::ConfigurationError(
                        format!("Tool '{}' has parameter with empty name", tool.name)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    pub fn from_toml(toml: &str) -> Result<Self> {
        let config: ToolConfig = toml::from_str(toml)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to parse TOML: {}", e)))?;
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self)
            .map_err(|e| LokiError::ConfigurationError(format!("Failed to serialize TOML: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parameter_type_serialization() {
        let param = ToolParameter::new("test".to_string(), ParameterType::String, true);
        assert_eq!(param.param_type, ParameterType::String);
    }
    
    #[test]
    fn test_return_type_default() {
        let tool = ToolDefinition::new("test".to_string(), "desc".to_string());
        assert_eq!(tool.return_type, ReturnType::String);
    }
}
