# Local LLM Providers - llama.cpp & BitNet.cpp

## Overview

Geri supports two local LLM providers for on-device inference:

1. **llama.cpp** - High-quality GGUF format models
2. **BitNet.cpp** - Ultra-efficient 1-bit quantized models

## llama.cpp Provider

### Features

- GGUF format model support
- CPU and GPU acceleration
- Configurable context size
- Multi-threading support
- Compatible with Llama, Mistral, and other GGUF models

### Usage

```rust
use geri::llm::llamacpp::{LlamaCppClient, LlamaCppConfig, LlamaCppLLMProvider};
use geri::llm::provider::{LLMProvider, PromptRequest};

// Configure the client
let config = LlamaCppConfig {
    model_path: "/path/to/llama-3-8b.gguf".to_string(),
    n_ctx: 2048,           // Context window size
    n_threads: 4,          // Number of CPU threads
    n_gpu_layers: 0,       // GPU layers (0 = CPU only)
};

// Create client and provider
let client = LlamaCppClient::new(config)?;
let provider = LlamaCppLLMProvider::new(client);

// Process a prompt
let request = PromptRequest {
    prompt: "What is the capital of France?".to_string(),
    context: None,
    max_tokens: Some(100),
};

let response = provider.process_prompt(request).await?;
println!("Response: {}", response.text);
println!("Tokens used: {}", response.tokens_used);
```

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `model_path` | String | Path to GGUF model file | Required |
| `n_ctx` | u32 | Context window size (tokens) | Required |
| `n_threads` | u32 | Number of CPU threads | Required |
| `n_gpu_layers` | u32 | GPU layers to offload (0 = CPU) | Required |

### Recommended Models

- **Llama 3 8B** - Best balance of quality and performance
- **Mistral 7B** - Fast inference, good quality
- **Llama 3 70B** - Highest quality (requires more resources)

## BitNet.cpp Provider

### Features

- **90% memory reduction** vs. full precision models
- **5-10x faster inference** speed
- 1-bit quantization with maintained quality
- Extreme efficiency mode for resource-constrained devices
- Ideal for mobile and edge computing

### Usage

```rust
use geri::llm::bitnet::{BitNetClient, BitNetConfig, BitNetLLMProvider};
use geri::llm::provider::{LLMProvider, PromptRequest};

// Configure the client
let config = BitNetConfig {
    model_path: "/path/to/bitnet-3b.bitnet".to_string(),
    n_ctx: 2048,                     // Context window size
    n_threads: 4,                    // Number of CPU threads
    use_extreme_efficiency: true,    // Enable extreme efficiency mode
};

// Create client and provider
let client = BitNetClient::new(config)?;
let provider = BitNetLLMProvider::new(client);

// Check memory efficiency
println!("Estimated memory usage: {}MB", provider.estimated_memory_mb());
println!("Bit depth: {}-bit", provider.bit_depth());

// Process a prompt
let request = PromptRequest {
    prompt: "Explain quantum computing".to_string(),
    context: None,
    max_tokens: Some(100),
};

let response = provider.process_prompt(request).await?;
println!("Response: {}", response.text);
```

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `model_path` | String | Path to BitNet model file | Required |
| `n_ctx` | u32 | Context window size (tokens) | Required |
| `n_threads` | u32 | Number of CPU threads | Required |
| `use_extreme_efficiency` | bool | Enable extreme efficiency optimizations | Required |

### Memory Comparison

| Model | Full Precision (FP16) | BitNet (1-bit) | Reduction |
|-------|----------------------|----------------|-----------|
| 3B params | ~6 GB | ~390 MB | 93% |
| 7B params | ~14 GB | ~910 MB | 94% |
| 13B params | ~26 GB | ~1.7 GB | 93% |

### Recommended Use Cases

- **Mobile devices** - Low memory footprint
- **Edge computing** - Fast inference on limited hardware
- **Battery-constrained devices** - Lower power consumption
- **Real-time applications** - 5-10x faster inference

## LocalLLMManager - Automatic Provider Selection

The `LocalLLMManager` automatically selects the best local provider based on your hardware.

### Features

- **Automatic hardware detection** (memory, CPU, GPU)
- **Smart provider selection** (llama.cpp vs. BitNet)
- **Model size recommendations** based on available resources
- **Configuration-driven** with fallback to auto-detection

### Usage

```rust
use geri::llm::local_manager::LocalLLMManager;

// Create manager
let manager = LocalLLMManager::new()?;

// Auto-detect and create provider
let provider = manager.create_provider_auto().await?;

// Process prompts
let request = PromptRequest {
    prompt: "Your query".to_string(),
    context: None,
    max_tokens: Some(100),
};

let response = provider.process_prompt(request).await?;
```

### Selection Logic

| Available Memory | Selected Provider | Model Size | Rationale |
|-----------------|-------------------|------------|-----------|
| < 4 GB | BitNet | 3B | Extreme efficiency required |
| 4-8 GB | BitNet | 7B | Balance efficiency & quality |
| 8-16 GB | llama.cpp | 8B | Better quality available |
| > 16 GB | llama.cpp | 13B | Maximum quality |

### Configuration

Add to `config/geri.json`:

```json
{
  "local_provider": {
    "provider_type": "auto",
    "llamacpp_models_dir": "./models/llamacpp",
    "bitnet_models_dir": "./models/bitnet",
    "auto_select": true,
    "llamacpp_min_memory_mb": 8000
  }
}
```

**Options:**
- `provider_type`: `"auto"` (hardware-based), `"llamacpp"` (force), or `"bitnet"` (force)
- `auto_select`: `true` for automatic, `false` for explicit provider_type
- `llamacpp_min_memory_mb`: Minimum memory threshold for llama.cpp (default: 8000MB)

### Manual Provider Selection

```rust
use geri::llm::local_manager::{LocalLLMManager, LocalModelConfig};

let manager = LocalLLMManager::new()?;

// Detect hardware
let hardware = manager.detect_hardware();
println!("Available memory: {}MB", hardware.available_memory_mb);
println!("CPU cores: {}", hardware.cpu_cores);

// Get recommendation
let selection = manager.select_provider(&hardware);
println!("Recommended: {} with {} model", 
    selection.provider_type, 
    selection.model_size
);

// Create with specific config
let config = LocalModelConfig {
    model_path: "/custom/path/model.gguf".to_string(),
    n_ctx: 4096,
    n_threads: 8,
    n_gpu_layers: 16,
};

let provider = manager.create_provider("llamacpp", config).await?;
```

## Comparison: llama.cpp vs. BitNet.cpp

| Feature | llama.cpp | BitNet.cpp |
|---------|-----------|------------|
| Model format | GGUF | BitNet |
| Quantization | 2-8 bit | 1-bit |
| Memory usage | Moderate | Very low (90% reduction) |
| Inference speed | Fast | Very fast (5-10x) |
| Quality | High | Good (optimized training) |
| Best for | General use | Resource-constrained devices |

## Integration with Model Selection

Both providers integrate with Geri's multi-factor model selection system:

```rust
use geri::selection::selector::ModelSelector;
use geri::selection::efficiency::EfficiencyScoreCalculator;

// Create both providers
let llamacpp_provider = create_llamacpp_provider()?;
let bitnet_provider = create_bitnet_provider()?;

// Register in model selector
let selector = ModelSelector::new();
selector.add_provider(Box::new(llamacpp_provider));
selector.add_provider(Box::new(bitnet_provider));

// Automatic selection based on:
// - Available memory
// - Required inference speed
// - Quality requirements
// - Power constraints (mobile/battery)
let best_provider = selector.select_best_provider(&requirements)?;
```

## Error Handling

Both providers use typed errors:

```rust
use geri::llm::llamacpp::LlamaCppError;
use geri::llm::bitnet::BitNetError;

match client.generate(prompt, max_tokens).await {
    Ok(text) => println!("Generated: {}", text),
    Err(LlamaCppError::ModelLoadFailed(e)) => {
        eprintln!("Failed to load model: {}", e);
    }
    Err(LlamaCppError::GenerationFailed(e)) => {
        eprintln!("Generation failed: {}", e);
    }
    Err(LlamaCppError::InvalidConfig(e)) => {
        eprintln!("Invalid config: {}", e);
    }
}
```

## FFI Integration (Future)

Current implementation uses stubs for the FFI layer. Future updates will integrate:

- **llama.cpp**: Via `llama-cpp-sys` or direct FFI bindings
- **BitNet.cpp**: Via custom FFI bindings to BitNet.cpp library

The API will remain unchanged when FFI is added.

## Testing

Run tests for local providers:

```bash
# Test llama.cpp provider
cargo test --test llamacpp_client_test
cargo test --test llamacpp_llm_provider_test

# Test BitNet.cpp provider
cargo test --test bitnet_client_test
cargo test --test bitnet_llm_provider_test

# Run all local provider tests
cargo test llamacpp bitnet
```

## Platform Support

| Platform | llama.cpp | BitNet.cpp |
|----------|-----------|------------|
| Linux | ✅ | ✅ |
| macOS | ✅ | ✅ |
| Windows | ✅ | ✅ |
| Mobile (iOS/Android) | ✅ | ✅ Recommended |
| Edge devices | ⚠️ Limited | ✅ Optimized |
