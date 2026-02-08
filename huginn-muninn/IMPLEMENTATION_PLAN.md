# IMPLEMENTATION_PLAN - Huginn & Muninn (STT/TTS Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Huginn & Muninn - dem STT/TTS Service. Huginn empfängt alle eingehenden Medien (Text, Audio/STT, Bilder, Videos, Video-Streams) und leitet sie an Odin weiter. Muninn konvertiert Text zu Audio (TTS) und gibt Audio aus.

**Mythologische Bedeutung**: Odin's Raben - Huginn und Muninn.

**Programmiersprache**: Rust

**Service-Typ**: Core Service (Teil aller Platformen)

## Entschiedene Konfiguration

### Protobuf-Rust-Tool
✅ **ENTSCHEIDUNG**: prost + tonic
**Begründung**: Async-native, beste tokio-Integration, modern, idiomatisches Rust

### Local-STT-Engine
✅ **ENTSCHEIDUNG**: Whisper.cpp
**Begründung**: OpenAI Whisper, beste Qualität, robuste Performance, bewährt

### Local-TTS-Engine
✅ **ENTSCHEIDUNG**: Coqui TTS
**Begründung**: Natural-sounding, trainierbar, beste Sprachqualität, production-ready

### Audio-Library
✅ **ENTSCHEIDUNG**: cpal
**Begründung**: Cross-platform, low-latency, Rust-native, beste Performance

### Video-Processing-Library
✅ **ENTSCHEIDUNG**: ffmpeg-next
**Begründung**: Umfassend, robuste Format-Support, industry-standard, zuverlässig

---

## Phase 1: Projekt-Setup & Grundstruktur ✅

### 1.1 Projekt-Initialisierung ✅

**Abhängigkeiten**: Keine
**Entscheidung**: prost + tonic ✅

#### 1.1.1 Cargo-Workspace erstellen ✅
- [x] `Cargo.toml` mit Workspace erstellen
  - `huginn/` (STT + Data Forwarding)
  - `muninn/` (TTS)
  - `shared/` (Shared audio utilities)
- [x] Basis-Dependencies für beide Services definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost)
  - Audio (cpal, rodio, hound)
  - Video (ffmpeg-next) - nur Huginn
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
- [x] `.gitignore` erstellt (bereits vorhanden)

#### 1.1.2 Huginn Verzeichnisstruktur erstellen ✅
- [x] `huginn/src/main.rs` erstellen
- [x] `huginn/src/lib.rs` erstellen
- [x] `huginn/src/stt/` für STT-Engine erstellen (SttEngine, SttConfig, SttResult)
- [x] `huginn/src/text_input/` für Text-Input erstellen (bereits vorhanden)
- [x] `huginn/src/media/` für Media-Input (Bild, Video, Video-Stream) erstellen (forwarding/)
- [x] `huginn/src/audio/` für Audio-Processing erstellen (in shared/)
- [x] `huginn/src/video/` für Video-Processing erstellen (TODO: Phase 2)
- [x] `huginn/src/grpc/` für gRPC-Service erstellen
- [x] `huginn/src/utils/` für Utilities erstellen
- [x] `huginn/config/` für Konfigurationsdateien erstellen (bereits vorhanden)
- [x] `huginn/tests/` für Tests erstellen (stt_engine_test.rs, audio_processing_test.rs)

#### 1.1.3 Muninn Verzeichnisstruktur erstellen ✅
- [x] `muninn/src/main.rs` erstellen
- [x] `muninn/src/lib.rs` erstellen
- [x] `muninn/src/tts/` für TTS-Engine erstellen (TtsEngine, TtsConfig, TtsVoice)
- [x] `muninn/src/audio/` für Audio-Processing erstellen (in shared/)
- [x] `muninn/src/grpc/` für gRPC-Service erstellen
- [x] `muninn/src/utils/` für Utilities erstellen
- [x] `muninn/config/` für Konfigurationsdateien erstellen (bereits vorhanden)
- [x] `muninn/tests/` für Tests erstellen (tts_engine_test.rs)

#### 1.1.4 Build-System einrichten ✅
- [x] Build-Scripts in `Cargo.toml` definieren (`[build] build = "build.rs"` in huginn, muninn)
- [x] Code-Generierungs-Pipeline (Protobuf → Rust) – bereits in `huginn/build.rs`, `muninn/build.rs` (tonic_build)
- [x] Cargo-Features definieren: huginn `whisper`, `vosk`; muninn `coqui-tts`, `piper` (FFI-Platzhalter)

### 1.2 Test-Infrastruktur ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung) ✅

#### 1.2.1 Container-Setup für Tests ✅
- [x] `Dockerfile.test` für Test-Umgebung (inkl. `COPY shared` für Path-Dependency)
- [x] Docker Compose für Test-Services (`docker-compose.test.yml`) – Mock-Odin-Service, huginn-muninn-test
- [x] Test-Container-Startup-Scripts (`scripts/run-tests.ps1`, `scripts/run-tests.sh`)
- [x] **WICHTIG**: Tests in Containern – keine lokalen Dependencies

#### 1.2.2 Test-Framework konfigurieren ✅
- [x] Test-Dependencies (tokio-test, mockall, tempfile) in huginn, muninn, shared
- [x] Test-Utilities und Helpers (`tests/utils/test_helpers.rs`, `tests/mocks/`)
- [x] Mock-Setup (Mock-Odin in `tests/mocks/`)
- [ ] Test-Audio-Files für STT-Tests (optional, bei Phase 4.2)
- [ ] Test-Video-Files für Video-Tests (optional, bei Phase 2 Video)

#### 1.2.3 CI/CD-Pipeline ✅
- [x] GitHub Actions Workflow (`.github/workflows/huginn-muninn.yml`)
- [x] Test in Container, Lint (cargo fmt --check, cargo clippy)
- [ ] Code-Coverage (cargo-tarpaulin) optional

### 1.3 Settings-System ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung) ✅

#### 1.3.1 Settings-Schema definieren ✅
- [x] Settings-Struktur entwerfen (JSON-Format)
  - audio_device_configuration ✅
  - quality_settings ✅
  - language_settings ✅
  - voice_settings (TTS) ✅
  - wake_word_settings (optional) ✅
- [x] Rust-Structs in `shared/src/settings/schema.rs` (HuginnSettings, MuninnSettings, AudioDeviceConfig, SttQualitySettings, TtsQualitySettings, VoiceSettings, WakeWordSettings, TtsCacheSettings)

#### 1.3.2 Settings-Validierung ✅
- [x] Tests für Settings-Validierung schreiben (`shared/tests/settings_validator_test.rs`)
- [x] Settings-Validator implementieren (TDD) (`shared/src/settings/validator.rs`)
- [x] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader & Hot-Reload ✅
- [x] Tests für Settings-Loader schreiben (`shared/tests/settings_loader_test.rs`)
- [x] Settings-Loader implementieren (TDD) (`shared/src/settings/loader.rs`)
- [x] Hot-Reload-Mechanismus implementieren (`shared/src/settings/hot_reload.rs` – Arc<RwLock<Settings>>, periodische Prüfung)
- [x] Huginn/Muninn nutzen shared-Settings (Re-Export in `huginn/src/utils/config.rs`, `muninn/src/utils/config.rs`)

---

## Phase 2: Protobuf & gRPC Setup ✅

### 2.1 Protobuf Definitions ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung) ✅

#### 2.1.1 Shared Protobuf-Projekt verwenden ✅
- [x] Protobuf-Definitions in Huginn/Muninn definiert (lokale Proto-Files)
- [x] Build-Scripts konfiguriert (build.rs)

#### 2.1.2 RavenMessage Protocol ✅
- [x] `raven.proto` definieren
  - `RavenMessage` Message
  - `MessageDirection` Enum (INCOMING, OUTGOING)
  - `MessageMetadata` Message
- [x] Code-Generierung konfigurieren (tonic_build)

#### 2.1.3 Huginn Media Protocol ✅
- [x] `huginn.proto` definieren
  - `ForwardTextRequest`, `ForwardImageRequest`, `ForwardVideoRequest` Messages
  - `ForwardVideoStreamChunk` Message (Streaming)
  - `MediaForwardResponse` Message
  - `TranscribeAudioRequest`, `TranscribeAudioResponse` Messages
- [x] Code-Generierung konfigurieren

#### 2.1.4 Muninn TTS Protocol ✅
- [x] `muninn.proto` definieren
  - `TTSRequest` Message (Text + Settings)
  - `TTSResponse` Message (Audio-Data)
  - `TTSStreamChunk` Message (Streaming)
  - `TTSVoice` Enum, `TTSSettings` Message
- [x] Code-Generierung konfigurieren

### 2.2 gRPC Server Implementation ✅

**Abhängigkeiten**: 2.1 (Protobuf Definitions) ✅

#### 2.2.1 gRPC Server Setup ✅
- [x] Tests für gRPC-Server-Setup schreiben (`huginn/tests/grpc_service_test.rs`, `muninn/tests/grpc_service_test.rs`)
- [x] gRPC-Server-Setup implementieren (TDD)
  - tonic-Server konfiguriert
- [x] Tests ausführen und bestehen ✅

#### 2.2.2 Huginn Media Service ✅
- [x] Tests für Huginn-Media-Service schreiben (`huginn/tests/grpc_service_test.rs`)
- [x] `HuginnMediaServiceImpl` implementieren (TDD) - `huginn/src/grpc/server.rs`
  - `ForwardText()` RPC
  - `ForwardImage()` RPC
  - `ForwardVideo()` RPC
  - `ForwardVideoStream()` RPC (Streaming)
  - `TranscribeAudio()` RPC
- [x] Tests ausführen und bestehen ✅

#### 2.2.3 Muninn TTS Service ✅
- [x] Tests für Muninn-TTS-Service schreiben (`muninn/tests/grpc_service_test.rs`)
- [x] `MuninnTTSServiceImpl` implementieren (TDD) - `muninn/src/grpc/server.rs`
  - `GenerateSpeech()` RPC
  - `GenerateSpeechStream()` RPC (Streaming - TODO)
- [x] Tests ausführen und bestehen ✅

---

## Phase 3: Audio-Processing (Shared)

### 3.1 Audio-Device-Management

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Audio-Library

#### 3.1.1 Audio-Device-Manager
- [ ] Tests für Audio-Device-Manager schreiben
- [ ] `AudioDeviceManager` implementieren (TDD)
  - Verfügbare Audio-Devices erkennen
  - Device-Auswahl
  - Device-Änderungen behandeln
- [ ] Tests ausführen und bestehen

### 3.2 Audio-Format-Conversion

**Abhängigkeiten**: 3.1 (Audio-Device-Management)

#### 3.2.1 Audio-Format-Converter
- [ ] Tests für Audio-Format-Converter schreiben
- [ ] `AudioFormatConverter` implementieren (TDD)
  - WAV ↔ MP3 ↔ Opus ↔ FLAC Konvertierung
  - Sample-Rate-Conversion
  - Quality-Trade-offs
- [ ] Tests ausführen und bestehen

### 3.3 Audio-Buffering ✅

**Abhängigkeiten**: 3.2 (Audio-Format-Conversion) ✅

#### 3.3.1 Audio-Buffer ✅
- [x] Tests für Audio-Buffer schreiben (in `shared/src/audio.rs`)
- [x] `AudioBuffer` implementieren (TDD) - bereits in Phase 1 erstellt
  - Audio buffern für Streaming
  - Duration-Berechnung
  - Frame-Counting
- [x] Tests ausführen und bestehen ✅

### 3.4 Noise-Cancellation

**Abhängigkeiten**: 3.1 (Audio-Device-Management)

#### 3.4.1 Noise-Cancellation-Engine
- [ ] Tests für Noise-Cancellation schreiben
- [ ] `NoiseCancellationEngine` implementieren (TDD)
  - Noise-Cancellation-Algorithmen
  - Verschiedene Levels (Low, Medium, High)
  - Konfigurierbar
- [ ] Tests ausführen und bestehen

---

## Phase 4: Huginn - STT Engine ✅ (Teilweise)

### 4.1 STT-Service-Abstraction ✅

**Abhängigkeiten**: 3.1 (Audio-Device-Management)

#### 4.1.1 STT-Service-Integration ✅
- [x] STT-Engine in gRPC-Server integriert (`src/grpc/server.rs`)
  - `SttEngine` als Dependency ✅
  - `transcribe()` Methode ✅ - Verwendet `SttEngine::transcribe()`
  - Audio-Daten-Konvertierung ✅ - Konvertiert gRPC-Request zu `AudioBuffer`
  - RavenMessage-Erstellung ✅ - Erstellt `RavenMessage` mit Transcription-Result
- [x] Tests ausführen und bestehen ✅ (Bereits vorhanden in `tests/grpc_service_test.rs`)
- [ ] `transcribe_stream()` Methode (Streaming) - TODO: Noch nicht implementiert
- [ ] `get_supported_languages()` Methode - TODO: Noch nicht implementiert

### 4.2 Local STT Integration

**Abhängigkeiten**: 4.1 (STT-Service-Abstraction)
**Erforderliche USER-Eingaben**: Local-STT-Engine

#### 4.2.1 Whisper.cpp Integration
- [ ] Tests für Whisper.cpp-Integration schreiben
- [ ] `WhisperSTTService` implementieren (TDD)
  - Whisper.cpp-Bindings
  - Model-Loading
  - Audio-Transkription
- [ ] Tests ausführen und bestehen

#### 4.2.2 Vosk Integration (optional)
- [ ] Tests für Vosk-Integration schreiben
- [ ] `VoskSTTService` implementieren (TDD)
  - Vosk-Bindings
  - Model-Loading
  - Audio-Transkription
- [ ] Tests ausführen und bestehen

### 4.3 Cloud STT Integration (optional)

**Abhängigkeiten**: 4.1 (STT-Service-Abstraction)

#### 4.3.1 Google Speech-to-Text
- [ ] Tests für Google-STT schreiben
- [ ] `GoogleSTTService` implementieren (TDD)
  - Google Speech-to-Text API-Integration
  - API-Key-Management
  - Audio-Transkription
- [ ] Tests ausführen und bestehen

#### 4.3.2 Azure Speech (optional)
- [ ] Tests für Azure-Speech schreiben
- [ ] `AzureSTTService` implementieren (TDD)
  - Azure Speech API-Integration
- [ ] Tests ausführen und bestehen

### 4.4 Real-Time STT-Processing

**Abhängigkeiten**: 4.2 (Local STT Integration), 3.3 (Audio-Buffering)

#### 4.4.1 Real-Time-STT-Processor
- [ ] Tests für Real-Time-STT schreiben
- [ ] `RealTimeSTTProcessor` implementieren (TDD)
  - Audio-Stream-Processing
  - Real-time Transkription
  - Latenz-Optimierung
- [ ] Tests ausführen und bestehen

### 4.5 Wake-Word-Detection (optional)

**Abhängigkeiten**: 4.4 (Real-Time STT-Processing)

#### 4.5.1 Wake-Word-Detector
- [ ] Tests für Wake-Word-Detection schreiben
- [ ] `WakeWordDetector` implementieren (TDD)
  - Wake-Word-Model-Integration (Porcupine, Snowboy)
  - Wake-Word-Detection
  - Custom-Wake-Words
- [ ] Tests ausführen und bestehen

---

## Phase 5: Muninn - TTS Engine ✅ (Teilweise)

### 5.1 TTS-Service-Integration ✅

**Abhängigkeiten**: 2.2.2 (Muninn TTS Service)

#### 5.1.1 TTS-Engine-Integration ✅
- [x] TTS-Engine in gRPC-Server integriert (`muninn/src/grpc/server.rs`)
  - `TtsEngine` als Dependency ✅
  - `synthesize()` Methode ✅ - Verwendet `TtsEngine::synthesize()`
  - Audio-Daten-Konvertierung ✅ - Konvertiert `AudioBuffer` zu gRPC-Response-Bytes
  - Voice-Selection ✅ - Unterstützt Male, Female, Neutral Voices
  - Settings-Support ✅ - Speed, Pitch, Volume, Audio-Format, Sample-Rate
- [x] Tests ausführen und bestehen ✅ (Bereits vorhanden in `tests/grpc_service_test.rs`)
- [ ] `generate_speech_stream()` Methode (Streaming) - TODO: Noch nicht implementiert

---

## Phase 6: Huginn - Text-Input ✅

### 6.1 Text-Input-Handler ✅

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)

#### 6.1.1 Text-Input-Processor ✅
- [x] Tests für Text-Input-Processor schreiben (`tests/text_input_processor_test.rs`)
- [x] `TextInputProcessor` implementieren (TDD) (`src/text_input/processor.rs`)
  - Text-Input vom Frontend empfangen ✅ - Via `process()` Methode
  - RavenMessage erstellen ✅ - `create_raven_message()` mit Message-ID, Timestamp, Metadata
  - Integration in gRPC-Server ✅ - `ForwardText` verwendet `TextInputProcessor`
  - An Odin senden ✅ - TODO: Vollständige Odin-Client-Integration (aktuell nur Logging)
- [x] Tests ausführen und bestehen ✅

---

## Phase 6: Huginn - Media Input (Bild/Video/Video-Stream)

### 6.1 Image-Input-Handler ✅

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)

#### 6.1.1 Image-Input-Processor ✅
- [x] Tests für Image-Input-Processor schreiben (`tests/image_input_processor_test.rs`)
- [x] `ImageInputProcessor` implementieren (TDD) (`src/image_input/processor.rs`)
  - Bild-Dateien empfangen (JPEG, PNG, WebP) ✅ - Unterstützt jpg, jpeg, png, webp
  - Bild-Größen-Limits ✅ - Konfigurierbares Max-Size (Default: 10MB), Validierung
  - Bild-Dimensionen-Validierung ✅ - Prüft width > 0 und height > 0
  - Format-Validierung ✅ - Prüft erlaubte Formate
  - Integration in gRPC-Server ✅ - `ForwardImage` verwendet `ImageInputProcessor`
  - Bild-Daten an Odin weiterleiten ✅ - TODO: Vollständige Odin-Client-Integration (aktuell nur Logging)
- [x] Tests ausführen und bestehen ✅

### 6.2 Video-Input-Handler ✅

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)
**Erforderliche USER-Eingaben**: Video-Processing-Library (ffmpeg-next bereits entschieden)

#### 6.2.1 Video-Input-Processor ✅
- [x] Tests für Video-Input-Processor schreiben (`tests/video_input_processor_test.rs`)
- [x] `VideoInputProcessor` implementieren (TDD) (`src/video_input/processor.rs`)
  - Video-Dateien empfangen (MP4, AVI, MKV, WebM) ✅ - Unterstützt mp4, webm, avi, mkv
  - Video-Größen-Limits ✅ - Konfigurierbares Max-Size (Default: 100MB), Validierung
  - Video-Dauer-Limits ✅ - Konfigurierbares Max-Duration (Default: 1h), Validierung
  - Format-Validierung ✅ - Prüft erlaubte Formate
  - Integration in gRPC-Server ✅ - `ForwardVideo` verwendet `VideoInputProcessor`
  - Video-Daten an Odin weiterleiten ✅ - TODO: Vollständige Odin-Client-Integration (aktuell nur Logging)
- [x] Tests ausführen und bestehen ✅

### 6.3 Video-Stream-Handler ✅

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)

#### 6.3.1 Video-Stream-Processor ✅
- [x] Tests für Video-Stream-Processor schreiben (`tests/video_stream_processor_test.rs`)
- [x] `VideoStreamProcessor` implementieren (TDD) (`src/video_stream/processor.rs`)
  - Live-Camera-Streams empfangen (RTSP, WebRTC) ✅ - Unterstützt mp4, webm, rtsp, webrtc
  - Stream-Chunk-Processing ✅ - Verarbeitet einzelne Chunks mit Index, Format, Last-Flag
  - Stream-Session-Management ✅ - Verwaltet Stream-Sessions pro User/Device, Trackt Statistiken
  - Stream-Unterbrechungen behandeln ✅ - Erkennt fehlende Chunks, markiert Sessions als Interrupted
  - Stream-Statistiken ✅ - Chunk-Count, Total-Bytes, Format-Tracking
  - Integration in gRPC-Server ✅ - `ForwardVideoStream` verwendet `VideoStreamProcessor`
  - Video-Stream-Chunks an Odin weiterleiten ✅ - TODO: Vollständige Odin-Client-Integration (aktuell nur Logging)
- [x] Tests ausführen und bestehen ✅

---

## Phase 7: Muninn - TTS Engine

### 7.1 TTS-Service-Abstraction ✅

**Abhängigkeiten**: 3.1 (Audio-Device-Management)

#### 7.1.1 TTS-Service-Trait ✅
- [x] Tests für TTS-Service-Trait schreiben (`tests/tts_service_trait_test.rs`)
- [x] `TTSService` Trait definieren (`src/tts/service.rs`)
  - `synthesize()` Methode ✅ - Synthesisiert Text zu Audio, gibt `TtsResult` zurück
  - `synthesize_stream()` Methode (Streaming) ✅ - Synthesisiert Text zu Audio-Stream, gibt `mpsc::Receiver<TtsStreamChunk>` zurück
  - `get_supported_voices()` Methode ✅ - Gibt Liste der unterstützten Stimmen zurück
  - `get_supported_languages()` Methode ✅ - Gibt Liste der unterstützten Sprachen zurück
  - `TtsResult` Struktur ✅ - Enthält `AudioBuffer` und `duration_ms`
  - `TtsStreamChunk` Struktur ✅ - Enthält `audio_data` und `is_last` Flag
- [x] Tests ausführen und bestehen ✅

### 7.2 Local TTS Integration

**Abhängigkeiten**: 7.1 (TTS-Service-Abstraction)
**Erforderliche USER-Eingaben**: Local-TTS-Engine

#### 7.2.1 Coqui TTS Integration
- [ ] Tests für Coqui-TTS-Integration schreiben
- [ ] `CoquiTTSService` implementieren (TDD)
  - Coqui-TTS-Bindings
  - Model-Loading
  - Audio-Generierung
- [ ] Tests ausführen und bestehen

#### 7.2.2 Piper Integration (optional)
- [ ] Tests für Piper-Integration schreiben
- [ ] `PiperTTSService` implementieren (TDD)
  - Piper-Bindings
  - Model-Loading
  - Audio-Generierung
- [ ] Tests ausführen und bestehen

### 7.3 Cloud TTS Integration (optional)

**Abhängigkeiten**: 7.1 (TTS-Service-Abstraction)

#### 7.3.1 Google TTS
- [ ] Tests für Google-TTS schreiben
- [ ] `GoogleTTSService` implementieren (TDD)
  - Google TTS API-Integration
  - API-Key-Management
  - Audio-Generierung
- [ ] Tests ausführen und bestehen

#### 7.3.2 Azure TTS (optional)
- [ ] Tests für Azure-TTS schreiben
- [ ] `AzureTTSService` implementieren (TDD)
  - Azure TTS API-Integration
- [ ] Tests ausführen und bestehen

### 7.4 Voice-Management

**Abhängigkeiten**: 7.2 (Local TTS Integration)

#### 7.4.1 Voice-Manager
- [ ] Tests für Voice-Manager schreiben
- [ ] `VoiceManager` implementieren (TDD)
  - Verfügbare Stimmen verwalten
  - Voice-Auswahl
  - Voice-Präferenzen speichern
- [ ] Tests ausführen und bestehen

### 7.5 SSML-Processing

**Abhängigkeiten**: 7.2 (Local TTS Integration)

#### 7.5.1 SSML-Parser
- [ ] Tests für SSML-Parser schreiben
- [ ] `SSMLParser` implementieren (TDD)
  - SSML validieren
  - SSML parsen
  - SSML-Features extrahieren (Prosody, Emphasis, Break)
- [ ] Tests ausführen und bestehen

---

## Phase 8: Caching System ✅

### 8.1 TTS-Phrase-Cache ✅

**Abhängigkeiten**: 7.2 (Local TTS Integration)

#### 8.1.1 TTS-Cache-Manager ✅
- [x] Tests für TTS-Cache schreiben (`tests/tts_cache_test.rs`)
- [x] `TTSCacheManager` implementieren (TDD) (`src/cache/manager.rs`)
  - Häufig verwendete TTS-Phrasen cachen ✅ - LRU-Cache mit konfigurierbarer Größe
  - Cache-Key-Generierung (Text + Voice + Language) ✅ - `generate_key()` Methode
  - Cache-Hit/Miss-Handling ✅ - `get()` und `set()` Methoden mit Statistiken
  - TTL-basierte Expiration ✅ - Automatische Expiration nach TTL, `clean_expired()` Methode
  - LRU-Eviction ✅ - Entfernt älteste Einträge bei Cache-Overflow
  - Cache-Statistiken ✅ - Hits, Misses, Size-Tracking
  - Integration in gRPC-Server ✅ - `generate_speech` verwendet Cache
- [x] Tests ausführen und bestehen ✅

---

## Phase 9: Error Handling & Fallback

### 9.1 Service-Fallback

**Abhängigkeiten**: 4.2 (Local STT), 4.3 (Cloud STT), 7.2 (Local TTS), 7.3 (Cloud TTS)

#### 9.1.1 Fallback-Manager
- [ ] Tests für Fallback-Manager schreiben
- [ ] `FallbackManager` implementieren (TDD)
  - Service-Availability prüfen
  - Automatischer Fallback zu alternativen Services
  - Fallback-Prioritäten (Local → Cloud oder umgekehrt)
- [ ] Tests ausführen und bestehen

### 9.2 Error-Handler ✅

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 9.2.1 Error-Handler ✅
- [x] Tests für Error-Handler schreiben (`tests/error_handler_test.rs`)
- [x] `ErrorHandler` implementieren (TDD) (`src/error_handler/handler.rs`)
  - Audio-Device-Fehler behandeln ✅ - `handle_audio_device_error()` mit FailedPrecondition Status
  - Service-Unavailability behandeln ✅ - `handle_service_unavailable()` mit Unavailable Status
  - Network-Errors behandeln ✅ - `handle_network_error()` mit DeadlineExceeded/Unavailable Status
  - gRPC-Status-Codes ✅ - `handle_grpc_status()` für alle gRPC Status-Codes
  - HuginnError-Enum ✅ - Strukturierte Fehler-Typen (AudioDeviceError, ServiceUnavailable, NetworkError, InvalidInput, InternalError)
  - Error-to-Status-Konvertierung ✅ - `huginn_error_to_status()` Methode
- [x] Tests ausführen und bestehen ✅

### 9.3 Retry-Manager ✅

**Abhängigkeiten**: 9.2 (Error-Handler)

#### 9.3.1 Retry-Manager ✅
- [x] Tests für Retry-Manager schreiben (`tests/retry_manager_test.rs`)
- [x] `RetryManager` implementieren (TDD) (`src/retry/manager.rs`)
  - Exponential-Backoff-Retry ✅ - `calculate_delays()` mit 2^i Multiplier
  - Max-Retry-Count ✅ - Konfigurierbare Max-Retries (Default: 3)
  - Retry-Delay berechnen ✅ - Initial-Delay konfigurierbar (Default: 100ms)
  - Execute-Methode ✅ - Generische `execute()` Methode mit Retry-Logik
- [x] Tests ausführen und bestehen ✅

---

## Phase 10: Language & Multi-Language Support ✅

### 10.1 Language-Detection ✅

**Abhängigkeiten**: 4.4 (Real-Time STT-Processing)

#### 10.1.1 Language-Detector ✅
- [x] Tests für Language-Detection schreiben (`shared/tests/language_detector_test.rs`)
- [x] `LanguageDetector` implementieren (TDD) (`shared/src/language/detector.rs`)
  - Automatische Language-Detection ✅ - Heuristische Pattern-Matching-basierte Erkennung (DE, FR, ES, EN)
  - Confidence-Score ✅ - Berechnet Confidence basierend auf Pattern-Matches
  - Fallback zu Default-Language ✅ - Verwendet "en-US" als Default bei niedriger Confidence oder leerem Text
  - Konfigurierbare Default-Language ✅ - `set_default_language()` Methode
- [x] Tests ausführen und bestehen ✅

### 10.2 Multi-Language-Manager ✅

**Abhängigkeiten**: 10.1 (Language-Detection), 4.1 (STT-Service), 7.1 (TTS-Service)

#### 10.2.1 Language-Manager ✅
- [x] Tests für Language-Manager schreiben (`shared/src/language/manager.rs` - inline tests)
- [x] `LanguageManager` implementieren (TDD) (`shared/src/language/manager.rs`)
  - Unterstützte Sprachen verwalten ✅ - 10 Standard-Sprachen (EN, DE, FR, ES, IT, PT, RU, ZH, JA, KO)
  - Language-Auswahl ✅ - `is_supported()`, `add_language()`, `remove_language()` Methoden
  - Language-Präferenzen speichern ✅ - User-spezifische Präferenzen (`get_user_language()`, `set_user_language()`)
  - Default-Language-Management ✅ - Konfigurierbare Default-Language
- [x] Tests ausführen und bestehen ✅

---

## Phase 11: Performance Optimization

### 11.1 Streaming-Optimization

**Abhängigkeiten**: 4.4 (Real-Time STT), 7.3 (TTS Integration)

#### 11.1.1 Streaming-Optimizer
- [ ] Tests für Streaming-Optimization schreiben
- [ ] Streaming optimieren
  - Audio-Stream-Buffering
  - Video-Stream-Buffering
  - Latenz-Minimierung
- [ ] Tests ausführen und bestehen

### 11.2 Performance-Benchmarks

**Abhängigkeiten**: 4.4 (Real-Time STT), 7.2 (Local TTS)

#### 11.2.1 Performance-Benchmarking
- [ ] Performance-Benchmarks schreiben
  - STT-Latency (< 500ms für kurze Phrasen)
  - TTS-Generation (< 1s für Standard-Phrasen)
- [ ] Benchmarks ausführen und Ziele erreichen

---

## Phase 12: Monitoring & Logging

### 12.1 Structured Logging ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 12.1.1 Logging Setup ✅
- [x] Structured-Logging konfigurieren (tracing) ✅ - `shared/src/logging/setup.rs` mit `setup_logging()` Funktion
- [x] Audio-specific Log-Levels ✅ - Separate Log-Levels für `huginn::audio`, `muninn::audio`, `shared::audio` Module
- [x] Log-Rotation konfigurieren ✅ - `tracing-appender` mit `rolling::daily()` Support, konfigurierbare Max-File-Size und Max-Files
- [x] LoggingConfig ✅ - Strukturierte Konfiguration (`LoggingConfig`) mit Level, Audio-Level, JSON-Output, Log-File-Path, Rotation-Settings
- [x] JSON-Output-Support ✅ - Optional JSON-Format für strukturierte Logs

### 12.2 Performance-Monitoring ✅

**Abhängigkeiten**: 11.2 (Performance-Benchmarks)

#### 12.2.1 Metrics-Collector ✅
- [x] Tests für Metrics-Collector schreiben ✅ - Inline-Tests in `shared/src/metrics/collector.rs`
- [x] `MetricsCollector` implementieren (TDD) (`shared/src/metrics/collector.rs`)
  - STT-Performance tracken ✅ - `record_stt_latency()`, `record_stt_error()`, SttMetrics-Snapshot
  - TTS-Performance tracken ✅ - `record_tts_latency()`, `record_tts_error()`, TtsMetrics-Snapshot
  - Audio-Processing-Performance tracken ✅ - `record_audio_processing_latency()`, AudioProcessingMetrics
  - Video-Processing-Performance tracken ✅ - `record_video_processing_latency()`, VideoProcessingMetrics
  - Snapshot & Reset ✅ - `snapshot()`, `reset()`, `stt_avg_latency_ms()`, `tts_avg_latency_ms()`
  - Thread-safe ✅ - AtomicU64 für alle Zähler/Latenzen, Clone für Arc-Sharing
- [x] Tests ausführen und bestehen ✅

---

## Phase 13: Documentation ✅

### 13.1 API Documentation ✅

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 13.1.1 gRPC Service Documentation ✅
- [x] gRPC-Service-Documentation erstellen ✅ - `docs/API.md` erstellt
- [x] Huginn-Media-Service-API dokumentieren ✅ - Alle RPCs dokumentiert (TranscribeAudio, ForwardText, ForwardImage, ForwardVideo, ForwardVideoStream)
- [x] Muninn-TTS-Service-API dokumentieren ✅ - Alle RPCs dokumentiert (GenerateSpeech, GenerateSpeechStream)
- [x] RavenMessage-Protocol dokumentieren ✅ - RavenMessage, MessageDirection, MessageMetadata vollständig dokumentiert
- [x] Error-Handling dokumentieren ✅ - gRPC Error-Codes dokumentiert
- [x] Format-Support dokumentieren ✅ - Audio-, Image-, Video-Formate dokumentiert

### 13.2 Configuration-Documentation ✅

**Abhängigkeiten**: 1.3 (Settings-System)

#### 13.2.1 Settings-Guide ✅
- [x] Settings-Guide erstellen ✅ - `docs/CONFIGURATION.md` erstellt
- [x] Audio-Device-Configuration dokumentieren ✅ - Input/Output-Device-Konfiguration dokumentiert
- [x] Quality-Settings dokumentieren ✅ - STT/TTS Quality-Settings dokumentiert
- [x] Voice-Settings dokumentieren ✅ - Voice-Selection und Voice-Parameters dokumentiert
- [x] Cache-Settings dokumentieren ✅ - TTS-Cache-Konfiguration dokumentiert
- [x] Logging-Settings dokumentieren ✅ - Logging-Konfiguration dokumentiert
- [x] Language-Settings dokumentieren ✅ - Supported-Languages und User-Preferences dokumentiert
- [x] Resource-Limits dokumentieren ✅ - Resource-Limits-Konfiguration dokumentiert
- [x] gRPC-Settings dokumentieren ✅ - gRPC-Server-Konfiguration dokumentiert
- [x] Retry-Settings dokumentieren ✅ - Retry-Konfiguration dokumentiert
- [x] Environment-Variables dokumentieren ✅ - Environment-Variable-Support dokumentiert
- [x] Hot-Reload dokumentieren ✅ - Hot-Reload-Funktionalität dokumentiert

---

## Phase 14: Testing & Quality Assurance

### 14.1 Integration Testing ✅

**Abhängigkeiten**: Alle vorherigen Phasen

#### 14.1.1 End-to-End Tests ✅
- [x] E2E-Tests für Audio-Workflows schreiben ✅
  - STT-Workflow: Audio → Text → RavenMessage ✅ - `huginn/tests/e2e_workflow_test.rs` (e2e_stt_workflow_audio_to_text_to_raven_message)
  - TTS-Workflow: RavenMessage/Text → Audio → Output ✅ - `muninn/tests/e2e_workflow_test.rs` (e2e_tts_workflow_text_to_audio_output, e2e_tts_workflow_empty_text_fails)
  - Media-Workflow: Text/Bild/Video → Huginn ✅ - `huginn/tests/e2e_workflow_test.rs` (e2e_media_workflow_text_forward, e2e_media_workflow_image_forward, e2e_media_workflow_video_forward)
- [x] E2E-Tests ausführen und bestehen ✅

### 14.2 Performance Testing

**Abhängigkeiten**: 11.1 (Streaming-Optimization)

#### 14.2.1 Performance Test Suite
- [ ] Performance-Tests ausführen
  - STT-Latency-Tests (< 500ms)
  - TTS-Generation-Tests (< 1s)
  - Audio-Processing-Performance-Tests
- [ ] Performance-Tests bestehen

---

## Verbleibende Punkte (Übersicht)

- [ ] **Phase 14.2.1** Performance Test Suite (STT-Latency < 500ms, TTS < 1s, Audio-Processing)
- **Optional / bei Bedarf (laut Master-Plan):** Phase 4.2+ (Whisper/Vosk FFI), 5.2+/7.2+ (Coqui/Piper FFI), Phase 9.1 (Service-Fallback), Phase 11 (Streaming/Performance), Cloud-Integration

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 14
**Gesamtanzahl Schritte**: ~270+

**Kritische Abhängigkeiten**:
1. Protobuf-Rust-Tool (prost + tonic empfohlen)
2. Local-STT-Engine (Whisper.cpp empfohlen)
3. Local-TTS-Engine (Coqui TTS empfohlen)
4. Audio-Library (cpal empfohlen)
5. Video-Processing-Library (ffmpeg-next empfohlen)

**Offene Fragen für USER**:
1. Protobuf-Rust-Tool (prost + tonic, rust-protobuf)
2. Local-STT-Engine (Whisper.cpp, Vosk, Coqui STT)
3. Local-TTS-Engine (Coqui TTS, Piper, Espeak)
4. Audio-Library (cpal, rodio, portaudio-rs)
5. Video-Processing-Library (ffmpeg-next, gstreamer-rs, Eigene)

**Hinweise**:
- Alle Schritte folgen TDD
- Alle Tests in Containern
- Huginn: STT + Data Forwarding (Text, Bild, Video, Video-Stream)
- Muninn: TTS
- Real-time Processing für niedrige Latenz
- Lokale Verarbeitung bevorzugt für Privacy
