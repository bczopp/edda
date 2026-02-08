# Huginn & Muninn API Documentation

## Overview

Huginn & Muninn provide gRPC services for Speech-to-Text (STT), Text-to-Speech (TTS), and media data forwarding. This document describes the gRPC API interfaces.

## Huginn Media Service

The Huginn Media Service handles incoming media (text, audio, images, videos, video streams) and forwards them to Odin.

### Service: `HuginnMediaService`

#### RPC: `TranscribeAudio`

Transcribes audio data to text using the STT engine.

**Request:**
```protobuf
message TranscribeAudioRequest {
  bytes audio_data = 1;        // Raw audio data
  string language = 2;         // Language code (e.g., "en-US")
  AudioFormat audio_format = 3; // Audio format specification
  string user_id = 4;          // User identifier
  string device_id = 5;        // Device identifier
}
```

**Response:**
```protobuf
message TranscribeAudioResponse {
  string transcription = 1;   // Transcribed text
  float confidence = 2;         // Confidence score (0.0-1.0)
  int32 duration_ms = 3;       // Audio duration in milliseconds
  string message_id = 4;       // RavenMessage ID
}
```

#### RPC: `ForwardText`

Forwards text input to Odin.

**Request:**
```protobuf
message ForwardTextRequest {
  string text = 1;             // Text content
  string user_id = 2;          // User identifier
  string device_id = 3;         // Device identifier
}
```

**Response:**
```protobuf
message MediaForwardResponse {
  bool success = 1;            // Success status
  string message_id = 2;       // RavenMessage ID
  string error_message = 3;    // Error message if failed
}
```

#### RPC: `ForwardImage`

Forwards image data to Odin.

**Request:**
```protobuf
message ForwardImageRequest {
  bytes image_data = 1;        // Image data (JPEG, PNG, WebP)
  string format = 2;           // Image format ("jpeg", "png", "webp")
  int32 width = 3;             // Image width in pixels
  int32 height = 4;            // Image height in pixels
  string user_id = 5;          // User identifier
  string device_id = 6;        // Device identifier
}
```

**Response:**
```protobuf
message MediaForwardResponse {
  bool success = 1;
  string message_id = 2;
  string error_message = 3;
}
```

#### RPC: `ForwardVideo`

Forwards video data to Odin.

**Request:**
```protobuf
message ForwardVideoRequest {
  bytes video_data = 1;        // Video data (MP4, AVI, MKV, WebM)
  string format = 2;           // Video format
  int32 duration_seconds = 3;  // Video duration in seconds
  string user_id = 4;          // User identifier
  string device_id = 5;        // Device identifier
}
```

**Response:**
```protobuf
message MediaForwardResponse {
  bool success = 1;
  string message_id = 2;
  string error_message = 3;
}
```

#### RPC: `ForwardVideoStream`

Forwards video stream chunks to Odin.

**Request:**
```protobuf
message ForwardVideoStreamRequest {
  bytes chunk_data = 1;         // Video stream chunk data
  int32 chunk_index = 2;       // Chunk index (0-based)
  string format = 3;            // Stream format ("mp4", "webm", "rtsp", "webrtc")
  bool is_last = 4;             // True if this is the last chunk
  string session_id = 5;        // Stream session identifier
  string user_id = 6;           // User identifier
  string device_id = 7;         // Device identifier
}
```

**Response:**
```protobuf
message MediaForwardResponse {
  bool success = 1;
  string message_id = 2;
  string error_message = 3;
}
```

## Muninn TTS Service

The Muninn TTS Service converts text to speech audio.

### Service: `MuninnTtsService`

#### RPC: `GenerateSpeech`

Generates speech audio from text using the TTS engine.

**Request:**
```protobuf
message TtsRequest {
  string text = 1;              // Text to synthesize
  string language = 2;          // Language code (e.g., "en-US")
  TtsVoice voice = 3;           // Voice selection (Male, Female, Neutral)
  TtsSettings settings = 4;     // TTS settings (speed, pitch, volume, etc.)
  string user_id = 5;          // User identifier
}
```

**Response:**
```protobuf
message TtsResponse {
  bytes audio_data = 1;         // Generated audio data
  int32 duration_ms = 2;       // Audio duration in milliseconds
  string format = 3;            // Audio format ("wav", "mp3", etc.)
  int32 sample_rate = 4;        // Sample rate in Hz
}
```

#### RPC: `GenerateSpeechStream`

Generates speech audio stream from text (streaming).

**Request:**
```protobuf
message TtsRequest {
  string text = 1;
  string language = 2;
  TtsVoice voice = 3;
  TtsSettings settings = 4;
  string user_id = 5;
}
```

**Response (Stream):**
```protobuf
message TtsStreamChunk {
  bytes audio_data = 1;         // Audio chunk data
  bool is_last = 2;             // True if this is the last chunk
  int32 chunk_index = 3;        // Chunk index (0-based)
}
```

## RavenMessage Protocol

The RavenMessage protocol is used for communication between Huginn/Muninn and Odin.

### Message: `RavenMessage`

```protobuf
message RavenMessage {
  string message_id = 1;        // Unique message identifier
  MessageDirection direction = 2; // Incoming (user → Odin) or Outgoing (Odin → user)
  string content = 3;          // Message content (text, transcription, etc.)
  MessageMetadata metadata = 4; // Additional metadata
  int64 timestamp = 5;          // Unix timestamp
}
```

### Enum: `MessageDirection`

- `INCOMING = 0`: From user to Odin (via Huginn)
- `OUTGOING = 1`: From Odin to user (via Muninn)

### Message: `MessageMetadata`

```protobuf
message MessageMetadata {
  string user_id = 1;           // User identifier
  string device_id = 2;          // Device identifier
  string language = 3;           // Language code
  float confidence = 4;         // Confidence score (0.0-1.0)
  int32 duration_ms = 5;        // Duration in milliseconds (for audio)
  map<string, string> custom = 6; // Custom metadata fields
}
```

## Error Handling

All gRPC methods may return the following error codes:

- `INVALID_ARGUMENT`: Invalid request parameters
- `FAILED_PRECONDITION`: Audio device error or precondition failed
- `UNAVAILABLE`: Service unavailable or network error
- `DEADLINE_EXCEEDED`: Request timeout
- `INTERNAL`: Internal server error

## Language Codes

Supported language codes follow the BCP 47 format (e.g., "en-US", "de-DE", "fr-FR").

## Audio Formats

Supported audio formats:
- **Input**: WAV, MP3, FLAC, OGG
- **Output**: WAV, MP3

## Image Formats

Supported image formats:
- JPEG, PNG, WebP

## Video Formats

Supported video formats:
- MP4, AVI, MKV, WebM

## Video Stream Formats

Supported video stream formats:
- MP4, WebM, RTSP, WebRTC
