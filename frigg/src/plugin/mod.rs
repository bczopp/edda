use async_trait::async_trait;

#[async_trait]
pub trait OdinPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn capabilities(&self) -> Vec<String>;
    
    async fn process_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>>;
}
