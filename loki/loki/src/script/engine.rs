//! Script Engine implementation

use shared::{LokiError, Result, ScriptDefinition, ScriptLanguage, ResourceLimits};
use crate::tools::config::{ToolDefinition, ScriptSource};
use mlua::{Lua, Value};
use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;
use tracing::{info, warn, error};

pub struct ScriptContext {
    data: HashMap<String, serde_json::Value>,
}

impl ScriptContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        self.data.insert(key.to_string(), value);
    }
    
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

impl Default for ScriptContext {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ScriptEngine {
    // Future: Add resource monitoring, execution tracking
}

impl ScriptEngine {
    pub fn new() -> Self {
        info!("Creating ScriptEngine");
        Self {}
    }
    
    pub fn is_ready(&self) -> bool {
        true
    }
    
    pub async fn execute(
        &self,
        script: &ScriptDefinition,
        context: ScriptContext,
    ) -> Result<String> {
        self.execute_with_lua_setup(script, context, None).await
    }

    /// Execute script with optional Lua API registration (e.g. fenrir, jormungandr, hel).
    pub async fn execute_with_lua_setup<F>(&self, script: &ScriptDefinition, context: ScriptContext, lua_setup: Option<F>) -> Result<String>
    where
        F: FnOnce(&Lua) -> Result<()> + Send,
    {
        match script.language {
            ScriptLanguage::Lua => self.execute_lua(script, context, lua_setup).await,
        }
    }
    
    async fn execute_lua<F>(
        &self,
        script: &ScriptDefinition,
        context: ScriptContext,
        lua_setup: Option<F>,
    ) -> Result<String>
    where
        F: FnOnce(&Lua) -> Result<()> + Send,
    {
        info!("Executing Lua script: {}", script.name);
        
        let lua = Lua::new();

        if let Some(setup) = lua_setup {
            setup(&lua).map_err(|e| LokiError::ExecutionError(e.to_string()))?;
        }
        
        // Set context data as global variables
        if let Some(data) = context.get("data") {
            if let Ok(value_str) = serde_json::to_string(data) {
                // Load context data as JSON into Lua
                let load_code = format!("data = {}", value_str);
                if let Err(e) = lua.load(&load_code).exec() {
                    warn!("Failed to load context data: {}", e);
                }
            }
        }
        
        // Execute script with timeout
        let timeout = Duration::from_millis(script.limits.max_execution_time_ms as u64);
        
        // Spawn execution in tokio task with timeout
        let code = script.code.clone();
        let name = script.name.clone();
        
        let result = tokio::time::timeout(timeout, tokio::task::spawn_blocking(move || {
            match lua.load(&code).eval::<Value>() {
                Ok(value) => {
                    let result_str = match value {
                        Value::String(s) => s.to_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|_| "[binary string]".to_string()),
                        Value::Integer(i) => i.to_string(),
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => b.to_string(),
                        Value::Nil => "nil".to_string(),
                        _ => format!("{:?}", value),
                    };
                    Ok(result_str)
                }
                Err(e) => {
                    error!("Lua execution error in {}: {}", name, e);
                    Err(LokiError::ExecutionError(e.to_string()))
                }
            }
        }))
        .await;
        
        match result {
            Ok(Ok(execution_result)) => execution_result,
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                error!("Script execution timeout: {}", script.name);
                Err(LokiError::ResourceLimitExceeded(
                    format!("Execution timeout after {}ms", script.limits.max_execution_time_ms)
                ))
            }
        }
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Trait-like interface for ScriptEngine
impl ScriptEngine {
    /// Execute a script from a ToolDefinition
    pub async fn execute_script(
        &self,
        tool: &ToolDefinition,
        context: ScriptContext,
    ) -> Result<String> {
        let script_def = self.load_script(tool).await?;
        self.execute(&script_def, context).await
    }
    
    /// Load a script from a ToolDefinition (inline or path)
    pub async fn load_script(&self, tool: &ToolDefinition) -> Result<ScriptDefinition> {
        let code = match &tool.script {
            ScriptSource { inline: Some(code), path: None } => code.clone(),
            ScriptSource { inline: None, path: Some(path) } => {
                std::fs::read_to_string(path)
                    .map_err(|e| LokiError::IoError(format!("Failed to read script file {}: {}", path, e)))?
            }
            _ => return Err(LokiError::InvalidScript(
                format!("Tool '{}' must have either inline script or path", tool.name)
            )),
        };
        
        let language = match tool.name.as_str() {
            _ if code.contains("function") || code.contains("return") => ScriptLanguage::Lua,
            _ => ScriptLanguage::Lua, // Default to Lua for now
        };
        
        Ok(ScriptDefinition {
            name: tool.name.clone(),
            language,
            code,
            description: Some(tool.description.clone()),
            limits: ResourceLimits::default(), // TODO: Get from tool config
        })
    }
    
    /// Validate a script from a ToolDefinition
    pub async fn validate_script(&self, tool: &ToolDefinition) -> Result<()> {
        let script_def = self.load_script(tool).await?;
        
        // Try to compile/parse the script
        match script_def.language {
            ScriptLanguage::Lua => {
                let lua = Lua::new();
                lua.load(&script_def.code)
                    .exec()
                    .map_err(|e| LokiError::CompilationError(format!("Lua syntax error: {}", e)))?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::ResourceLimits;
    
    #[tokio::test]
    async fn test_execute_simple_lua() {
        let engine = ScriptEngine::new();
        let script = ScriptDefinition::new(
            "test".to_string(),
            ScriptLanguage::Lua,
            "return 42".to_string(),
        );
        
        let context = ScriptContext::new();
        let result = engine.execute(&script, context).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }
    
    #[tokio::test]
    async fn test_execute_invalid_lua() {
        let engine = ScriptEngine::new();
        let script = ScriptDefinition::new(
            "invalid".to_string(),
            ScriptLanguage::Lua,
            "invalid syntax".to_string(),
        );
        
        let context = ScriptContext::new();
        let result = engine.execute(&script, context).await;
        
        assert!(result.is_err());
    }
}
