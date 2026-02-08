# Model Download Guide

## Overview

Geri uses local LLM models which need to be downloaded before use. This guide explains how to obtain and manage models for both llama.cpp and BitNet.cpp providers.

## Model Directory Structure

```
./models/
├── llamacpp/
│   ├── llama-3b.gguf
│   ├── llama-7b.gguf
│   ├── llama-8b.gguf
│   └── llama-13b.gguf
└── bitnet/
    ├── bitnet-3b.bitnet
    └── bitnet-7b.bitnet
```

## Automatic Model Detection

The `LocalLLMManager` automatically:
1. Detects your hardware (memory, CPU, GPU)
2. Selects the optimal provider (llama.cpp or BitNet)
3. Recommends model size (3B, 7B, 8B, 13B)
4. Checks if model exists
5. Provides download instructions if missing

## Recommended Models

### llama.cpp Models (GGUF Format)

| Size | Memory Required | Model | Download Source |
|------|-----------------|-------|-----------------|
| 3B | ~2GB | Llama-2-3B Q4_K_M | [HuggingFace](https://huggingface.co/TheBloke/Llama-2-3B-GGUF) |
| 7B | ~4GB | Llama-2-7B Q4_K_M | [HuggingFace](https://huggingface.co/TheBloke/Llama-2-7B-GGUF) |
| 8B | ~5GB | Llama-3-8B Q4_K_M | [HuggingFace](https://huggingface.co/meta-llama/Meta-Llama-3-8B) |
| 13B | ~8GB | Llama-2-13B Q4_K_M | [HuggingFace](https://huggingface.co/TheBloke/Llama-2-13B-GGUF) |

**Quantization Recommendation:** Q4_K_M provides the best balance of quality and size.

### BitNet Models (1-bit Quantization)

| Size | Memory Required | Model | Download Source |
|------|-----------------|-------|-----------------|
| 3B | ~390MB | BitNet-3B 1-bit | [HuggingFace](https://huggingface.co/microsoft/bitnet-3b) |
| 7B | ~910MB | BitNet-7B 1-bit | [HuggingFace](https://huggingface.co/microsoft/bitnet-7b) |

**Note:** BitNet models use **90% less memory** than full precision models.

## Manual Download Instructions

### Option 1: HuggingFace Hub (Recommended)

```bash
# Install huggingface-cli
pip install huggingface-hub

# Download llama.cpp model (example: 7B)
huggingface-cli download TheBloke/Llama-2-7B-GGUF llama-2-7b.Q4_K_M.gguf \
  --local-dir ./models/llamacpp \
  --local-dir-use-symlinks False

# Download BitNet model (example: 3B)
huggingface-cli download microsoft/bitnet-3b bitnet-3b-1bit.bitnet \
  --local-dir ./models/bitnet \
  --local-dir-use-symlinks False
```

### Option 2: Direct Download

```bash
# Create directories
mkdir -p ./models/llamacpp
mkdir -p ./models/bitnet

# Download with wget or curl
wget -P ./models/llamacpp \
  https://huggingface.co/TheBloke/Llama-2-7B-GGUF/resolve/main/llama-2-7b.Q4_K_M.gguf

# Or with curl
curl -L -o ./models/llamacpp/llama-2-7b.gguf \
  https://huggingface.co/TheBloke/Llama-2-7B-GGUF/resolve/main/llama-2-7b.Q4_K_M.gguf
```

### Option 3: Browser Download

1. Visit the model's HuggingFace page
2. Navigate to "Files and versions"
3. Click on the `.gguf` or `.bitnet` file
4. Save to the appropriate `./models/` subdirectory

## Programmatic Model Management

### Check Available Models

```rust
use geri::llm::local_manager::LocalLLMManager;

let manager = LocalLLMManager::new()?;

// List all available models
let models = manager.list_models().await?;
println!("Available models: {:?}", models);
```

### Get Recommended Download

```rust
use geri::llm::model_downloader::ModelDownloader;

let downloader = ModelDownloader::new("./models".to_string())?;

// Get recommended source for 7B llama.cpp model
if let Some(source) = downloader.get_recommended_source("7b", "llamacpp") {
    println!("Download from: {}", source.get_url());
}
```

### Check Model Existence

```rust
let downloader = ModelDownloader::new("./models".to_string())?;

if !downloader.check_model_exists("llama-7b.gguf") {
    println!("Model not found! Please download it.");
}
```

## Hardware-Based Recommendations

The system automatically recommends models based on your hardware:

| Available Memory | Provider | Model Size | Total Memory Usage |
|-----------------|----------|------------|-------------------|
| < 4 GB | BitNet | 3B | ~390 MB |
| 4-8 GB | BitNet | 7B | ~910 MB |
| 8-16 GB | llama.cpp | 8B | ~5 GB |
| > 16 GB | llama.cpp | 13B | ~8 GB |

## License Considerations

**Important:** Check the license for each model before downloading:

- **Llama 2**: [Meta License](https://ai.meta.com/llama/license/)
- **Llama 3**: [Meta License](https://llama.meta.com/llama3/license/)
- **BitNet**: Check specific model repository

Some models require acceptance of terms on HuggingFace before download.

## Storage Requirements

### Per Model

- **3B GGUF Q4**: ~2 GB
- **7B GGUF Q4**: ~4 GB
- **8B GGUF Q4**: ~5 GB
- **13B GGUF Q4**: ~8 GB
- **3B BitNet 1-bit**: ~390 MB
- **7B BitNet 1-bit**: ~910 MB

### Recommended Storage

- **Minimal** (low memory system): 1 GB (one BitNet 3B model)
- **Standard** (medium memory): 5 GB (one BitNet 7B + one llama.cpp 7B)
- **Full** (high memory): 20 GB (multiple model sizes)

## Troubleshooting

### Model Not Found Error

```
ERROR: Model llama-7b.gguf not found at ./models/llamacpp/llama-7b.gguf
```

**Solution:** Download the model using one of the methods above.

### Out of Memory Error

```
ERROR: Failed to load model: out of memory
```

**Solutions:**
1. Use a smaller model size (e.g., 3B instead of 7B)
2. Switch to BitNet provider (90% memory reduction)
3. Enable GPU offloading if available

### Slow Download

**Tips:**
- Use a download manager for resume capability
- Download during off-peak hours
- Consider using a mirror if available
- Use HuggingFace CLI with `--resume-download` flag

## Future: Automatic Download

**Note:** Automatic download functionality is currently in development. The ModelDownloader has the infrastructure ready but requires implementation of:

1. Streaming download with progress tracking
2. Checksum verification
3. Resume capability for interrupted downloads
4. Bandwidth throttling options

For now, please download models manually using the instructions above.

## See Also

- [Local Providers Guide](./LOCAL_PROVIDERS.md)
- [Configuration Guide](../config/geri.example.json)
- [llama.cpp Documentation](https://github.com/ggerganov/llama.cpp)
- [BitNet Documentation](https://github.com/microsoft/BitNet)
