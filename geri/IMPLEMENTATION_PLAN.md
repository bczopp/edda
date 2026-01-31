# IMPLEMENTATION_PLAN - Geri (LLM Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Geri - dem LLM (Large Language Model) Service. Geri verarbeitet Prompts (mit oder ohne RAG-Context), unterstützt Vision-Models für Bild/Video-Interpretation, implementiert Multi-Faktor-Bewertung für Model-Auswahl und Load-Balancing.

**Mythologische Bedeutung**: Geri ist einer von Odins Wölfen.

**Programmiersprache**: Rust

**Service-Typ**: Core Service (Teil aller Platformen)

## Entschiedene Konfiguration

### Protobuf-Rust-Tool
✅ **ENTSCHEIDUNG**: prost + tonic
**Begründung**: Moderne Rust-Lösung, async-native, beste gRPC-Integration, idiomatisches Rust

### Default-Local-LLM
✅ **ENTSCHEIDUNG**: Llama 3 8B
**Begründung**: Beste Balance zwischen Größe und Qualität, bewährt, robuste Performance, läuft auf den meisten Geräten

### Local-LLM-Provider
✅ **ENTSCHEIDUNG**: llama.cpp (direkt) + BitNet.cpp (1-bit Modelle)
**Begründung**: 
- llama.cpp = minimaler Resource-Impact, direkte Einbindung als Library
- BitNet.cpp = extreme Effizienz für 1-bit Modelle (90% weniger RAM, 5-10x schneller)
- Beide Libraries unterstützen für maximale Flexibilität (Standard + Ultra-Efficient)

### Vision-Model-Default
✅ **ENTSCHEIDUNG**: GPT-4V
**Begründung**: Beste Qualität für Bild/Video-Interpretation, production-ready, zuverlässig

### Model-Registry-Storage
✅ **ENTSCHEIDUNG**: PostgreSQL
**Begründung**: Strukturiert, persistent, robuste Queries, konsistent mit anderen Services

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Protobuf-Rust-Tool, Default-Local-LLM, Local-LLM-Provider

#### 1.1.1 Cargo-Projekt erstellen
- [x] `Cargo.toml` erstellen
- [x] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost) - oder rust-protobuf
  - HTTP-Client (reqwest für Cloud-Provider)
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
- [x] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [x] `geri/src/main.rs` erstellen
- [x] `geri/src/lib.rs` erstellen
- [ ] `geri/src/model/` für Model-Management erstellen
- [ ] `geri/src/providers/` für Provider-Integration erstellen (vorhanden: llm/, evaluation/, vision/)
- [ ] `geri/src/prompt/` für Prompt-Processing erstellen
- [ ] `geri/src/cost/` für Cost-Management erstellen
- [ ] `geri/src/selection/` für Model-Selection erstellen
- [x] `geri/src/vision/` für Vision-Model-Support erstellen
- [x] `geri/src/grpc/` für gRPC-Service erstellen
- [x] `geri/src/utils/` für Utilities erstellen
- [x] `geri/config/` für Konfigurationsdateien erstellen
- [x] `geri/tests/` für Tests erstellen

#### 1.1.3 Build-System einrichten
- [x] Build-Scripts in `Cargo.toml` definieren
- [x] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren (z.B. `ollama`, `openai`, `anthropic`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
  - Mock-llama.cpp-Service
  - Mock-BitNet.cpp-Service (1-bit Models)
  - Mock-OpenAI-API
  - Mock-Anthropic-API
  - Mock-Odin-Service
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (tokio-test, mockall, wiremock, etc.)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Provider-APIs
- [ ] Test-Data-Generators für Prompts erstellen

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin)
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren
- [ ] Settings-Struktur entwerfen (JSON-Format, siehe README.md)
  - models (local, cloud)
  - default_model
  - vision_models
  - fallback
  - performance

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
- [ ] Geri als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 2.1.2 Wolf Protocol (WolfRequest/WolfResponse)
- [ ] `Wolf.proto` verwenden (falls nicht vorhanden, erstellen)
  - `WolfRequest` Message (modelType: LLM, prompt, ragContext)
  - `WolfResponse` Message (generatedText, tokens, latency, cost)
- [ ] Code-Generierung konfigurieren

#### 2.1.3 Vision Protocol
- [ ] `Vision.proto` definieren
  - `ImageAnalysisRequest` Message
  - `ImageAnalysisResponse` Message
  - `VideoAnalysisRequest` Message
  - `VideoAnalysisResponse` Message
  - `VideoStreamChunk` Message
  - `VideoAnalysisChunk` Message
- [ ] Code-Generierung konfigurieren

### 2.2 gRPC Server Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Server Setup
- [ ] Tests für gRPC-Server-Setup schreiben
- [ ] gRPC-Server-Setup implementieren (TDD)
  - tonic-Server konfigurieren
  - Health-Check-Service
- [ ] Tests ausführen und bestehen

#### 2.2.2 Wolf Service (LLM-Service)
- [ ] Tests für Wolf-Service schreiben
- [ ] `WolfServiceImpl` implementieren (TDD)
  - `ProcessPrompt()` RPC → WolfRequest verarbeiten
  - WolfResponse generieren
- [ ] Tests ausführen und bestehen

#### 2.2.3 Vision Service
- [ ] Tests für Vision-Service schreiben
- [ ] `VisionServiceImpl` implementieren (TDD)
  - `AnalyzeImage()` RPC → ImageAnalysisRequest verarbeiten
  - `AnalyzeVideo()` RPC → VideoAnalysisRequest verarbeiten
  - `AnalyzeVideoStream()` RPC → VideoStreamChunk verarbeiten (Streaming)
- [ ] Tests ausführen und bestehen

---

## Phase 3: Provider Abstraction Layer

### 3.1 Provider-Interface

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 3.1.1 LLM-Provider-Trait
- [ ] Tests für LLM-Provider-Trait schreiben
- [ ] `LLMProvider` Trait definieren
  - `generate_text()` Methode
  - `stream_text()` Methode (Streaming)
  - `count_tokens()` Methode
  - `get_model_info()` Methode
- [ ] Tests ausführen und bestehen

#### 3.1.2 Vision-Provider-Trait
- [ ] Tests für Vision-Provider-Trait schreiben
- [ ] `VisionProvider` Trait definieren
  - `analyze_image()` Methode
  - `analyze_video()` Methode
  - `stream_video_analysis()` Methode (Streaming)
- [ ] Tests ausführen und bestehen

---

## Phase 4: Local Provider Integration (llama.cpp + BitNet.cpp)

### 4.1 llama.cpp + BitNet.cpp Providers

**Abhängigkeiten**: 3.1 (Provider-Interface)
**Erforderliche USER-Eingaben**: Local-LLM-Provider (llama.cpp + BitNet.cpp)

#### 4.1.1 llama.cpp Bindings
- [ ] Tests für llama.cpp-Bindings schreiben
- [ ] `LlamaCppClient` implementieren (TDD)
  - FFI-Bindings zu llama.cpp
  - Model-Loading (GGUF-Format)
  - Error-Handling
- [ ] Tests ausführen und bestehen

#### 4.1.2 BitNet.cpp Bindings (1-bit Modelle)
- [ ] Tests für BitNet.cpp-Bindings schreiben
- [ ] `BitNetCppClient` implementieren (TDD)
  - FFI-Bindings zu BitNet.cpp
  - 1-bit Model-Loading
  - Extreme-Effizienz-Modus
  - Error-Handling
- [ ] Tests ausführen und bestehen

#### 4.1.3 llama.cpp-LLM-Provider
- [ ] Tests für llama.cpp-LLM-Provider schreiben
- [ ] `LlamaCppLLMProvider` implementieren (TDD)
  - `LLMProvider` Trait implementieren
  - Text-Generierung via llama.cpp
  - Streaming-Support
- [ ] Tests ausführen und bestehen

#### 4.1.4 BitNet.cpp-LLM-Provider (1-bit Modelle)
- [ ] Tests für BitNet.cpp-LLM-Provider schreiben
- [ ] `BitNetCppLLMProvider` implementieren (TDD)
  - `LLMProvider` Trait implementieren
  - Text-Generierung via BitNet.cpp (1-bit)
  - Extreme-Effizienz-Modus
  - Streaming-Support
- [ ] Tests ausführen und bestehen

#### 4.1.5 llama.cpp-Vision-Provider (optional)
- [ ] Tests für llama.cpp-Vision-Provider schreiben
- [ ] `LlamaCppVisionProvider` implementieren (TDD)
  - `VisionProvider` Trait implementieren
  - Bild-Analyse via llama.cpp (llava, bakllava)
- [ ] Tests ausführen und bestehen

---

## Phase 5: Cloud Provider Integration

### 5.1 OpenAI Provider

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 5.1.1 OpenAI-Client
- [ ] Tests für OpenAI-Client schreiben
- [ ] `OpenAIClient` implementieren (TDD)
  - HTTP-Requests an OpenAI-API
  - API-Key-Management (aus Settings)
  - Connection-Pooling
  - Rate-Limiting
- [ ] Tests ausführen und bestehen

#### 5.1.2 OpenAI-LLM-Provider
- [ ] Tests für OpenAI-LLM-Provider schreiben
- [ ] `OpenAILLMProvider` implementieren (TDD)
  - `LLMProvider` Trait implementieren
  - GPT-4, GPT-3.5, etc.
  - Streaming-Support
- [ ] Tests ausführen und bestehen

#### 5.1.3 OpenAI-Vision-Provider
- [ ] Tests für OpenAI-Vision-Provider schreiben
- [ ] `OpenAIVisionProvider` implementieren (TDD)
  - `VisionProvider` Trait implementieren
  - GPT-4V für Bild-Analyse
  - Video-Analyse (falls unterstützt)
- [ ] Tests ausführen und bestehen

### 5.2 Anthropic Provider

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 5.2.1 Anthropic-Client
- [ ] Tests für Anthropic-Client schreiben
- [ ] `AnthropicClient` implementieren (TDD)
  - HTTP-Requests an Anthropic-API
  - API-Key-Management
  - Connection-Pooling
- [ ] Tests ausführen und bestehen

#### 5.2.2 Anthropic-LLM-Provider
- [ ] Tests für Anthropic-LLM-Provider schreiben
- [ ] `AnthropicLLMProvider` implementieren (TDD)
  - `LLMProvider` Trait implementieren
  - Claude Models
  - Streaming-Support
- [ ] Tests ausführen und bestehen

#### 5.2.3 Anthropic-Vision-Provider
- [ ] Tests für Anthropic-Vision-Provider schreiben
- [ ] `AnthropicVisionProvider` implementieren (TDD)
  - `VisionProvider` Trait implementieren
  - Claude Vision für Bild-Analyse
- [ ] Tests ausführen und bestehen

### 5.3 Google Provider (optional)

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 5.3.1 Google-Client
- [ ] Tests für Google-Client schreiben
- [ ] `GoogleClient` implementieren (TDD)
  - HTTP-Requests an Google-API
  - API-Key-Management
- [ ] Tests ausführen und bestehen

#### 5.3.2 Google-LLM-Provider
- [ ] Tests für Google-LLM-Provider schreiben
- [ ] `GoogleLLMProvider` implementieren (TDD)
  - `LLMProvider` Trait implementieren
  - Gemini Models
- [ ] Tests ausführen und bestehen

---

## Phase 6: Model Management & Registry

### 6.1 Model Registry

**Abhängigkeiten**: 3.1 (Provider-Interface)
**Erforderliche USER-Eingaben**: Model-Registry-Storage

#### 6.1.1 Model-Info Structure
- [ ] Tests für Model-Info schreiben
- [ ] `ModelInfo` Struct definieren
  - id, name, provider, type, parameter_count, etc.
  - Hardware-Requirements
  - Performance-Metriken
- [ ] Tests ausführen und bestehen

#### 6.1.2 Model-Registry
- [ ] Tests für Model-Registry schreiben
- [ ] `ModelRegistry` implementieren (TDD)
  - Register/Unregister-Models
  - Get-Model-by-ID
  - List-All-Models
  - Filter-Models (by type, provider, etc.)
- [ ] Tests ausführen und bestehen

#### 6.1.3 Model-Discovery (Einherjar Protocol)
- [ ] Tests für Model-Discovery schreiben
- [ ] `ModelDiscovery` implementieren (TDD)
  - Einherjar Protocol nutzen für Network-Model-Discovery
  - Remote-Models registrieren
  - Model-Availability tracken
- [ ] Tests ausführen und bestehen

### 6.2 Model Health Monitoring

**Abhängigkeiten**: 6.1 (Model Registry)

#### 6.2.1 Health-Checker
- [ ] Tests für Health-Checker schreiben
- [ ] `ModelHealthChecker` implementieren (TDD)
  - Periodische Health-Checks für Models
  - Availability-Status tracken
  - Uptime-Percentage berechnen
- [ ] Tests ausführen und bestehen

---

## Phase 7: Model Selection (Multi-Faktor-Bewertung)

### 7.1 Efficiency Scoring

**Abhängigkeiten**: 6.1 (Model Registry)

#### 7.1.1 Efficiency-Score-Calculator
- [ ] Tests für Efficiency-Score schreiben
- [ ] `EfficiencyScoreCalculator` implementieren (TDD)
  - Model-Size-Score berechnen (20%)
  - Hardware-Score berechnen (15%)
  - Reliability-Score berechnen (20%)
  - Latency-Score berechnen (25%)
  - Distance-Score berechnen (10%)
  - Cost-Score berechnen (10%)
  - Gesamt-Efficiency-Score berechnen
- [ ] Tests ausführen und bestehen

### 7.2 Model Selector

**Abhängigkeiten**: 7.1 (Efficiency Scoring), 6.1 (Model Registry)

#### 7.2.1 Model-Selection-Engine
- [ ] Tests für Model-Selection schreiben
- [ ] `ModelSelector` implementieren (TDD)
  - Automatische Model-Auswahl basierend auf Efficiency-Score
  - User-Explizite Model-Auswahl (übersteuert automatische Auswahl)
  - Constraint-basierte Auswahl (Budget, Latency, etc.)
- [ ] Tests ausführen und bestehen

### 7.3 Load Balancing

**Abhängigkeiten**: 7.1 (Efficiency Scoring), 7.2 (Model Selector)

#### 7.3.1 Load-Balancer
- [ ] Tests für Load-Balancer schreiben
- [ ] `LoadBalancer` implementieren (TDD)
  - Gewichtete Provider-Auswahl basierend auf Efficiency-Score
  - Request-Counting pro Provider
  - Load-Threshold-Überwachung (80%)
  - Dynamische Gewichtungs-Anpassung
- [ ] Tests ausführen und bestehen

---

## Phase 8: Prompt Processing

### 8.1 Prompt Formatting

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 8.1.1 Prompt-Formatter
- [ ] Tests für Prompt-Formatter schreiben
- [ ] `PromptFormatter` implementieren (TDD)
  - System-Prompt hinzufügen
  - User-Prompt formatieren
  - Provider-spezifische Formatierung
- [ ] Tests ausführen und bestehen

### 8.2 RAG-Context-Integration

**Abhängigkeiten**: 8.1 (Prompt Formatting)

#### 8.2.1 Context-Formatter
- [ ] Tests für Context-Formatter schreiben
- [ ] `ContextFormatter` implementieren (TDD)
  - RAG-Context formatieren (siehe README.md Struktur)
  - Dokumente nach Relevanz sortieren
  - Metadaten beibehalten
- [ ] Tests ausführen und bestehen

#### 8.2.2 Context-Integrator
- [ ] Tests für Context-Integrator schreiben
- [ ] `ContextIntegrator` implementieren (TDD)
  - Context zwischen System-Prompt und User-Prompt einfügen
  - Prompt-Template anwenden
- [ ] Tests ausführen und bestehen

### 8.3 Context-Window-Management

**Abhängigkeiten**: 8.2 (RAG-Context-Integration)

#### 8.3.1 Token-Counter
- [ ] Tests für Token-Counter schreiben
- [ ] `TokenCounter` implementieren (TDD)
  - Token-Anzahl für Text berechnen
  - Model-spezifisches Token-Counting
- [ ] Tests ausführen und bestehen

#### 8.3.2 Context-Window-Manager
- [ ] Tests für Context-Window-Manager schreiben
- [ ] `ContextWindowManager` implementieren (TDD)
  - Context-Größe prüfen gegen Model-Limit
  - Response-Reserve berechnen (20%)
  - Context-Truncation wenn nötig (Relevanz-basiert)
  - Dokument-Deduplizierung
  - Fallback zu größerem Context-Window-Model
- [ ] Tests ausführen und bestehen

---

## Phase 9: Cost Management

### 9.1 Token Counting

**Abhängigkeiten**: 8.3.1 (Token-Counter)

#### 9.1.1 Provider-Specific Token-Counter
- [ ] Tests für Provider-spezifisches Token-Counting schreiben
- [ ] Provider-spezifische Token-Counter implementieren (TDD)
  - OpenAI Token-Counter
  - Anthropic Token-Counter
  - etc.
- [ ] Tests ausführen und bestehen

### 9.2 Cost Calculation

**Abhängigkeiten**: 9.1 (Token Counting)

#### 9.2.1 Cost-Calculator
- [ ] Tests für Cost-Calculator schreiben
- [ ] `CostCalculator` implementieren (TDD)
  - Cost pro Token für jeden Provider
  - Input-Tokens + Output-Tokens
  - Total-Cost berechnen
- [ ] Tests ausführen und bestehen

### 9.3 Budget Management

**Abhängigkeiten**: 9.2 (Cost Calculation)

#### 9.3.1 Budget-Manager
- [ ] Tests für Budget-Manager schreiben
- [ ] `BudgetManager` implementieren (TDD)
  - Budget-Limits verwalten
  - Budget-Usage tracken
  - Budget-Limit-Erkennung
  - Budget-Alerts
- [ ] Tests ausführen und bestehen

---

## Phase 10: Fallback System

### 10.1 Cloud-to-Local Fallback

**Abhängigkeiten**: 7.2 (Model Selector), 6.1 (Model Registry)

#### 10.1.1 Fallback-Manager
- [ ] Tests für Fallback-Manager schreiben
- [ ] `FallbackManager` implementieren (TDD)
  - Cloud-Limit-Erkennung
  - Automatischer Fallback zu lokalem LLM
  - Bestes lokales LLM identifizieren (Multi-Faktor-Bewertung)
  - Netzwerk-LLM-Suche via Einherjar Protocol
- [ ] Tests ausführen und bestehen

### 10.2 Fallback-Notifications

**Abhängigkeiten**: 10.1 (Fallback-Manager)

#### 10.2.1 Notification-Generator
- [ ] Tests für Notification-Generator schreiben
- [ ] `FallbackNotificationGenerator` implementieren (TDD)
  - Benachrichtigungs-Text generieren (siehe README.md Beispiele)
  - Notification an Odin senden (für TTS via Muninn)
  - User-Einstellungen berücksichtigen (Notifications aktiviert/deaktiviert)
- [ ] Tests ausführen und bestehen

### 10.3 Automatic Return to Cloud LLM

**Abhängigkeiten**: 10.1 (Fallback-Manager), 9.3 (Budget-Manager)

#### 10.3.1 Budget-Reset-Listener
- [ ] Tests für Budget-Reset-Listener schreiben
- [ ] `BudgetResetListener` implementieren (TDD)
  - Yggdrasil-Integration für Budget-Reset-Events
  - Automatische Rückkehr zu Cloud-LLM nach Reset
- [ ] Tests ausführen und bestehen

---

## Phase 11: Caching System

### 11.1 Response-Caching

**Abhängigkeiten**: 8.1 (Prompt Formatting)

#### 11.1.1 Cache-Manager
- [ ] Tests für Cache-Manager schreiben
- [ ] `CacheManager` implementieren (TDD)
  - Responses für ähnliche Prompts cachen
  - Cache-Key-Generierung (basierend auf Prompt-Hash)
  - Cache-Hit/Miss-Handling
  - TTL-basierte Expiration
- [ ] Tests ausführen und bestehen

### 11.2 Cache-Invalidation

**Abhängigkeiten**: 11.1 (Response-Caching)

#### 11.2.1 Cache-Invalidator
- [ ] Tests für Cache-Invalidation schreiben
- [ ] `CacheInvalidator` implementieren (TDD)
  - Event-basierte Invalidation (Model-Updates, Provider-Status-Änderungen)
  - Timeout-basierte Invalidation (Fallback)
- [ ] Tests ausführen und bestehen

---

## Phase 12: Streaming Support

### 12.1 LLM-Response-Streaming

**Abhängigkeiten**: 3.1 (Provider-Interface), 2.2.2 (Wolf Service)

#### 12.1.1 Streaming-Manager
- [ ] Tests für Streaming-Manager schreiben
- [ ] `StreamingManager` implementieren (TDD)
  - Streaming für LLM-Responses (wenn Provider unterstützt)
  - Chunk-basiertes Streaming
  - Error-Handling bei Streaming-Fehlern
- [ ] Tests ausführen und bestehen

### 12.2 Video-Stream-Processing

**Abhängigkeiten**: 2.2.3 (Vision Service)

#### 12.2.1 Video-Stream-Processor
- [ ] Tests für Video-Stream-Processing schreiben
- [ ] `VideoStreamProcessor` implementieren (TDD)
  - Video-Stream-Chunks verarbeiten
  - Frame-Extraction aus Video-Stream
  - Vision-Model für Frame-Analyse nutzen
  - Streaming-Results zurückgeben
- [ ] Tests ausführen und bestehen

---

## Phase 13: Request Queuing & Prioritization

### 13.1 Request-Queue

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 13.1.1 Request-Queue-Manager
- [ ] Tests für Request-Queue schreiben
- [ ] `RequestQueueManager` implementieren (TDD)
  - Requests in Queue bei hoher Last
  - FIFO-Queue-Processing
  - Queue-Backlog-Handling
- [ ] Tests ausführen und bestehen

### 13.2 Priority-Queue

**Abhängigkeiten**: 13.1 (Request-Queue)

#### 13.2.1 Priority-Queue-Manager
- [ ] Tests für Priority-Queue schreiben
- [ ] `PriorityQueueManager` implementieren (TDD)
  - Requests mit Priorität versehen
  - Priority-basierte Queue-Processing
  - High-Priority-Requests bevorzugen
- [ ] Tests ausführen und bestehen

---

## Phase 14: API-Key-Management & Secure Storage

### 14.1 API-Key-Storage

**Abhängigkeiten**: 1.3 (Settings-System)

#### 14.1.1 Secure-Key-Storage
- [ ] Tests für Secure-Key-Storage schreiben
- [ ] `SecureKeyStorage` implementieren (TDD)
  - API-Keys in OS-Secure-Storage speichern (Platform-spezifisch)
  - API-Keys laden
  - Encryption für Keys
- [ ] Tests ausführen und bestehen

### 14.2 Key-Rotation

**Abhängigkeiten**: 14.1 (API-Key-Storage)

#### 14.2.1 Key-Rotation-Manager
- [ ] Tests für Key-Rotation schreiben
- [ ] `KeyRotationManager` implementieren (TDD)
  - Alte API-Keys entfernen
  - Neue API-Keys hinzufügen
  - Rotation-Workflow
- [ ] Tests ausführen und bestehen

---

## Phase 15: Performance Monitoring & Metrics

### 15.1 Metrics Collector

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 15.1.1 Performance-Metrics-Collector
- [ ] Tests für Metrics-Collector schreiben
- [ ] `MetricsCollector` implementieren (TDD)
  - Response-Zeiten tracken
  - Durchsatz tracken
  - Resource-Usage tracken
  - Latency-Metriken
- [ ] Tests ausführen und bestehen

### 15.2 Model-Performance-Tracker

**Abhängigkeiten**: 15.1 (Metrics Collector)

#### 15.2.1 Model-Performance-Tracker
- [ ] Tests für Model-Performance-Tracker schreiben
- [ ] `ModelPerformanceTracker` implementieren (TDD)
  - Tokens pro Sekunde tracken
  - Model-Response-Zeiten tracken
  - Model-Verfügbarkeit tracken
- [ ] Tests ausführen und bestehen

---

## Phase 16: Error Handling & Retry Logic

### 16.1 Error-Handler

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 16.1.1 Provider-Error-Handler
- [ ] Tests für Error-Handler schreiben
- [ ] `ProviderErrorHandler` implementieren (TDD)
  - Provider-spezifische Fehler behandeln
  - gRPC-Status-Codes
  - Logging
- [ ] Tests ausführen und bestehen

### 16.2 Retry-Manager

**Abhängigkeiten**: 16.1 (Error-Handler)

#### 16.2.1 Retry-Manager
- [ ] Tests für Retry-Manager schreiben
- [ ] `RetryManager` implementieren (TDD)
  - Exponential-Backoff-Retry
  - Max-Retry-Count
  - Retry-Delay berechnen
- [ ] Tests ausführen und bestehen

---

## Phase 17: Default Local LLM Installation

### 17.1 Local-LLM-Installer

**Abhängigkeiten**: 4.1 (llama.cpp Provider)
**Erforderliche USER-Eingaben**: Default-Local-LLM

#### 17.1.1 LLM-Installer
- [ ] Tests für LLM-Installer schreiben
- [ ] `LLMInstaller` implementieren (TDD)
  - Default-Local-LLM herunterladen (Llama 3 8B GGUF für Standard-Hardware)
  - Alternative 1-bit Model herunterladen (BitNet 3B für schwache Hardware)
  - llama.cpp kompilieren (falls nicht vorhanden)
  - BitNet.cpp kompilieren (falls nicht vorhanden)
  - Model-Paths konfigurieren
  - Auto-Auswahl basierend auf verfügbarem RAM
  - Verfügbarkeit prüfen
- [ ] Tests ausführen und bestehen

---

## Phase 18: Monitoring & Logging

### 18.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 18.1.1 Logging Setup
- [ ] Structured-Logging konfigurieren (tracing)
- [ ] LLM-specific Log-Levels
- [ ] Log-Rotation konfigurieren

### 18.2 Context-Tracking

**Abhängigkeiten**: 18.1 (Structured Logging)

#### 18.2.1 Context-Logger
- [ ] Tests für Context-Logging schreiben
- [ ] `ContextLogger` implementieren (TDD)
  - Request-Context in Logs
  - Trace-IDs für Request-Tracking
- [ ] Tests ausführen und bestehen

---

## Phase 19: Documentation

### 19.1 API Documentation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 19.1.1 gRPC Service Documentation
- [ ] gRPC-Service-Documentation erstellen
- [ ] Wolf-Service-API dokumentieren
- [ ] Vision-Service-API dokumentieren

### 19.2 Provider Documentation

**Abhängigkeiten**: 4.1 (llama.cpp Provider), 5.1-5.3 (Cloud Providers)

#### 19.2.1 Provider-Integration-Guide
- [ ] Provider-Integration-Guide erstellen
- [ ] API-Key-Setup dokumentieren
- [ ] Local-LLM-Setup dokumentieren

---

## Phase 20: Testing & Quality Assurance

### 20.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 20.1.1 End-to-End Tests
- [ ] E2E-Tests für LLM-Workflows schreiben
  - WolfRequest → Model-Selection → LLM-Call → WolfResponse
  - RAG-Context-Integration → LLM-Call → Response
  - Cloud-Limit-Fallback → Local-LLM → Response
- [ ] E2E-Tests ausführen und bestehen

### 20.2 Performance Testing

**Abhängigkeiten**: 15.1 (Metrics Collector)

#### 20.2.1 Performance Test Suite
- [ ] Performance-Tests ausführen
  - Response-Time-Tests (< 500ms für lokale LLMs, < 2s für Cloud LLMs)
  - Throughput-Tests (parallele Requests)
  - Streaming-Performance-Tests
- [ ] Performance-Tests bestehen

### 20.3 Load Testing

**Abhängigkeiten**: 13.1 (Request-Queue)

#### 20.3.1 Load Test Suite
- [ ] Load-Tests ausführen
  - High-Concurrency-Tests
  - Queue-Backlog-Tests
  - Load-Balancing-Tests
- [ ] Load-Tests bestehen

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 20
**Gesamtanzahl Schritte**: ~300+

**Kritische Abhängigkeiten**:
1. Protobuf-Rust-Tool (prost + tonic empfohlen)
2. Default-Local-LLM (Llama 3 8B für Standard-Hardware, BitNet 3B für schwache Hardware)
3. Local-LLM-Provider (llama.cpp + BitNet.cpp für 1-bit Modelle)
4. Vision-Model-Default (GPT-4V empfohlen)
5. Model-Registry-Storage (PostgreSQL)

**Offene Fragen für USER**:
1. Protobuf-Rust-Tool (prost + tonic)
2. Default-Local-LLM (Llama 3 8B standard, BitNet 3B für extreme Effizienz)
3. Local-LLM-Provider (llama.cpp + BitNet.cpp)
4. Vision-Model-Default (GPT-4V)
5. Model-Registry-Storage (PostgreSQL)

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
- Multi-Faktor-Bewertung für Model-Auswahl ist kritisch
- Load-Balancing verhindert Überlastung einzelner Provider
- Lokales LLM muss garantiert sein (mit Installation mitgeliefert)
- Vision-Model-Support für Bild/Video-Interpretation
- Fallback zu lokalem LLM bei Cloud-Limit
