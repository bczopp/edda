use valkyries::ValkyriesPlugin;

#[tokio::test]
async fn test_valkyries_plugin_name() {
    let plugin = ValkyriesPlugin::new();
    assert_eq!(plugin.name(), "valkyries");
}

#[tokio::test]
async fn test_valkyries_plugin_capabilities() {
    let plugin = ValkyriesPlugin::new();
    let caps = plugin.capabilities();
    assert!(caps.contains(&"code_analysis".to_string()));
    assert!(caps.contains(&"code_generation".to_string()));
    assert!(caps.contains(&"documentation".to_string()));
    assert!(caps.contains(&"refactoring".to_string()));
}

#[tokio::test]
async fn test_valkyries_plugin_process_document_request() {
    let plugin = ValkyriesPlugin::new();
    let result = plugin.process_request("Please document this code").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Documentation"));
}

#[tokio::test]
async fn test_valkyries_plugin_process_analyze_request() {
    let plugin = ValkyriesPlugin::new();
    let result = plugin.process_request("Analyze this function").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("analysis"));
}

#[tokio::test]
async fn test_valkyries_plugin_process_generate_request() {
    let plugin = ValkyriesPlugin::new();
    let result = plugin.process_request("Generate a hello world").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("generated"));
}

#[tokio::test]
async fn test_valkyries_plugin_process_generic_request() {
    let plugin = ValkyriesPlugin::new();
    let result = plugin.process_request("Do something").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Valkyries processed request"));
}
