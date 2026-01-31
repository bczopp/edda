use frigg::FriggPlugin;

#[tokio::test]
async fn test_frigg_plugin_name() {
    let plugin = FriggPlugin::new();
    assert_eq!(plugin.name(), "frigg");
}

#[tokio::test]
async fn test_frigg_plugin_capabilities() {
    let plugin = FriggPlugin::new();
    let caps = plugin.capabilities();
    assert!(caps.contains(&"health_questions".to_string()));
    assert!(caps.contains(&"mental_health".to_string()));
    assert!(caps.contains(&"physical_health".to_string()));
    assert!(caps.contains(&"certified_courses".to_string()));
}

#[tokio::test]
async fn test_frigg_plugin_process_mental_health_request() {
    let plugin = FriggPlugin::new();
    let result = plugin.process_request("I have anxiety").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Mental health"));
}

#[tokio::test]
async fn test_frigg_plugin_process_physical_health_request() {
    let plugin = FriggPlugin::new();
    let result = plugin.process_request("I have pain in my back").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Physical health"));
}

#[tokio::test]
async fn test_frigg_plugin_process_course_request() {
    let plugin = FriggPlugin::new();
    let result = plugin.process_request("Tell me about certified courses").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("course"));
}

#[tokio::test]
async fn test_frigg_plugin_process_generic_request() {
    let plugin = FriggPlugin::new();
    let result = plugin.process_request("General health question").await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Frigg processed"));
}
