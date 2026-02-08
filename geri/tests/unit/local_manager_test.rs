use geri::llm::local_manager::{LocalLLMManager, HardwareProfile, LocalModelConfig};

#[tokio::test]
async fn test_manager_creation() {
    let manager = LocalLLMManager::new();
    assert!(manager.is_ok());
}

#[tokio::test]
async fn test_hardware_detection() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    let profile = manager.detect_hardware();
    
    // Should detect some hardware profile
    assert!(profile.total_memory_mb > 0);
    assert!(profile.available_memory_mb > 0);
    assert!(profile.cpu_cores > 0);
}

#[tokio::test]
async fn test_select_provider_high_memory() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    
    // Simulate high-memory system (16GB+)
    let profile = HardwareProfile {
        total_memory_mb: 16384,
        available_memory_mb: 12000,
        cpu_cores: 8,
        has_gpu: false,
        gpu_memory_mb: 0,
    };
    
    let selection = manager.select_provider(&profile);
    
    // High memory system should prefer llama.cpp for better quality
    assert_eq!(selection.provider_type, "llamacpp");
}

#[tokio::test]
async fn test_select_provider_low_memory() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    
    // Simulate low-memory system (< 4GB)
    let profile = HardwareProfile {
        total_memory_mb: 3072,
        available_memory_mb: 1500,
        cpu_cores: 4,
        has_gpu: false,
        gpu_memory_mb: 0,
    };
    
    let selection = manager.select_provider(&profile);
    
    // Low memory system should prefer BitNet for efficiency
    assert_eq!(selection.provider_type, "bitnet");
}

#[tokio::test]
async fn test_select_provider_mobile() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    
    // Simulate mobile device (2GB RAM, battery-constrained)
    let profile = HardwareProfile {
        total_memory_mb: 2048,
        available_memory_mb: 800,
        cpu_cores: 4,
        has_gpu: false,
        gpu_memory_mb: 0,
    };
    
    let selection = manager.select_provider(&profile);
    
    // Mobile should always use BitNet
    assert_eq!(selection.provider_type, "bitnet");
    assert_eq!(selection.model_size, "3b"); // Smallest viable model
}

#[tokio::test]
async fn test_model_config_generation() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    
    let profile = HardwareProfile {
        total_memory_mb: 8192,
        available_memory_mb: 5000,
        cpu_cores: 4,
        has_gpu: false,
        gpu_memory_mb: 0,
    };
    
    let config = manager.generate_model_config(&profile);
    
    assert!(!config.model_path.is_empty());
    assert!(config.n_ctx > 0);
    assert!(config.n_threads > 0);
    assert_eq!(config.n_threads, 4); // Should match CPU cores
}

#[tokio::test]
async fn test_recommend_model_size() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    
    // < 4GB RAM -> 3B model
    let profile_small = HardwareProfile {
        total_memory_mb: 3000,
        available_memory_mb: 1500,
        cpu_cores: 4,
        has_gpu: false,
        gpu_memory_mb: 0,
    };
    assert_eq!(manager.recommend_model_size(&profile_small), "3b");
    
    // 4-8GB RAM -> 7B model
    let profile_medium = HardwareProfile {
        total_memory_mb: 6000,
        available_memory_mb: 4000,
        cpu_cores: 4,
        has_gpu: false,
        gpu_memory_mb: 0,
    };
    assert_eq!(manager.recommend_model_size(&profile_medium), "7b");
    
    // > 8GB RAM -> 8B model (or 13B if > 16GB)
    let profile_large = HardwareProfile {
        total_memory_mb: 16000,
        available_memory_mb: 12000,
        cpu_cores: 8,
        has_gpu: false,
        gpu_memory_mb: 0,
    };
    assert_eq!(manager.recommend_model_size(&profile_large), "8b");
}

#[tokio::test]
async fn test_gpu_detection_and_offloading() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    
    // System with GPU
    let profile = HardwareProfile {
        total_memory_mb: 8192,
        available_memory_mb: 5000,
        cpu_cores: 6,
        has_gpu: true,
        gpu_memory_mb: 8000,
    };
    
    let config = manager.generate_model_config(&profile);
    
    // Should enable GPU offloading
    assert!(config.n_gpu_layers > 0);
}

#[tokio::test]
async fn test_auto_select_and_create_provider() {
    let manager = LocalLLMManager::new().expect("manager creation failed");
    
    // Auto-detect hardware and create provider
    let result = manager.create_provider_auto().await;
    
    // Should succeed (even with stub clients)
    assert!(result.is_ok());
}
