//! Tests for Loki tool configuration system

use loki::tools::config::{ToolConfig, ToolDefinition, ToolParameter, ParameterType, ReturnType, ScriptSource};
use loki::utils::{LokiError, Result};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_tool_config_default() {
    let config = ToolConfig::default();
    assert!(config.tools.is_empty());
}

#[test]
fn test_tool_definition_new() {
    let tool = ToolDefinition::new(
        "test_tool".to_string(),
        "A test tool".to_string(),
    );
    
    assert_eq!(tool.name, "test_tool");
    assert_eq!(tool.description, "A test tool");
    assert!(tool.parameters.is_empty());
    assert_eq!(tool.return_type, ReturnType::String);
}

#[test]
fn test_tool_parameter_new() {
    let param = ToolParameter::new(
        "input".to_string(),
        ParameterType::String,
        true,
    );
    
    assert_eq!(param.name, "input");
    assert_eq!(param.param_type, ParameterType::String);
    assert!(param.required);
    assert!(param.description.is_none());
}

#[test]
fn test_tool_config_from_toml() {
    let toml = r#"
[[tools]]
name = "hello_world"
description = "A simple hello world script"
return_type = "String"

[tools.script]
inline = "print('Hello, World!')"
"#;
    
    let config = ToolConfig::from_toml(toml).unwrap();
    assert_eq!(config.tools.len(), 1);
    assert_eq!(config.tools[0].name, "hello_world");
    assert_eq!(config.tools[0].description, "A simple hello world script");
    assert!(config.tools[0].script.is_inline());
    assert_eq!(config.tools[0].script.get_inline().unwrap(), "print('Hello, World!')");
}

#[test]
fn test_tool_config_from_toml_with_parameters() {
    let toml = r#"
[[tools]]
name = "greet"
description = "Greet someone"
return_type = "String"

[[tools.parameters]]
name = "name"
type = "String"
required = true
description = "Name of the person to greet"

[tools.script]
path = "./scripts/greet.lua"
"#;
    
    let config = ToolConfig::from_toml(toml).unwrap();
    assert_eq!(config.tools.len(), 1);
    assert_eq!(config.tools[0].parameters.len(), 1);
    assert_eq!(config.tools[0].parameters[0].name, "name");
    assert_eq!(config.tools[0].parameters[0].param_type, ParameterType::String);
    assert!(config.tools[0].parameters[0].required);
    match &config.tools[0].script {
        ScriptSource::Path(path) => assert_eq!(path, "./scripts/greet.lua"),
        _ => panic!("Expected path script"),
    }
}

#[test]
fn test_tool_config_from_toml_multiple_tools() {
    let toml = r#"
[[tools]]
name = "tool1"
description = "First tool"
return_type = "String"

[tools.script]
inline = "return 'tool1'"

[[tools]]
name = "tool2"
description = "Second tool"
return_type = "Number"

[tools.script]
inline = "return 42"
"#;
    
    let config = ToolConfig::from_toml(toml).unwrap();
    assert_eq!(config.tools.len(), 2);
    assert_eq!(config.tools[0].name, "tool1");
    assert_eq!(config.tools[1].name, "tool2");
}

#[test]
fn test_tool_config_validation_missing_name() {
    let toml = r#"
[[tools]]
description = "Missing name"
return_type = "String"

[tools.script]
inline = "code"
"#;
    
    let result = ToolConfig::from_toml(toml);
    assert!(result.is_err());
}

#[test]
fn test_tool_config_validation_missing_script() {
    let toml = r#"
[[tools]]
name = "test"
description = "Missing script"
return_type = "String"
"#;
    
    let result = ToolConfig::from_toml(toml);
    assert!(result.is_err());
}

#[test]
fn test_tool_config_validation_both_script_sources() {
    let toml = r#"
[[tools]]
name = "test"
description = "Both inline and path"
return_type = "String"

[tools.script]
inline = "code"
path = "./script.lua"
"#;
    
    let result = ToolConfig::from_toml(toml);
    assert!(result.is_err());
}

#[test]
fn test_tool_config_to_toml() {
    let mut config = ToolConfig::default();
    let tool = ToolDefinition {
        name: "test_tool".to_string(),
        description: "Test description".to_string(),
        parameters: vec![
            ToolParameter {
                name: "param1".to_string(),
                param_type: ParameterType::String,
                required: true,
                description: Some("Parameter 1".to_string()),
            },
        ],
        return_type: ReturnType::String,
        script: ScriptSource::inline("print('test')".to_string()),
    };
    config.tools.push(tool);
    
    let toml = config.to_toml().unwrap();
    let parsed = ToolConfig::from_toml(&toml).unwrap();
    
    assert_eq!(parsed.tools.len(), 1);
    assert_eq!(parsed.tools[0].name, "test_tool");
}

#[tokio::test]
async fn test_tool_config_loader_new_with_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("tools.toml");
    
    let toml = r#"
[[tools]]
name = "test_tool"
description = "Test tool"
return_type = "String"

[tools.script]
inline = "return 'test'"
"#;
    
    fs::write(&config_path, toml).unwrap();
    
    let loader = loki::tools::config_loader::ToolConfigLoader::new(config_path.clone());
    assert!(loader.is_ok());
    
    let loader = loader.unwrap();
    let config = loader.get_config().await;
    assert_eq!(config.tools.len(), 1);
    assert_eq!(config.tools[0].name, "test_tool");
}

#[tokio::test]
async fn test_tool_config_loader_new_without_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("nonexistent.toml");
    
    let loader = loki::tools::config_loader::ToolConfigLoader::new(config_path.clone());
    assert!(loader.is_ok());
    
    let loader = loader.unwrap();
    let config = loader.get_config().await;
    assert!(config.tools.is_empty());
}
