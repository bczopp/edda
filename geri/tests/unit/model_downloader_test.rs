use geri::llm::model_downloader::{ModelDownloader, ModelSource, DownloadProgress};

#[tokio::test]
async fn test_downloader_creation() {
    let downloader = ModelDownloader::new("./test_models".to_string());
    assert!(downloader.is_ok());
}

#[tokio::test]
async fn test_check_model_exists() {
    let downloader = ModelDownloader::new("./test_models".to_string())
        .expect("downloader creation failed");
    
    // Non-existent model should return false
    let exists = downloader.check_model_exists("llama-3-8b.gguf");
    assert!(!exists);
}

#[tokio::test]
async fn test_model_source_url_generation() {
    // HuggingFace source
    let source = ModelSource::HuggingFace {
        repo: "TheBloke/Llama-2-7B-GGUF".to_string(),
        file: "llama-2-7b.Q4_K_M.gguf".to_string(),
    };
    
    let url = source.get_url();
    assert!(url.contains("huggingface.co"));
    assert!(url.contains("TheBloke"));
    assert!(url.contains("llama-2-7b.Q4_K_M.gguf"));
}

#[tokio::test]
async fn test_download_progress_tracking() {
    let mut progress = DownloadProgress {
        downloaded_bytes: 0,
        total_bytes: 1000,
        percentage: 0.0,
    };
    
    // Update progress
    progress.downloaded_bytes = 500;
    progress.percentage = (progress.downloaded_bytes as f64 / progress.total_bytes as f64) * 100.0;
    
    assert_eq!(progress.percentage, 50.0);
}

#[tokio::test]
async fn test_get_model_path() {
    let downloader = ModelDownloader::new("./test_models".to_string())
        .expect("downloader creation failed");
    
    let path = downloader.get_model_path("llama-3-8b.gguf");
    assert!(path.ends_with("llama-3-8b.gguf"));
}

#[tokio::test]
async fn test_list_models() {
    let downloader = ModelDownloader::new("./test_models".to_string())
        .expect("downloader creation failed");
    
    // Should return empty list for non-existent directory
    let models = downloader.list_models().await;
    assert!(models.is_ok());
}

#[tokio::test]
async fn test_validate_download_url() {
    let source_hf = ModelSource::HuggingFace {
        repo: "valid/repo".to_string(),
        file: "model.gguf".to_string(),
    };
    assert!(source_hf.validate().is_ok());
    
    let source_direct = ModelSource::DirectUrl {
        url: "https://example.com/model.gguf".to_string(),
    };
    assert!(source_direct.validate().is_ok());
    
    // Invalid URL
    let source_invalid = ModelSource::DirectUrl {
        url: "".to_string(),
    };
    assert!(source_invalid.validate().is_err());
}

#[tokio::test]
async fn test_recommended_models() {
    let downloader = ModelDownloader::new("./test_models".to_string())
        .expect("downloader creation failed");
    
    // Get recommendations for different sizes
    let rec_3b = downloader.get_recommended_source("3b", "llamacpp");
    assert!(rec_3b.is_some());
    
    let rec_7b = downloader.get_recommended_source("7b", "llamacpp");
    assert!(rec_7b.is_some());
    
    let rec_bitnet = downloader.get_recommended_source("3b", "bitnet");
    assert!(rec_bitnet.is_some());
}
