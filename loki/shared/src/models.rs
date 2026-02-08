//! Shared models for Loki services

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLanguage {
    Lua,
    // Future: Python, JavaScript, etc.
}

impl ScriptLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Lua => "lua",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: u32,
    pub max_execution_time_ms: u32,
    pub max_cpu_percent: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 10,         // 10 MB default
            max_execution_time_ms: 5000, // 5 seconds default
            max_cpu_percent: 50,       // 50% CPU default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDefinition {
    pub name: String,
    pub language: ScriptLanguage,
    pub code: String,
    pub description: Option<String>,
    pub limits: ResourceLimits,
}

impl ScriptDefinition {
    pub fn new(name: String, language: ScriptLanguage, code: String) -> Self {
        Self {
            name,
            language,
            code,
            description: None,
            limits: ResourceLimits::default(),
        }
    }
    
    pub fn with_limits(mut self, limits: ResourceLimits) -> Self {
        self.limits = limits;
        self
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_script_language_as_str() {
        assert_eq!(ScriptLanguage::Lua.as_str(), "lua");
    }
    
    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_memory_mb, 10);
        assert_eq!(limits.max_execution_time_ms, 5000);
        assert_eq!(limits.max_cpu_percent, 50);
    }
    
    #[test]
    fn test_script_definition_new() {
        let script = ScriptDefinition::new(
            "test_script".to_string(),
            ScriptLanguage::Lua,
            "print('hello')".to_string(),
        );
        
        assert_eq!(script.name, "test_script");
        assert_eq!(script.language, ScriptLanguage::Lua);
        assert_eq!(script.code, "print('hello')");
        assert!(script.description.is_none());
    }
    
    #[test]
    fn test_script_definition_with_limits() {
        let limits = ResourceLimits {
            max_memory_mb: 20,
            max_execution_time_ms: 10000,
            max_cpu_percent: 80,
        };
        
        let script = ScriptDefinition::new(
            "test".to_string(),
            ScriptLanguage::Lua,
            "code".to_string(),
        ).with_limits(limits.clone());
        
        assert_eq!(script.limits.max_memory_mb, 20);
        assert_eq!(script.limits.max_execution_time_ms, 10000);
    }
}
