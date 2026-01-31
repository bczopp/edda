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

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Protobuf-Rust-Tool

#### 1.1.1 Cargo-Workspace erstellen
- [ ] `Cargo.toml` mit Workspace erstellen
  - `huginn/` (STT + Data Forwarding)
  - `muninn/` (TTS)
  - `shared/` (Shared audio utilities)
- [ ] Basis-Dependencies für beide Services definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost)
  - Audio (cpal, rodio, oder portaudio)
  - Video (ffmpeg-next, gstreamer, etc.) - nur Huginn
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
- [ ] `.gitignore` erstellen

#### 1.1.2 Huginn Verzeichnisstruktur erstellen
- [ ] `huginn/src/main.rs` erstellen
- [ ] `huginn/src/lib.rs` erstellen
- [ ] `huginn/src/stt/` für STT-Engine erstellen
- [ ] `huginn/src/text_input/` für Text-Input erstellen
- [ ] `huginn/src/media/` für Media-Input (Bild, Video, Video-Stream) erstellen
- [ ] `huginn/src/audio/` für Audio-Processing erstellen
- [ ] `huginn/src/video/` für Video-Processing erstellen
- [ ] `huginn/src/grpc/` für gRPC-Service erstellen
- [ ] `huginn/src/utils/` für Utilities erstellen
- [ ] `huginn/config/` für Konfigurationsdateien erstellen
- [ ] `huginn/tests/` für Tests erstellen

#### 1.1.3 Muninn Verzeichnisstruktur erstellen
- [ ] `muninn/src/main.rs` erstellen
- [ ] `muninn/src/lib.rs` erstellen
- [ ] `muninn/src/tts/` für TTS-Engine erstellen
- [ ] `muninn/src/audio/` für Audio-Processing erstellen
- [ ] `muninn/src/grpc/` für gRPC-Service erstellen
- [ ] `muninn/src/utils/` für Utilities erstellen
- [ ] `muninn/config/` für Konfigurationsdateien erstellen
- [ ] `muninn/tests/` für Tests erstellen

#### 1.1.4 Build-System einrichten
- [ ] Build-Scripts in `Cargo.toml` definieren
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren (z.B. `whisper`, `vosk`, `coqui-tts`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [ ] `Dockerfile` für Test-Umgebung erstellen
- [ ] Docker Compose für Test-Services konfigurieren
  - Mock-Odin-Service
  - Mock-STT-Service
  - Mock-TTS-Service
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (tokio-test, mockall, etc.)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Services
- [ ] Test-Audio-Files erstellen (für STT-Tests)
- [ ] Test-Video-Files erstellen (für Video-Tests)

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin)
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren
- [ ] Settings-Struktur entwerfen (JSON-Format)
  - audio_device_configuration
  - quality_settings
  - language_settings
  - voice_settings (TTS)
  - wake_word_settings (optional)

#### 1.3.2 Settings-Validierung
- [ ] Tests für Settings-Validierung schreiben
- [ ] Rust-Structs für Settings definieren
- [ ] Settings-Validator implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader & Hot-Reload
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
- [ ] Hot-Reload-Mechanismus implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 2: Protobuf & gRPC Setup

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Huginn/Muninn als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 2.1.2 RavenMessage Protocol
- [ ] `RavenMessage.proto` definieren (siehe README.md)
  - `RavenMessage` Message
  - `MessageDirection` Enum (INCOMING, OUTGOING)
  - `MessageMetadata` Message
- [ ] Code-Generierung konfigurieren

#### 2.1.3 Huginn Media Protocol
- [ ] `HuginnMediaService.proto` definieren
  - `ForwardTextRequest` Message
  - `ForwardImageRequest` Message
  - `ForwardVideoRequest` Message
  - `ForwardVideoStreamChunk` Message (Streaming)
  - `MediaForwardResponse` Message
- [ ] Code-Generierung konfigurieren

#### 2.1.4 Muninn TTS Protocol
- [ ] `MuninnTTSService.proto` definieren
  - `TTSRequest` Message (Text + Settings)
  - `TTSResponse` Message (Audio-Data)
  - `TTSStreamChunk` Message (Streaming)
- [ ] Code-Generierung konfigurieren

### 2.2 gRPC Server Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Server Setup
- [ ] Tests für gRPC-Server-Setup schreiben
- [ ] gRPC-Server-Setup implementieren (TDD)
  - tonic-Server konfigurieren
  - Health-Check-Service
- [ ] Tests ausführen und bestehen

#### 2.2.2 Huginn Media Service
- [ ] Tests für Huginn-Media-Service schreiben
- [ ] `HuginnMediaServiceImpl` implementieren (TDD)
  - `ForwardText()` RPC
  - `ForwardImage()` RPC
  - `ForwardVideo()` RPC
  - `ForwardVideoStream()` RPC (Streaming)
- [ ] Tests ausführen und bestehen

#### 2.2.3 Muninn TTS Service
- [ ] Tests für Muninn-TTS-Service schreiben
- [ ] `MuninnTTSServiceImpl` implementieren (TDD)
  - `GenerateSpeech()` RPC
  - `GenerateSpeechStream()` RPC (Streaming)
- [ ] Tests ausführen und bestehen

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

### 3.3 Audio-Buffering

**Abhängigkeiten**: 3.2 (Audio-Format-Conversion)

#### 3.3.1 Audio-Buffer
- [ ] Tests für Audio-Buffer schreiben
- [ ] `AudioBuffer` implementieren (TDD)
  - Audio buffern für Streaming
  - Ring-Buffer-Implementation
  - Buffer-Overflow-Handling
- [ ] Tests ausführen und bestehen

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

## Phase 4: Huginn - STT Engine

### 4.1 STT-Service-Abstraction

**Abhängigkeiten**: 3.1 (Audio-Device-Management)

#### 4.1.1 STT-Service-Trait
- [ ] Tests für STT-Service-Trait schreiben
- [ ] `STTService` Trait definieren
  - `transcribe()` Methode
  - `transcribe_stream()` Methode (Streaming)
  - `get_supported_languages()` Methode
- [ ] Tests ausführen und bestehen

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

## Phase 5: Huginn - Text-Input

### 5.1 Text-Input-Handler

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)

#### 5.1.1 Text-Input-Processor
- [ ] Tests für Text-Input-Processor schreiben
- [ ] `TextInputProcessor` implementieren (TDD)
  - Text-Input vom Frontend empfangen
  - RavenMessage erstellen
  - An Odin senden
- [ ] Tests ausführen und bestehen

---

## Phase 6: Huginn - Media Input (Bild/Video/Video-Stream)

### 6.1 Image-Input-Handler

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)

#### 6.1.1 Image-Input-Processor
- [ ] Tests für Image-Input-Processor schreiben
- [ ] `ImageInputProcessor` implementieren (TDD)
  - Bild-Dateien empfangen (JPEG, PNG, WebP)
  - Bild-Größen-Limits
  - Bild-Daten an Odin weiterleiten
- [ ] Tests ausführen und bestehen

### 6.2 Video-Input-Handler

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)
**Erforderliche USER-Eingaben**: Video-Processing-Library

#### 6.2.1 Video-Input-Processor
- [ ] Tests für Video-Input-Processor schreiben
- [ ] `VideoInputProcessor` implementieren (TDD)
  - Video-Dateien empfangen (MP4, AVI, MKV)
  - Video-Größen-Limits
  - Video-Daten an Odin weiterleiten
- [ ] Tests ausführen und bestehen

### 6.3 Video-Stream-Handler

**Abhängigkeiten**: 2.2.2 (Huginn Media Service)

#### 6.3.1 Video-Stream-Processor
- [ ] Tests für Video-Stream-Processor schreiben
- [ ] `VideoStreamProcessor` implementieren (TDD)
  - Live-Camera-Streams empfangen (RTSP, WebRTC)
  - Stream-Chunk-Processing
  - Stream-Unterbrechungen behandeln
  - Video-Stream-Chunks an Odin weiterleiten
- [ ] Tests ausführen und bestehen

---

## Phase 7: Muninn - TTS Engine

### 7.1 TTS-Service-Abstraction

**Abhängigkeiten**: 3.1 (Audio-Device-Management)

#### 7.1.1 TTS-Service-Trait
- [ ] Tests für TTS-Service-Trait schreiben
- [ ] `TTSService` Trait definieren
  - `synthesize()` Methode
  - `synthesize_stream()` Methode (Streaming)
  - `get_supported_voices()` Methode
  - `get_supported_languages()` Methode
- [ ] Tests ausführen und bestehen

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

## Phase 8: Caching System

### 8.1 TTS-Phrase-Cache

**Abhängigkeiten**: 7.2 (Local TTS Integration)

#### 8.1.1 TTS-Cache-Manager
- [ ] Tests für TTS-Cache schreiben
- [ ] `TTSCacheManager` implementieren (TDD)
  - Häufig verwendete TTS-Phrasen cachen
  - Cache-Key-Generierung (Text + Voice + Language)
  - Cache-Hit/Miss-Handling
  - TTL-basierte Expiration
- [ ] Tests ausführen und bestehen

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

### 9.2 Error-Handler

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 9.2.1 Error-Handler
- [ ] Tests für Error-Handler schreiben
- [ ] `ErrorHandler` implementieren (TDD)
  - Audio-Device-Fehler behandeln
  - Service-Unavailability behandeln
  - Network-Errors behandeln
  - gRPC-Status-Codes
- [ ] Tests ausführen und bestehen

### 9.3 Retry-Manager

**Abhängigkeiten**: 9.2 (Error-Handler)

#### 9.3.1 Retry-Manager
- [ ] Tests für Retry-Manager schreiben
- [ ] `RetryManager` implementieren (TDD)
  - Exponential-Backoff-Retry
  - Max-Retry-Count
  - Retry-Delay berechnen
- [ ] Tests ausführen und bestehen

---

## Phase 10: Language & Multi-Language Support

### 10.1 Language-Detection

**Abhängigkeiten**: 4.4 (Real-Time STT-Processing)

#### 10.1.1 Language-Detector
- [ ] Tests für Language-Detection schreiben
- [ ] `LanguageDetector` implementieren (TDD)
  - Automatische Language-Detection
  - Confidence-Score
  - Fallback zu Default-Language
- [ ] Tests ausführen und bestehen

### 10.2 Multi-Language-Manager

**Abhängigkeiten**: 10.1 (Language-Detection), 4.1 (STT-Service), 7.1 (TTS-Service)

#### 10.2.1 Language-Manager
- [ ] Tests für Language-Manager schreiben
- [ ] `LanguageManager` implementieren (TDD)
  - Unterstützte Sprachen verwalten
  - Language-Auswahl
  - Language-Präferenzen speichern
- [ ] Tests ausführen und bestehen

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

### 12.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 12.1.1 Logging Setup
- [ ] Structured-Logging konfigurieren (tracing)
- [ ] Audio-specific Log-Levels
- [ ] Log-Rotation konfigurieren

### 12.2 Performance-Monitoring

**Abhängigkeiten**: 11.2 (Performance-Benchmarks)

#### 12.2.1 Metrics-Collector
- [ ] Tests für Metrics-Collector schreiben
- [ ] `MetricsCollector` implementieren (TDD)
  - STT-Performance tracken
  - TTS-Performance tracken
  - Audio-Processing-Performance tracken
  - Video-Processing-Performance tracken
- [ ] Tests ausführen und bestehen

---

## Phase 13: Documentation

### 13.1 API Documentation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 13.1.1 gRPC Service Documentation
- [ ] gRPC-Service-Documentation erstellen
- [ ] Huginn-Media-Service-API dokumentieren
- [ ] Muninn-TTS-Service-API dokumentieren
- [ ] RavenMessage-Protocol dokumentieren

### 13.2 Configuration-Documentation

**Abhängigkeiten**: 1.3 (Settings-System)

#### 13.2.1 Settings-Guide
- [ ] Settings-Guide erstellen
- [ ] Audio-Device-Configuration dokumentieren
- [ ] Quality-Settings dokumentieren
- [ ] Voice-Settings dokumentieren

---

## Phase 14: Testing & Quality Assurance

### 14.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 14.1.1 End-to-End Tests
- [ ] E2E-Tests für Audio-Workflows schreiben
  - STT-Workflow: Audio → Text → RavenMessage → Odin
  - TTS-Workflow: RavenMessage → Text → Audio → Output
  - Media-Workflow: Bild/Video → Huginn → Odin
- [ ] E2E-Tests ausführen und bestehen

### 14.2 Performance Testing

**Abhängigkeiten**: 11.1 (Streaming-Optimization)

#### 14.2.1 Performance Test Suite
- [ ] Performance-Tests ausführen
  - STT-Latency-Tests (< 500ms)
  - TTS-Generation-Tests (< 1s)
  - Audio-Processing-Performance-Tests
- [ ] Performance-Tests bestehen

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
