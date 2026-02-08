//! XML Protocol Instructions for Geri prompts (Phase 8.1.1).

/// The standard system prompt additions for the Edda XML protocol.
pub const XML_PROTOCOL_INSTRUCTIONS: &str = r#"
### INTER-AGENT XML PROTOCOL
You must communicate using the following XML tags when requesting actions or defining goals.

1. High-Level Intent (<task>)
Use this when you have a goal but don't know the exact tool command.
- `<collection type="file|process|network" location="..." />`: Request data collection.
- `<instruction>description</instruction>`: A declarative command for a complex operation.
- `<analysis type="resource|security|log">...</analysis>`: Request a deep-dive analysis.

2. Atomic Execution (<call>)
Use this when you know exactly which tool to call.
- `<thinking>your internal reasoning</thinking>`: EXPLAIN your plan before action.
- `<call type="SANDBOX_COMMAND">`: Execute a command in a secure container.
  - `<arg name="command">ls</arg>`
  - `<arg name="args">["-la"]</arg>`

3. Response Recognition
All actions return a `<response status="success|error">` tag.
"#;

/// Injects the XML protocol instructions into a system prompt.
pub fn inject_xml_protocol(system_prompt: &str) -> String {
    if system_prompt.contains("XML PROTOCOL") {
        return system_prompt.to_string();
    }
    format!("{}\n\n{}", system_prompt, XML_PROTOCOL_INSTRUCTIONS)
}
