use quick_xml::events::Event;
use quick_xml::reader::Reader;
use crate::actions::{ActionContext, ActionError, ActionDispatcher};
use std::collections::HashMap;
use serde_json::Value;

/// Dispatcher that parses structural XML protocol (Tasks and Calls)
pub struct XmlDispatcher {
    dispatcher: std::sync::Arc<ActionDispatcher>,
}

impl XmlDispatcher {
    pub fn new(dispatcher: std::sync::Arc<ActionDispatcher>) -> Self {
        Self { dispatcher }
    }

    /// Execute an action from an XML protocol blob (supports <task> and <call>)
    pub async fn execute_xml(&self, context: &ActionContext, xml_data: &str) -> Result<Vec<u8>, ActionError> {
        let mut reader = Reader::from_str(xml_data);
        reader.trim_text(true);

        let mut buf = Vec::new();
        
        // State for <task> parsing
        let mut is_task = false;
        let mut task_type: Option<String> = None;
        let mut task_args: HashMap<String, String> = HashMap::new();

        // State for <call> parsing
        let mut action_type: Option<String> = None;
        let mut call_args: HashMap<String, Value> = HashMap::new();
        
        let mut current_tag: Option<String> = None;
        let mut current_attr_name: Option<String> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    current_tag = Some(tag_name.clone());

                    match tag_name.as_str() {
                        "task" => {
                            is_task = true;
                        }
                        "collection" => {
                            task_type = Some("collection".to_string());
                            for attr in e.attributes() {
                                let attr = attr.map_err(|e| ActionError::InvalidAction(format!("XML attribute error: {}", e)))?;
                                if attr.key.as_ref() == b"type" {
                                    task_args.insert("collection_type".to_string(), String::from_utf8_lossy(&attr.value).to_string());
                                } else if attr.key.as_ref() == b"location" {
                                    task_args.insert("location".to_string(), String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                        "analysis" => {
                            task_type = Some("analysis".to_string());
                            for attr in e.attributes() {
                                let attr = attr.map_err(|e| ActionError::InvalidAction(format!("XML attribute error: {}", e)))?;
                                if attr.key.as_ref() == b"type" {
                                    task_args.insert("analysis_type".to_string(), String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                        "call" => {
                            for attr in e.attributes() {
                                let attr = attr.map_err(|e| ActionError::InvalidAction(format!("XML attribute error: {}", e)))?;
                                if attr.key.as_ref() == b"type" {
                                    action_type = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                        "arg" => {
                            for attr in e.attributes() {
                                let attr = attr.map_err(|e| ActionError::InvalidAction(format!("XML attribute error: {}", e)))?;
                                if attr.key.as_ref() == b"name" {
                                    current_attr_name = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().map_err(|e| ActionError::InvalidAction(format!("XML unescape error: {}", e)))?.into_owned();
                    
                    if let Some(ref tag) = current_tag {
                        if is_task {
                            if tag == "filter" {
                                task_args.insert("filter".to_string(), text);
                            } else if tag == "instruction" {
                                task_args.insert("instruction".to_string(), text);
                            }
                        } else {
                            if let Some(attr_name) = current_attr_name.take() {
                                let value = if (text.starts_with('[') && text.ends_with(']')) || 
                                               (text.starts_with('{') && text.ends_with('}')) {
                                    serde_json::from_str(&text).unwrap_or(Value::String(text))
                                } else {
                                    Value::String(text)
                                };
                                call_args.insert(attr_name, value);
                            }
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref());
                    if tag_name == "task" || tag_name == "call" {
                        break;
                    }
                    if tag_name == "instruction" {
                        task_type = Some("instruction".to_string());
                    }
                    current_tag = None;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(ActionError::InvalidAction(format!("XML parse error: {}", e))),
                _ => {}
            }
            buf.clear();
        }

        let result = if is_task {
            // If instructions was found during text parsing but task_type wasn't set by an element
            if task_type.is_none() && is_task {
                 // Fallback to instruction if we found content
            }
            self.handle_task(context, task_type, task_args).await
        } else if let Some(a_type) = action_type {
            let payload = serde_json::to_vec(&call_args)
                .map_err(|e| ActionError::InvalidAction(format!("Failed to serialize call arguments: {}", e)))?;
            self.dispatcher.dispatch(&a_type, context, &payload).await
        } else {
            return Err(ActionError::InvalidAction("No <task> or <call> found in XML".to_string()));
        };

        match result {
            Ok(data) => {
                let payload = String::from_utf8_lossy(&data);
                let response = format!(
                    "<response status=\"success\"><payload>{}</payload></response>",
                    payload
                );
                Ok(response.into_bytes())
            }
            Err(e) => {
                let response = format!(
                    "<response status=\"error\"><payload>{}</payload></response>",
                    e
                );
                Ok(response.into_bytes())
            }
        }
    }

    /// Translates a high-level <task> into one or more low-level <call> operations
    async fn handle_task(&self, context: &ActionContext, task_type: Option<String>, args: HashMap<String, String>) -> Result<Vec<u8>, ActionError> {
        let t_type = task_type.ok_or_else(|| ActionError::InvalidAction("Task missing type descriptor".to_string()))?;

        match t_type.as_str() {
            "collection" => {
                let c_type = args.get("collection_type").map(|s| s.as_str()).unwrap_or("unknown");
                let location = args.get("location").map(|s| s.as_str()).unwrap_or(".");
                let filter = args.get("filter").map(|s| s.as_str()).unwrap_or("*");

                tracing::info!("Thor: Translating intent <collection type='{}'> into execution", c_type);

                // Map intents to system tools (PREFER SANDBOX for safety)
                match c_type {
                    "file" => {
                        let payload = serde_json::json!({
                            "command": "ls",
                            "args": ["-la", filter],
                            "workdir": location
                        });
                        let data = serde_json::to_vec(&payload).unwrap();
                        self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                    }
                    "process" => {
                        let payload = serde_json::json!({
                            "command": "ps", // Use ps for unix-based sandbox
                            "args": ["aux"]
                        });
                        let data = serde_json::to_vec(&payload).unwrap();
                        self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                    }
                    "network" => {
                        let payload = serde_json::json!({
                            "command": "ip", 
                            "args": ["addr"]
                        });
                        let data = serde_json::to_vec(&payload).unwrap();
                        self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                    }
                    "logs" => {
                        let payload = serde_json::json!({
                            "command": "tail",
                            "args": ["-n", "50", "/var/log/messages"], // Generic path for Alpine
                            "workdir": location
                        });
                        let data = serde_json::to_vec(&payload).unwrap();
                        self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                    }
                    _ => Err(ActionError::InvalidAction(format!("Unsupported collection type: {}", c_type)))
                }
            }
            "analysis" => {
                let a_type = args.get("analysis_type").map(|s| s.as_str()).unwrap_or("unknown");
                tracing::info!("Thor: Translating analysis intent <analysis type='{}'> into execution", a_type);

                match a_type {
                    "resource" | "performance" => {
                        let payload = serde_json::json!({
                            "command": "top",
                            "args": ["-b", "-n", "1"]
                        });
                        let data = serde_json::to_vec(&payload).unwrap();
                        self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                    }
                    "security" => {
                        let payload = serde_json::json!({
                            "command": "netstat",
                            "args": ["-tuln"]
                        });
                        let data = serde_json::to_vec(&payload).unwrap();
                        self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                    }
                    "hardware" => {
                        let payload = serde_json::json!({
                            "command": "lscpu",
                            "args": []
                        });
                        let data = serde_json::to_vec(&payload).unwrap();
                        self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                    }
                    _ => Err(ActionError::InvalidAction(format!("Unsupported analysis type: {}", a_type)))
                }
            }
            "instruction" => {
                let instruction = args.get("instruction").map(|s| s.as_str()).unwrap_or("");
                tracing::info!("Thor: Processing declarative instruction: {}", instruction);

                // Simple heuristic: if instruction contains "Execute... command:", extract it
                if instruction.contains("Execute") && instruction.contains("command:") {
                    let cmd_part = instruction.split("command:").nth(1).unwrap_or("").trim();
                    let payload = serde_json::json!({
                        "command": cmd_part,
                        "args": []
                    });
                    let data = serde_json::to_vec(&payload).unwrap();
                    self.dispatcher.dispatch("SANDBOX_COMMAND", context, &data).await
                } else {
                    // Fallback or generic handling
                    Ok(format!("Instruction acknowledged (Sandboxed): {}", instruction).into_bytes())
                }
            }
            _ => Err(ActionError::InvalidAction(format!("Unsupported task type: {}", t_type))),
        }
    }
}
