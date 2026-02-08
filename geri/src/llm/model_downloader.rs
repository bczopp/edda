use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::{info, warn, error};

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Failed to download model: {0}")]
    DownloadFailed(String),
    #[error("Invalid model source: {0}")]
    InvalidSource(String),
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Model source for downloading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSource {
    /// HuggingFace model repository
    HuggingFace {
        repo: String,
        file: String,
    },
    /// Direct download URL
    DirectUrl {
        url: String,
    },
}

impl ModelSource {
    /// Get the download URL from the source
    pub fn get_url(&self) -> String {
        match self {
            ModelSource::HuggingFace { repo, file } => {
                format!("https://huggingface.co/{}/resolve/main/{}", repo, file)
            }
            ModelSource::DirectUrl { url } => url.clone(),
        }
    }
    
    /// Validate the source
    pub fn validate(&self) -> Result<(), DownloadError> {
        match self {
            ModelSource::HuggingFace { repo, file } => {
                if repo.is_empty() || file.is_empty() {
                    return Err(DownloadError::InvalidSource(
                        "HuggingFace repo and file cannot be empty".to_string()
                    ));
                }
                Ok(())
            }
            ModelSource::DirectUrl { url } => {
                if url.is_empty() {
                    return Err(DownloadError::InvalidSource(
                        "Direct URL cannot be empty".to_string()
                    ));
                }
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    return Err(DownloadError::InvalidSource(
                        "Direct URL must start with http:// or https://".to_string()
                    ));
                }
                Ok(())
            }
        }
    }
}

/// Download progress information
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percentage: f64,
}

impl DownloadProgress {
    pub fn new(total_bytes: u64) -> Self {
        Self {
            downloaded_bytes: 0,
            total_bytes,
            percentage: 0.0,
        }
    }
    
    pub fn update(&mut self, downloaded_bytes: u64) {
        self.downloaded_bytes = downloaded_bytes;
        self.percentage = if self.total_bytes > 0 {
            (downloaded_bytes as f64 / self.total_bytes as f64) * 100.0
        } else {
            0.0
        };
    }
}

/// Model downloader for automatic model downloads
pub struct ModelDownloader {
    models_dir: PathBuf,
}

impl ModelDownloader {
    /// Create a new model downloader
    /// 
    /// # Arguments
    /// 
    /// * `models_dir` - Directory to store downloaded models
    pub fn new(models_dir: String) -> Result<Self, DownloadError> {
        let path = PathBuf::from(models_dir);
        
        // Create directory if it doesn't exist
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
            info!("Created models directory: {:?}", path);
        }
        
        Ok(Self {
            models_dir: path,
        })
    }
    
    /// Check if a model exists locally
    pub fn check_model_exists(&self, model_name: &str) -> bool {
        let path = self.models_dir.join(model_name);
        path.exists()
    }
    
    /// Get the full path for a model
    pub fn get_model_path(&self, model_name: &str) -> PathBuf {
        self.models_dir.join(model_name)
    }
    
    /// List all models in the directory
    pub async fn list_models(&self) -> Result<Vec<String>, DownloadError> {
        if !self.models_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut models = Vec::new();
        
        let entries = std::fs::read_dir(&self.models_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(name) = file_name.to_str() {
                        // Filter for model files (.gguf, .bitnet)
                        if name.ends_with(".gguf") || name.ends_with(".bitnet") {
                            models.push(name.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(models)
    }
    
    /// Download a model from a source
    /// 
    /// # Arguments
    /// 
    /// * `source` - Model source to download from
    /// * `output_name` - Name for the downloaded file
    /// 
    /// # Note
    /// 
    /// This is currently a stub. Real implementation would use reqwest
    /// with progress tracking.
    pub async fn download(
        &self,
        source: ModelSource,
        output_name: &str,
    ) -> Result<PathBuf, DownloadError> {
        source.validate()?;
        
        let url = source.get_url();
        let output_path = self.models_dir.join(output_name);
        
        info!("Downloading model from {} to {:?}", url, output_path);
        
        // TODO: Implement actual download with reqwest
        // For now, return an error indicating this is a stub
        warn!("Model download is not yet implemented (stub)");
        
        Err(DownloadError::DownloadFailed(
            "Download functionality not yet implemented. Please download models manually.".to_string()
        ))
    }
    
    /// Get recommended model source for a given size and provider
    pub fn get_recommended_source(&self, model_size: &str, provider_type: &str) -> Option<ModelSource> {
        match (provider_type, model_size) {
            // llama.cpp recommendations (TheBloke's HuggingFace)
            ("llamacpp", "3b") => Some(ModelSource::HuggingFace {
                repo: "TheBloke/Llama-2-3B-GGUF".to_string(),
                file: "llama-2-3b.Q4_K_M.gguf".to_string(),
            }),
            ("llamacpp", "7b") => Some(ModelSource::HuggingFace {
                repo: "TheBloke/Llama-2-7B-GGUF".to_string(),
                file: "llama-2-7b.Q4_K_M.gguf".to_string(),
            }),
            ("llamacpp", "8b") => Some(ModelSource::HuggingFace {
                repo: "meta-llama/Meta-Llama-3-8B".to_string(),
                file: "llama-3-8b.Q4_K_M.gguf".to_string(),
            }),
            ("llamacpp", "13b") => Some(ModelSource::HuggingFace {
                repo: "TheBloke/Llama-2-13B-GGUF".to_string(),
                file: "llama-2-13b.Q4_K_M.gguf".to_string(),
            }),
            
            // BitNet recommendations
            ("bitnet", "3b") => Some(ModelSource::HuggingFace {
                repo: "microsoft/bitnet-3b".to_string(),
                file: "bitnet-3b-1bit.bitnet".to_string(),
            }),
            ("bitnet", "7b") => Some(ModelSource::HuggingFace {
                repo: "microsoft/bitnet-7b".to_string(),
                file: "bitnet-7b-1bit.bitnet".to_string(),
            }),
            
            _ => None,
        }
    }
    
    /// Download model if not exists
    /// 
    /// Checks if model exists, if not, downloads it from recommended source
    pub async fn ensure_model(&self, model_size: &str, provider_type: &str) -> Result<PathBuf, DownloadError> {
        // Generate expected filename
        let filename = match provider_type {
            "llamacpp" => format!("llama-{}.gguf", model_size),
            "bitnet" => format!("bitnet-{}.bitnet", model_size),
            _ => return Err(DownloadError::InvalidSource(
                format!("Unknown provider type: {}", provider_type)
            )),
        };
        
        // Check if exists
        if self.check_model_exists(&filename) {
            info!("Model {} already exists, skipping download", filename);
            return Ok(self.get_model_path(&filename));
        }
        
        // Get recommended source
        let source = self.get_recommended_source(model_size, provider_type)
            .ok_or_else(|| DownloadError::ModelNotFound(
                format!("No recommended source for {} {} model", provider_type, model_size)
            ))?;
        
        // Download
        warn!("Model {} not found locally. Automatic download not yet implemented.", filename);
        warn!("Please download manually from: {}", source.get_url());
        
        Err(DownloadError::ModelNotFound(
            format!("Model {} not found. Please download from: {}", filename, source.get_url())
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_source_huggingface() {
        let source = ModelSource::HuggingFace {
            repo: "TheBloke/Test".to_string(),
            file: "model.gguf".to_string(),
        };
        
        assert!(source.validate().is_ok());
        assert!(source.get_url().contains("huggingface.co"));
    }
    
    #[test]
    fn test_model_source_direct_url() {
        let source = ModelSource::DirectUrl {
            url: "https://example.com/model.gguf".to_string(),
        };
        
        assert!(source.validate().is_ok());
        assert_eq!(source.get_url(), "https://example.com/model.gguf");
    }
    
    #[test]
    fn test_download_progress() {
        let mut progress = DownloadProgress::new(1000);
        assert_eq!(progress.percentage, 0.0);
        
        progress.update(500);
        assert_eq!(progress.percentage, 50.0);
        
        progress.update(1000);
        assert_eq!(progress.percentage, 100.0);
    }
    
    #[tokio::test]
    async fn test_check_model_exists() {
        use tempfile::tempdir;
        
        let temp_dir = tempdir().unwrap();
        let downloader = ModelDownloader::new(temp_dir.path().to_string_lossy().to_string())
            .expect("Failed to create downloader");
        
        assert!(!downloader.check_model_exists("nonexistent.gguf"));
    }
}
