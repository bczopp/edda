# Huginn & Muninn Configuration Guide

## Overview

This guide describes the configuration options for Huginn & Muninn services.

## Audio Device Configuration

### Audio Input Devices

Huginn uses audio input devices for STT (Speech-to-Text) processing.

**Configuration:**
- Device selection: Automatic or manual device selection
- Sample rate: 16000 Hz (default), 44100 Hz, 48000 Hz
- Channels: Mono (1) or Stereo (2)
- Buffer size: Configurable buffer size for audio processing

**Example:**
```json
{
  "audio": {
    "input_device": "default",
    "sample_rate": 16000,
    "channels": 1,
    "buffer_size_ms": 100
  }
}
```

### Audio Output Devices

Muninn uses audio output devices for TTS (Text-to-Speech) playback.

**Configuration:**
- Device selection: Automatic or manual device selection
- Sample rate: 16000 Hz (default), 44100 Hz, 48000 Hz
- Channels: Mono (1) or Stereo (2)

**Example:**
```json
{
  "audio": {
    "output_device": "default",
    "sample_rate": 44100,
    "channels": 2
  }
}
```

## Quality Settings

### STT Quality Settings

**Configuration:**
- Model size: Small, Medium, Large (affects accuracy vs. speed)
- Language model: Language-specific models for better accuracy
- Confidence threshold: Minimum confidence score (0.0-1.0)

**Example:**
```json
{
  "stt": {
    "model_size": "medium",
    "language": "en-US",
    "confidence_threshold": 0.7
  }
}
```

### TTS Quality Settings

**Configuration:**
- Voice quality: Standard, High, Ultra (affects file size vs. quality)
- Sample rate: 16000 Hz, 22050 Hz, 44100 Hz, 48000 Hz
- Bitrate: For compressed formats (MP3)

**Example:**
```json
{
  "tts": {
    "voice_quality": "high",
    "sample_rate": 44100,
    "bitrate": 192
  }
}
```

## Voice Settings

### Voice Selection

**Configuration:**
- Voice type: Male, Female, Neutral
- Voice ID: Specific voice identifier (if available)
- Language: Language code for voice selection

**Example:**
```json
{
  "tts": {
    "voice": {
      "type": "female",
      "language": "en-US"
    }
  }
}
```

### Voice Parameters

**Configuration:**
- Speed: Speech rate (0.5-2.0, default: 1.0)
- Pitch: Pitch adjustment (-12 to +12 semitones, default: 0)
- Volume: Volume level (0.0-1.0, default: 1.0)

**Example:**
```json
{
  "tts": {
    "voice": {
      "speed": 1.0,
      "pitch": 0.0,
      "volume": 1.0
    }
  }
}
```

## Cache Settings

### TTS Cache Configuration

**Configuration:**
- Max cache size: Maximum number of cached entries (default: 100)
- TTL: Time-to-live for cache entries in seconds (default: 3600)
- Cache directory: Directory for persistent cache (optional)

**Example:**
```json
{
  "cache": {
    "tts": {
      "max_size": 100,
      "ttl_seconds": 3600,
      "persistent": false
    }
  }
}
```

## Logging Settings

### Logging Configuration

**Configuration:**
- Log level: trace, debug, info, warn, error (default: info)
- Audio log level: Separate log level for audio processing (default: debug)
- JSON output: Enable JSON-formatted logs (default: false)
- Log file: Path to log file (optional)
- Log rotation: Enable log rotation (default: true)
- Max file size: Maximum log file size in MB (default: 100)
- Max files: Maximum number of log files to keep (default: 5)

**Example:**
```json
{
  "logging": {
    "level": "info",
    "audio_level": "debug",
    "json": false,
    "log_file": "/var/log/huginn-muninn.log",
    "rotation": true,
    "max_file_size_mb": 100,
    "max_files": 5
  }
}
```

## Language Settings

### Supported Languages

**Default supported languages:**
- en-US (English - United States)
- de-DE (German - Germany)
- fr-FR (French - France)
- es-ES (Spanish - Spain)
- it-IT (Italian - Italy)
- pt-BR (Portuguese - Brazil)
- ru-RU (Russian - Russia)
- zh-CN (Chinese - China)
- ja-JP (Japanese - Japan)
- ko-KR (Korean - Korea)

**Configuration:**
```json
{
  "language": {
    "default": "en-US",
    "supported": ["en-US", "de-DE", "fr-FR"]
  }
}
```

### User Language Preferences

User-specific language preferences can be set via the LanguageManager API.

## Resource Limits

### Resource Limits Configuration

**Configuration:**
- Max memory: Maximum memory usage in MB (default: 512)
- Max CPU: Maximum CPU usage percentage (default: 80)
- Max execution time: Maximum execution time in milliseconds (default: 30000)
- Max disk: Maximum disk usage in MB (default: 1000)

**Example:**
```json
{
  "resources": {
    "max_memory_mb": 512,
    "max_cpu_percent": 80,
    "max_execution_time_ms": 30000,
    "max_disk_mb": 1000
  }
}
```

## gRPC Settings

### gRPC Server Configuration

**Configuration:**
- Port: gRPC server port (default: 50057 for Huginn, 50058 for Muninn)
- Max message size: Maximum message size in bytes (default: 4MB)
- Keepalive: Keepalive settings

**Example:**
```json
{
  "grpc": {
    "port": 50057,
    "max_message_size": 4194304,
    "keepalive": {
      "time_secs": 30,
      "timeout_secs": 5
    }
  }
}
```

## Retry Settings

### Retry Configuration

**Configuration:**
- Max retries: Maximum number of retry attempts (default: 3)
- Initial delay: Initial retry delay in milliseconds (default: 100)
- Exponential backoff: Enable exponential backoff (default: true)

**Example:**
```json
{
  "retry": {
    "max_retries": 3,
    "initial_delay_ms": 100,
    "exponential_backoff": true
  }
}
```

## Environment Variables

Configuration can also be set via environment variables:

- `HUGINN_LOG_LEVEL`: Log level
- `HUGINN_AUDIO_LOG_LEVEL`: Audio log level
- `HUGINN_GRPC_PORT`: gRPC server port
- `MUNINN_LOG_LEVEL`: Log level
- `MUNINN_AUDIO_LOG_LEVEL`: Audio log level
- `MUNINN_GRPC_PORT`: gRPC server port

## Hot Reload

Configuration files support hot-reload. Changes to configuration files are automatically detected and applied without service restart.
