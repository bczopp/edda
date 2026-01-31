use mlua::Lua;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Script execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Script parsing failed: {0}")]
    ParseFailed(String),
}

pub struct ScriptEngine {
    lua: Lua,
}

impl ScriptEngine {
    pub fn new() -> Result<Self, ScriptError> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    pub fn execute(&self, script: &str) -> Result<mlua::Value, ScriptError> {
        self.lua.load(script).eval()
            .map_err(|e| ScriptError::ExecutionFailed(format!("{}", e)))
    }
}
