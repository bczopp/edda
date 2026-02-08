//! Tests for Geri gRPC Client

use skuld::geri_client::GeriClient;

#[tokio::test]
#[ignore] // Requires Geri server running
async fn test_geri_client_list_models() {
    let mut client = GeriClient::connect("http://localhost:50052".to_string())
        .await
        .expect("connect to geri");
    
    let models = client.list_models(None, None).await.expect("list_models");
    // In real scenario with running server, would assert on actual models
    println!("Listed {} models", models.len());
}

#[tokio::test]
#[ignore] // Requires Geri server running
async fn test_geri_client_get_model_info() {
    let mut client = GeriClient::connect("http://localhost:50052".to_string())
        .await
        .expect("connect to geri");
    
    // This would require a known model ID from the registry
    // For now, just test the connection
    let result = client.get_model_info("gpt-4").await;
    println!("Get model info result: {:?}", result.is_ok());
}

// Unit test for ModelInfo conversion
#[test]
fn test_model_info_from_proto() {
    let proto = skuld::geri_client::client::geri::ModelInfo {
        id: "test-id".to_string(),
        name: "Test Model".to_string(),
        provider: "test-provider".to_string(),
        model_type: "llm".to_string(),
        parameter_count: 1000000,
        hardware_requirements: "GPU".to_string(),
        context_window: 4096,
    };
    
    let model_info = skuld::geri_client::ModelInfo::from(proto);
    assert_eq!(model_info.id, "test-id");
    assert_eq!(model_info.name, "Test Model");
    assert_eq!(model_info.provider, "test-provider");
    assert_eq!(model_info.model_type, "llm");
    assert_eq!(model_info.parameter_count, 1000000);
    assert_eq!(model_info.context_window, 4096);
}
