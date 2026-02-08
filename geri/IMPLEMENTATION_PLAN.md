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
- [x] `geri/src/model/` für Model-Management erstellen (mod.rs Stub)
- [x] Provider-Integration: `llm/`, `evaluation/`, `vision/` vorhanden
- [x] `geri/src/prompt/` für Prompt-Processing erstellen (mod.rs Stub)
- [x] `geri/src/cost/` für Cost-Management erstellen (mod.rs Stub)
- [x] `geri/src/selection/` für Model-Selection erstellen (mod.rs Stub)
- [x] `geri/src/vision/` für Vision-Model-Support erstellen
- [x] `geri/src/grpc/` für gRPC-Service erstellen
- [x] `geri/src/utils/` für Utilities erstellen
- [x] `geri/config/` für Konfigurationsdateien erstellen
- [x] `geri/tests/` für Tests erstellen

#### 1.1.3 Build-System einrichten
- [x] Build-Scripts in `Cargo.toml` definieren
- [x] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [x] Cargo-Features definieren (`ollama`, `openai`, `anthropic` in `[features]`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Dockerfile.test angepasst: Cargo.lock optional (generate-lockfile im Container), protobuf-compiler, `cargo build --release --tests`
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
  - Mock-llama.cpp-Service
  - Mock-BitNet.cpp-Service (1-bit Models)
  - Mock-OpenAI-API
  - Mock-Anthropic-API
  - Mock-Odin-Service
  - [x] Test-Container-Startup-Scripts erstellen (scripts/run-tests.ps1, scripts/run-tests.sh)
  - **WICHTIG**: Alle Tests müssen in Containern laufen – keine lokalen Dependencies (CI und scripts/run-tests.* nutzen Container).

#### 1.2.2 Test-Framework konfigurieren
- [x] Test-Dependencies hinzufügen (tokio-test, mockall, tempfile, wiremock)
- [x] Test-Utilities und Helpers erstellen (tests/utils/test_helpers.rs)
- [x] Mock-Setup für Provider-APIs (wiremock für HTTP-Mocks; mock-odin in docker-compose)
- [x] Test-Data-Generators für Prompts erstellen (tests/utils/prompt_test_data.rs: sample_system_prompt, sample_user_prompt, sample_rag_context, etc.)

#### 1.2.3 CI/CD-Pipeline
- [x] GitHub Actions Workflow erstellen (`.github/workflows/geri.yml`)
- [x] Automatische Test-Ausführung bei Push/PR auf `geri/**` (Test im Container)
- [x] Linting und Formatting (cargo fmt --check, cargo clippy)
- [x] Code-Coverage-Reporting (cargo-tarpaulin, Artefakt `geri-coverage` in `.github/workflows/geri.yml`)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren
- [x] Settings-Struktur (GeriSettings: grpc_port, default_local_llm, vision_model) – `src/utils/config.rs`
- [ ] Erweiterung optional: models, default_model, fallback, performance

#### 1.3.2 Settings-Validierung
- [x] Tests für Settings-Validierung schreiben (`src/utils/config.rs`: test_validate_default_ok, invalid_port, empty_default_local_llm, empty_vision_model)
- [x] Rust-Structs für Settings definieren (GeriSettings)
- [x] Settings-Validator implementieren (GeriSettings::validate, SettingsError)
- [x] Validierung in load() und Hot-Reload integriert

#### 1.3.3 Settings-Loader & Hot-Reload
- [x] Settings-Loader implementiert (SettingsManager::load, get)
- [x] Hot-Reload-Mechanismus implementiert (start_hot_reload, notify)
- [ ] Tests für Settings-Loader (optional, z. B. mit tempfile)

---

## Phase 2: Protobuf & gRPC Setup

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [x] Geri als Dependency zu Protobuf-Projekt hinzufügen
- [x] Protobuf-Definitions importieren
**Status**: `geri/proto/geri.proto` existiert und wird verwendet

#### 2.1.2 Wolf Protocol (WolfRequest/WolfResponse)
- [x] `Wolf.proto` verwenden (falls nicht vorhanden, erstellen)
  - `WolfRequest` Message → `ProcessPromptRequest` (prompt, context/ragContext, model_name, max_tokens)
  - `WolfResponse` Message → `ProcessPromptResponse` (text, tokens_used, model_used)
- [x] Code-Generierung konfigurieren (`build.rs` mit tonic_build)
**Status**: Implementiert als `ProcessPrompt*` in `geri.proto`

#### 2.1.3 Vision Protocol
- [x] `Vision.proto` definieren
  - `ImageAnalysisRequest` Message → `ProcessVisionRequest` (image_data, prompt, model_name)
  - `ImageAnalysisResponse` Message → `ProcessVisionResponse` (description, analysis_data, model_used)
  - `VideoAnalysisRequest` Message → optional (noch nicht benötigt)
  - `VideoAnalysisResponse` Message → optional
  - `VideoStreamChunk` Message → optional
  - `VideoAnalysisChunk` Message → optional
- [x] Code-Generierung konfigurieren (`build.rs`)
**Status**: Basis-Vision-Protocol in `geri.proto` vorhanden; Video/Streaming optional für später

### 2.2 gRPC Server Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Server Setup
- [x] Tests für gRPC-Handler schreiben (`tests/grpc_server_test.rs`: process_prompt, process_vision)
- [x] gRPC-Server-Setup (tonic, `src/grpc/server.rs`, `start_grpc_server`)
- [ ] Health-Check-Service (tonic health) – optional

#### 2.2.2 Wolf Service (LLM-Service)
- [x] Tests für ProcessPrompt schreiben (grpc_server_test)
- [x] `GeriServiceImpl::process_prompt` (ProcessPromptRequest → ProcessPromptResponse, Anbindung LLMProvider)
- [x] Response-Felder an Proto angepasst (model_used)

#### 2.2.3 Vision Service
- [x] Tests für ProcessVision schreiben (grpc_server_test)
- [x] `GeriServiceImpl::process_vision` (ProcessVisionRequest → ProcessVisionResponse, Anbindung VisionProcessor)
- [x] Vision-Anbindung an VisionProcessor/Proto angepasst (process, analysis_data, model_used)
- [ ] Video/Streaming (AnalyzeVideo, AnalyzeVideoStream) – optional

---

## Phase 3: Provider Abstraction Layer

### 3.1 Provider-Interface

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 3.1.1 LLM-Provider-Trait
- [x] Tests für LLM-Provider (`tests/unit/llm_provider_test.rs`, `tests/grpc_server_test.rs`)
- [x] `LLMProvider` Trait – `src/llm/provider.rs` (process_prompt, model_name)
- [ ] stream_text(), count_tokens(), get_model_info() – optional / spätere Phase

#### 3.1.2 Vision-Provider-Trait
- [x] Vision-Processor – `src/vision/processor.rs` (process = analyze_image-äquivalent)
- [x] VisionProvider-Trait – `src/vision/provider.rs` (model_name, process; VisionProcessor implementiert)
- [x] Gemeinsame Typen – `src/vision/types.rs` (VisionRequest, VisionResponse, VisionError)
- [x] Tests – `tests/unit/vision_provider_test.rs`
- [ ] analyze_video(), stream_video_analysis() – optional

---

## Phase 4: Local Provider Integration (llama.cpp + BitNet.cpp)

### 4.1 llama.cpp + BitNet.cpp Providers

**Abhängigkeiten**: 3.1 (Provider-Interface)
**Erforderliche USER-Eingaben**: Local-LLM-Provider (llama.cpp + BitNet.cpp)

#### 4.1.1 llama.cpp Bindings ✅
- [x] Tests für llama.cpp-Bindings schreiben (`tests/unit/llamacpp_client_test.rs`)
- [x] `LlamaCppClient` implementieren (TDD) - `src/llm/llamacpp/client.rs`
  - Config-Validierung (model_path, n_ctx, n_threads, n_gpu_layers) ✅
  - GGUF-Format-Support (model loading stub) ✅
  - Error-Handling (LlamaCppError) ✅
  - Model-Info (name, context_size) ✅
- [x] Tests ausführen und bestehen ✅
- **Hinweis**: FFI-Bindings als Stub implementiert; echte llama.cpp FFI-Integration folgt später

#### 4.1.2 BitNet.cpp Bindings (1-bit Modelle) ✅
- [x] Tests für BitNet.cpp-Bindings schreiben (`tests/unit/bitnet_client_test.rs`)
- [x] `BitNetClient` implementieren (TDD) - `src/llm/bitnet/client.rs`
  - 1-bit Model-Support ✅
  - Extreme-Effizienz-Modus (use_extreme_efficiency) ✅
  - Memory-Estimation (~90% Reduktion vs. FP16) ✅
  - Error-Handling (BitNetError) ✅
- [x] Tests ausführen und bestehen ✅
- **Hinweis**: FFI-Bindings als Stub implementiert; echte BitNet.cpp FFI-Integration folgt später

#### 4.1.3 llama.cpp-LLM-Provider ✅
- [x] Tests für llama.cpp-LLM-Provider schreiben (`tests/unit/llamacpp_llm_provider_test.rs`)
- [x] `LlamaCppLLMProvider` implementieren (TDD) - `src/llm/llamacpp/provider.rs`
  - `LLMProvider` Trait implementieren ✅
  - Text-Generierung via LlamaCppClient ✅
  - Context-Handling ✅
  - Token-Counting ✅
- [x] Tests ausführen und bestehen ✅
- [ ] Streaming-Support – optional / spätere Phase

#### 4.1.4 BitNet.cpp-LLM-Provider (1-bit Modelle) ✅
- [x] Tests für BitNet.cpp-LLM-Provider schreiben (`tests/unit/bitnet_llm_provider_test.rs`)
- [x] `BitNetLLMProvider` implementieren (TDD) - `src/llm/bitnet/provider.rs`
  - `LLMProvider` Trait implementieren ✅
  - Text-Generierung via BitNetClient (1-bit) ✅
  - Extreme-Effizienz-Modus ✅
  - Memory-Efficiency-Tracking ✅
- [x] Tests ausführen und bestehen ✅
- [ ] Streaming-Support – optional / spätere Phase

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
- [x] Tests für OpenAI-Client schreiben (`tests/unit/openai_client_test.rs`)
- [x] `OpenAIClient` implementieren (TDD)
  - HTTP-Requests an OpenAI-API (reqwest)
  - API-Key-Management (OpenAIConfig)
  - Connection-Pooling (reqwest client)
  - Request builders (chat, vision)
- [x] Tests schreiben (Struktur-Tests, Config-Validierung)
**Status**: Basis-Client implementiert; Rate-Limiting optional für später

#### 5.1.2 OpenAI-LLM-Provider
- [x] Tests für OpenAI-LLM-Provider schreiben (`tests/unit/openai_llm_provider_test.rs`)
- [x] `OpenAILLMProvider` implementieren (TDD)
  - `LLMProvider` Trait implementieren (`process_prompt`)
  - GPT-4, GPT-3.5, etc. (model_name parameter)
  - Chat-Completion-API
- [ ] Streaming-Support – optional / spätere Phase
**Status**: Implementiert; Streaming optional

#### 5.1.3 OpenAI-Vision-Provider
- [x] Vision-Request-Builder implementiert (`OpenAIClient::build_vision_request`, base64-encoding)
- [x] Vision-Completion-Method (`OpenAIClient::vision_completion`)
- [ ] `OpenAIVisionProvider` Trait (VisionProvider) – optional / spätere Phase
- [ ] Tests für OpenAI-Vision-Provider – optional
**Status**: Client-Support vorhanden; Provider-Wrapper optional

### 5.2 Anthropic Provider

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 5.2.1 Anthropic-Client
- [x] Tests für Anthropic-Client schreiben (`tests/unit/anthropic_client_test.rs`)
- [x] `AnthropicClient` implementieren (TDD)
  - HTTP-Requests an Anthropic-API (reqwest)
  - API-Key-Management (AnthropicConfig: `x-api-key` header, `anthropic-version`)
  - Connection-Pooling (reqwest client)
  - Request builders (messages, vision)
- [x] Tests schreiben (Struktur-Tests, Config-Validierung)
**Status**: Basis-Client implementiert; Messages-API mit system-parameter

#### 5.2.2 Anthropic-LLM-Provider
- [x] Tests für Anthropic-LLM-Provider schreiben (`tests/unit/anthropic_llm_provider_test.rs`)
- [x] `AnthropicLLMProvider` implementieren (TDD)
  - `LLMProvider` Trait implementieren (`process_prompt`)
  - Claude Models (claude-3-opus, claude-3-sonnet, claude-3-haiku)
  - Messages-API mit input_tokens + output_tokens
- [ ] Streaming-Support – optional / spätere Phase
**Status**: Implementiert; Streaming optional

#### 5.2.3 Anthropic-Vision-Provider
- [x] Vision-Request-Builder implementiert (`AnthropicClient::build_vision_request`, base64-encoding, VisionContentBlock)
- [x] Vision-Messages-Method (`AnthropicClient::vision_messages`)
- [ ] `AnthropicVisionProvider` Trait (VisionProvider) – optional / spätere Phase
- [ ] Tests für Anthropic-Vision-Provider – optional
**Status**: Client-Support vorhanden; Provider-Wrapper optional

### 5.3 Google Provider (optional)

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 5.3.1 Google-Client ✅
- [x] Tests für Google-Client schreiben (`tests/unit/google_client_test.rs`)
- [x] `GoogleClient` implementieren (TDD) – `src/llm/google/client.rs`
  - HTTP-Requests an Google-API
  - API-Key-Management
  - Header: `x-goog-api-key`
  - Vision-Support (inline_data mit base64)
- [x] Tests ausführen und bestehen

#### 5.3.2 Google-LLM-Provider ✅
- [x] Tests für Google-LLM-Provider schreiben (`tests/unit/google_llm_provider_test.rs`)
- [x] `GoogleLLMProvider` implementieren (TDD) – `src/llm/google/provider.rs`
  - `LLMProvider` Trait implementieren
  - Gemini Models (gemini-2.5-flash, etc.)
- [x] Tests ausführen und bestehen

---

## Phase 6: Model Management & Registry

### 6.1 Model Registry

**Abhängigkeiten**: 3.1 (Provider-Interface)
**Erforderliche USER-Eingaben**: Model-Registry-Storage

#### 6.1.1 Model-Info Structure
- [x] Tests für Model-Info schreiben (`tests/unit/model_registry_test.rs`: required fields)
- [x] `ModelInfo` Struct definieren – `src/model/info.rs`
  - id, name, provider, model_type (ModelType::Llm/Vision), parameter_count, hardware_requirements, context_window
  - ModelType enum (Llm, Vision)
- [ ] Performance-Metriken – optional / spätere Phase

#### 6.1.2 Model-Registry
- [x] Tests für Model-Registry schreiben (`tests/unit/model_registry_test.rs`: register, get_by_id, list_all, filter_by_type, filter_by_provider, unregister)
- [x] `ModelRegistry` implementieren (TDD) – `src/model/registry.rs` (In-Memory)
  - Register/Unregister-Models (`register(model)`, `unregister(id)`)
  - Get-Model-by-ID (`get_by_id(id)`)
  - List-All-Models (`list_all()`)
  - Filter-Models (`filter_by_type(ModelType)`, `filter_by_provider(provider)`)
- [x] **gRPC-Endpunkte für Model-Registry** (`geri.proto`: ListModels, GetModelInfo; `server.rs`: list_models, get_model_info; Tests: `tests/unit/grpc_model_registry_test.rs`) ✅
- [ ] Persistence (PostgreSQL) – optional / spätere Phase

#### 6.1.3 Model-Discovery (Einherjar Protocol)
- [x] Tests für Model-Discovery schreiben (`tests/unit/model_discovery_test.rs`: discover_and_register_registers_discovered_models, handles_client_error, preserves_existing_models)
- [x] `ModelDiscovery` implementieren (TDD) – `src/model/discovery.rs`
  - Einherjar-Client als Trait (`EinherjarCapabilityClient`) definiert (später gRPC/Einherjar-Integration)
  - Remote-Models registrieren (`discover_and_register` gibt aktualisiertes `ModelRegistry` zurück)
  - (Model-Availability-Tracking folgt später in Health-/Metrics-Komponenten)
- [x] Tests ausführen und bestehen (Container: `docker compose -f geri/docker-compose.test.yml run --rm geri-test`)

### 6.2 Model Health Monitoring

**Abhängigkeiten**: 6.1 (Model Registry)

#### 6.2.1 Health-Checker
- [x] Tests für Health-Checker schreiben (`tests/unit/model_health_checker_test.rs`)
- [x] `ModelHealthChecker` implementieren (TDD) – `src/model/health.rs`
  - Periodische Health-Checks für Models (`run_check_all()`)
  - Availability-Status tracken (`get_availability(model_id)`)
  - Uptime-Percentage berechnen (`get_uptime_percentage(model_id)`, letzte 100 Checks)
  - `ModelHealthProbe` Trait für injizierbare Checks (Tests/Provider)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test model_health_checker`)

---

## Phase 7: Model Selection (Multi-Faktor-Bewertung)

### 7.1 Efficiency Scoring

**Abhängigkeiten**: 6.1 (Model Registry)

#### 7.1.1 Efficiency-Score-Calculator
- [x] Tests für Efficiency-Score schreiben (`tests/unit/efficiency_score_test.rs`)
- [x] `EfficiencyScoreCalculator` implementieren (TDD) – `src/selection/efficiency.rs`
  - Model-Size-Score berechnen (20%) – `parameter_count / max_parameter_count`
  - Hardware-Score berechnen (15%) – 0.0–1.0
  - Reliability-Score berechnen (20%) – `(uptime/100) * (1 - error_rate)`
  - Latency-Score berechnen (25%) – `1.0 - (ping_ms / max_ping_ms)`
  - Distance-Score berechnen (10%) – lokal = 1.0, sonst `1.0 - (distance_km / max)`
  - Cost-Score berechnen (10%) – `1.0 - (cost_per_token / max_cost)`
  - Gesamt-Efficiency-Score berechnen (gewichtete Summe, 0.0–1.0)
  - `EfficiencyInput` für alle Eingaben, `EfficiencyWeights` konfigurierbar
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test efficiency_score`)

### 7.2 Model Selector

**Abhängigkeiten**: 7.1 (Efficiency Scoring), 6.1 (Model Registry)

#### 7.2.1 Model-Selection-Engine
- [x] Tests für Model-Selection schreiben (`tests/unit/model_selector_test.rs`)
- [x] `ModelSelector` implementieren (TDD) – `src/selection/selector.rs`
  - Automatische Model-Auswahl basierend auf Efficiency-Score (`select(candidates, options)`)
  - User-Explizite Model-Auswahl (übersteuert automatische Auswahl via `SelectionOptions::user_preferred_model_id`)
  - Constraint-basierte Auswahl (`max_latency_ms`, `max_cost_per_token` filtern Kandidaten)
  - `SelectionOptions` für User-Präferenz und Constraints
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test model_selector`)

### 7.3 Load Balancing

**Abhängigkeiten**: 7.1 (Efficiency Scoring), 7.2 (Model Selector)

#### 7.3.1 Load-Balancer
- [x] Tests für Load-Balancer schreiben (`tests/unit/load_balancer_test.rs`)
- [x] `LoadBalancer` implementieren (TDD) – `src/selection/load_balancer.rs`
  - Gewichtete Provider-Auswahl basierend auf Efficiency-Score (`next(candidates, calculator)`)
  - Request-Counting pro Provider (`record_request(provider_id)`, `get_load(provider_id)`)
  - Load-Threshold-Überwachung (konfigurierbar, Standard 80 %; überlastete Provider werden abgewertet)
  - Dynamische Gewichtungs-Anpassung (effektiver Score = Score × 0.2 bei Load ≥ Threshold)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test load_balancer`)

---

## Phase 8: Prompt Processing

### 8.1 Prompt Formatting

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 8.1.1 Prompt-Formatter
- [x] Tests für Prompt-Formatter schreiben (`tests/unit/prompt_formatter_test.rs`: format mit System+User, mit Context, leeres System, leerer Context, format_for_provider Llama/OpenAI)
- [x] `PromptFormatter` implementieren (TDD) – `src/prompt/formatter.rs`
  - System-Prompt hinzufügen (Default bei leerem System)
  - User-Prompt formatieren (Reihenfolge: System → Context → User)
  - Provider-spezifische Formatierung (`format_for_provider`: Llama [INST]/[/INST], OpenAI System/User-Sections)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test prompt_formatter`)

### 8.2 RAG-Context-Integration

**Abhängigkeiten**: 8.1 (Prompt Formatting)

#### 8.2.1 Context-Formatter
- [x] Tests für Context-Formatter schreiben (`tests/unit/context_formatter_test.rs`: sort by relevance, document sections, empty, single, metadata preserved, format_with_max_chars)
- [x] `ContextFormatter` implementieren (TDD) – `src/prompt/context_formatter.rs`
  - RAG-Context formatieren (`[Document N: id]\ncontent`, siehe README.md Struktur)
  - Dokumente nach Relevanz sortieren (score absteigend)
  - Metadaten beibehalten (`ContextDocument.metadata`, optional)
  - `format_with_max_chars` für Context-Window-Truncation (char-boundary-sicher)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test context_formatter`)

#### 8.2.2 Context-Integrator
- [x] Tests für Context-Integrator schreiben (`tests/unit/context_integrator_test.rs`: ohne Context, mit Dokumenten, Reihenfolge, leere Dokumente, integrate_with_max_chars)
- [x] `ContextIntegrator` implementieren (TDD) – `src/prompt/context_integrator.rs`
  - Context zwischen System-Prompt und User-Prompt einfügen (nutzt PromptFormatter + ContextFormatter)
  - Prompt-Template anwenden (Reihenfolge: System → Context → User)
  - `integrate_with_max_chars` für begrenzte Gesamtlänge (Context-Truncation)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test context_integrator`)

### 8.3 Context-Window-Management

**Abhängigkeiten**: 8.2 (RAG-Context-Integration)

#### 8.3.1 Token-Counter
- [x] Tests für Token-Counter schreiben (`tests/unit/token_counter_test.rs`: empty, non-empty, deterministisch, längere Texte, count_for_model Llama/GPT, empty zero)
- [x] `TokenCounter` implementieren (TDD) – `src/prompt/token_counter.rs`
  - Token-Anzahl für Text berechnen (Heuristik ~4 Zeichen pro Token, leer = 0)
  - Model-spezifisches Token-Counting (`count_for_model(text, model_name)`, erweiterbar)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test token_counter`)

#### 8.3.2 Context-Window-Manager
- [x] Tests für Context-Window-Manager schreiben (`tests/unit/context_window_manager_test.rs`: response_reserve 20%, max_context_tokens, truncate_to_fit, deduplicate_by_id, fits_in_window)
- [x] `ContextWindowManager` implementieren (TDD) – `src/prompt/context_window_manager.rs`
  - Context-Größe prüfen gegen Model-Limit (`fits_in_window(system, user, docs, limit, model)`)
  - Response-Reserve berechnen (20%) (`response_reserve_tokens`, `max_context_tokens`)
  - Context-Truncation wenn nötig (Relevanz-basiert, `truncate_to_fit(docs, max_tokens, model)`)
  - Dokument-Deduplizierung (`deduplicate_by_id(docs)`)
  - [ ] Fallback zu größerem Context-Window-Model – optional / spätere Phase
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test context_window_manager`)

---

## Phase 9: Cost Management

### 9.1 Token Counting

**Abhängigkeiten**: 8.3.1 (Token-Counter)

#### 9.1.1 Provider-Specific Token-Counter
- [x] Tests für Provider-spezifisches Token-Counting schreiben (`tests/unit/provider_token_counter_test.rs`: OpenAI/Anthropic count positive, zero für empty, deterministisch)
- [x] Provider-spezifische Token-Counter implementieren (TDD) – `src/cost/provider_token_counter.rs`
  - OpenAI Token-Counter (`OpenAITokenCounter::new(TokenCounter)`, `count(text, model)`)
  - Anthropic Token-Counter (`AnthropicTokenCounter::new(TokenCounter)`, `count(text, model)`)
  - Nutzen `prompt::TokenCounter::count_for_model` mit provider-prefixed model name (erweiterbar)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test provider_token_counter`)

### 9.2 Cost Calculation

**Abhängigkeiten**: 9.1 (Token Counting)

#### 9.2.1 Cost-Calculator
- [x] Tests für Cost-Calculator schreiben (`tests/unit/cost_calculator_test.rs`: OpenAI, Anthropic, zero tokens, deterministisch, unknown provider)
- [x] `CostCalculator` implementieren (TDD) – `src/cost/calculator.rs`
  - Cost pro 1K Tokens pro Provider/Model (OpenAI GPT-4/GPT-3, Anthropic Claude; unbekannt = 0)
  - Input-Tokens + Output-Tokens → Total-Cost (Dollar)
  - `total_cost(input_tokens, output_tokens, provider, model)` → f64
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test cost_calculator`)

### 9.3 Budget Management

**Abhängigkeiten**: 9.2 (Cost Calculation)

#### 9.3.1 Budget-Manager
- [x] Tests für Budget-Manager schreiben (`tests/unit/budget_manager_test.rs`: zero usage, add_usage, is_over_limit, remaining, check_alerts OverLimit/NearLimit)
- [x] `BudgetManager` implementieren (TDD) – `src/cost/budget.rs`
  - Budget-Limits verwalten (`new(limit)`, `get_limit()`)
  - Budget-Usage tracken (`add_usage(amount)`, `get_usage()`, `remaining()`)
  - Budget-Limit-Erkennung (`is_over_limit()`)
  - Budget-Alerts (`check_alerts()` → `BudgetAlert::OverLimit` / `NearLimit` bei ≥ 80 %)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test budget_manager`)

---

## Phase 10: Fallback System

### 10.1 Cloud-to-Local Fallback

**Abhängigkeiten**: 7.2 (Model Selector), 6.1 (Model Registry)

#### 10.1.1 Fallback-Manager
- [x] Tests für Fallback-Manager schreiben (`tests/unit/fallback_manager_test.rs`)
- [x] `FallbackManager` implementieren (TDD) – `src/fallback/manager.rs`
  - Cloud-Limit-Erkennung via `CloudLimitDetector` Trait (injizierbar für Tests/Provider)
  - Automatischer Fallback zu lokalem LLM (`get_fallback_model(local_candidates, detector)`)
  - Bestes lokales LLM identifizieren über `ModelSelector` (Multi-Faktor-Bewertung)
  - [ ] Netzwerk-LLM-Suche via Einherjar Protocol – optional / spätere Phase
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test fallback_manager`)

### 10.2 Fallback-Notifications

**Abhängigkeiten**: 10.1 (Fallback-Manager)

#### 10.2.1 Notification-Generator
- [x] Tests für Notification-Generator schreiben (`tests/unit/fallback_notification_test.rs`)
- [x] `FallbackNotificationGenerator` implementieren (TDD) – `src/fallback/notification.rs`
  - Benachrichtigungs-Text generieren (`generate_text(reason)`; siehe README: CloudLimitReached, CloudProviderUnavailable, NetworkLlmUsed, LocalLlmUsed)
  - Notification an Odin senden via `NotificationSender` Trait (für TTS via Muninn; injizierbar)
  - User-Einstellungen berücksichtigen (`generate_and_send(..., notifications_enabled)` → None wenn deaktiviert)
  - `FallbackNotificationReason` Enum für alle Benachrichtigungs-Typen
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test fallback_notification`)

### 10.3 Automatic Return to Cloud LLM

**Abhängigkeiten**: 10.1 (Fallback-Manager), 9.3 (Budget-Manager)

#### 10.3.1 Budget-Reset-Listener
- [x] Tests für Budget-Reset-Listener schreiben (`tests/unit/budget_reset_listener_test.rs`)
- [x] `BudgetResetListener` implementieren (TDD) – `src/fallback/budget_reset.rs`
  - Yggdrasil-Integration ruft `notify_reset()` bei Budget-Reset auf; Listener hält `BudgetResetHandler` Trait
  - Automatische Rückkehr zu Cloud-LLM: Handler (`on_budget_reset()`) wird bei jedem Reset aufgerufen (z. B. Provider umschalten)
  - `BudgetResetHandler` Trait für injizierbare Rückkehr-Logik (Tests/Integration)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test budget_reset_listener`)

---

## Phase 11: Caching System

### 11.1 Response-Caching

**Abhängigkeiten**: 8.1 (Prompt Formatting)

#### 11.1.1 Cache-Manager
- [x] Tests für Cache-Manager schreiben (`tests/unit/cache_manager_test.rs`)
- [x] `CacheManager` implementieren (TDD) – `src/cache/manager.rs`
  - Responses für Prompts cachen (`insert(prompt, response)`, `get(prompt) -> Option<String>`)
  - Cache-Key-Generierung (DefaultHasher über Prompt-String, deterministisch)
  - Cache-Hit/Miss-Handling (get liefert None bei unbekannt/abgelaufen)
  - TTL-basierte Expiration (konfigurierbar pro Cache; abgelaufene Einträge werden bei get entfernt)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test cache_manager`)

### 11.2 Cache-Invalidation

**Abhängigkeiten**: 11.1 (Response-Caching)

#### 11.2.1 Cache-Invalidator
- [x] Tests für Cache-Invalidation schreiben (`tests/unit/cache_invalidator_test.rs`)
- [x] `CacheInvalidator` implementieren (TDD) – `src/cache/invalidator.rs`
  - Event-basierte Invalidation (`invalidate_on_event(cache, event)`; `InvalidationEvent::ModelUpdate`, `ProviderStatusChange`)
  - Timeout-basierte Invalidation (Fallback: `invalidate_on_timeout(cache)` invalidiert, wenn seit letzter Invalidierung Timeout abgelaufen)
  - `CacheManager::invalidate_all()` für vollständige Leerung (von Invalidator genutzt)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test cache_invalidator`)

---

## Phase 12: Streaming Support

### 12.1 LLM-Response-Streaming

**Abhängigkeiten**: 3.1 (Provider-Interface), 2.2.2 (Wolf Service)

#### 12.1.1 Streaming-Manager
- [x] Tests für Streaming-Manager schreiben (`tests/unit/streaming_manager_test.rs`)
- [x] `StreamingManager` implementieren (TDD) – `src/streaming/manager.rs`
  - Chunk-basiertes Streaming (`collect_chunks(chunks)` sammelt `Result<String, E>` zu einem String)
  - Error-Handling bei Streaming-Fehlern (erster Fehler wird als `StreamingError` zurückgegeben; `user_message()` für Nutzer-Text)
  - `StreamingError` (StreamError) mit `From<String>` für Provider-Fehler
  - [ ] Integration mit Provider-Stream (z. B. `stream_text()`) – optional / spätere Phase
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test streaming_manager`)

### 12.2 Video-Stream-Processing

**Abhängigkeiten**: 2.2.3 (Vision Service)

#### 12.2.1 Video-Stream-Processor
- [x] Tests für Video-Stream-Processing schreiben (`tests/unit/video_stream_processor_test.rs`)
- [x] `VideoStreamProcessor` implementieren (TDD) – `src/streaming/video_stream.rs`
  - Video-Stream-Chunks verarbeiten (`push_chunk(chunk)`; `VideoStreamChunk`, konfigurierbar `chunks_per_frame`)
  - Frame-Extraction aus Video-Stream (N Chunks = 1 Frame; Buffer wird zu Frame-Daten konkateniert)
  - Vision-Model für Frame-Analyse nutzen (`FrameAnalyzer` Trait; injizierbar für Tests/Production)
  - Streaming-Results zurückgeben (`push_chunk` → `Option<Result<VideoAnalysisChunk, VideoStreamError>>`; `VideoAnalysisChunk` mit frame_index, analysis)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test video_stream_processor`)

---

## Phase 13: Request Queuing & Prioritization

### 13.1 Request-Queue

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 13.1.1 Request-Queue-Manager
- [x] Tests für Request-Queue schreiben (`tests/unit/request_queue_manager_test.rs`)
- [x] `RequestQueueManager` implementieren (TDD) – `src/queue/manager.rs`
  - Requests in Queue bei hoher Last (`enqueue(item)` mit optionaler Kapazitätsbegrenzung; `QueueFullError` bei voll)
  - FIFO-Queue-Processing (`dequeue()` → erstes eingereihtes Element)
  - Queue-Backlog-Handling (`len()`, `backlog_len()`, `is_empty()`; `with_capacity(max)` für begrenzte Queue)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test request_queue_manager`)

### 13.2 Priority-Queue

**Abhängigkeiten**: 13.1 (Request-Queue)

#### 13.2.1 Priority-Queue-Manager
- [x] Tests für Priority-Queue schreiben (`tests/unit/priority_queue_manager_test.rs`)
- [x] `PriorityQueueManager` implementieren (TDD) – `src/queue/priority.rs`
  - Requests mit Priorität versehen (`enqueue(item, priority)`; höhere Zahl = höhere Priorität)
  - Priority-basierte Queue-Processing (`dequeue()` liefert Element mit höchster Priorität)
  - High-Priority-Requests bevorzugen; bei gleicher Priorität FIFO (insert_order)
  - Optional `with_capacity(max)` für Kapazitätsbegrenzung (wie 13.1.1)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test priority_queue_manager`)

---

## Phase 14: API-Key-Management & Secure Storage

### 14.1 API-Key-Storage

**Abhängigkeiten**: 1.3 (Settings-System)

#### 14.1.1 Secure-Key-Storage
- [x] Tests für Secure-Key-Storage schreiben (`tests/unit/secure_key_storage_test.rs`)
- [x] `SecureKeyStorage` implementieren (TDD) – `src/keys/storage.rs`
  - API-Keys speichern/laden über `SecureKeyBackend` Trait (OS-Secure-Storage oder In-Memory für Tests)
  - `store_key(provider_id, api_key)`, `load_key(provider_id) -> Option<String>`
  - `InMemoryKeyBackend` für Tests; Production-Backend (OS-Keychain, etc.) kann später implementiert werden
  - Encryption: Backend verantwortlich (OS-Keychain verschlüsselt at rest); `KeyStorageError` für Fehler
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test secure_key_storage`)

### 14.2 Key-Rotation

**Abhängigkeiten**: 14.1 (API-Key-Storage)

#### 14.2.1 Key-Rotation-Manager
- [x] Tests für Key-Rotation schreiben (`tests/unit/key_rotation_manager_test.rs`)
- [x] `KeyRotationManager` implementieren (TDD) – `src/keys/rotation.rs`
  - Alte API-Keys entfernen (nutzt `SecureKeyStorage::remove_key`; Backend-Trait um `delete` erweitert)
  - Neue API-Keys hinzufügen (`rotate(storage, provider_id, new_api_key)` speichert neuen Key)
  - Rotation-Workflow: `rotate(&mut storage, provider_id, new_key)` → remove_key, dann store_key
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test key_rotation_manager`)

---

## Phase 15: Performance Monitoring & Metrics

### 15.1 Metrics Collector

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 15.1.1 Performance-Metrics-Collector
- [x] Tests für Metrics-Collector schreiben (`tests/unit/metrics_collector_test.rs`)
- [x] `MetricsCollector` implementieren (TDD) – `src/metrics/collector.rs`
  - Response-Zeiten tracken (`record_response(response_time_ms)`, Snapshot: total/min/max/avg)
  - Durchsatz-Grundlage (Request-Count in Snapshot; Durchsatz extern aus count/Zeitfenster ableitbar)
  - Latency-Metriken (min, max, average_response_time_ms in `MetricsSnapshot`)
  - [ ] Resource-Usage tracken – optional / spätere Phase
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test metrics_collector`)

### 15.2 Model-Performance-Tracker

**Abhängigkeiten**: 15.1 (Metrics Collector)

#### 15.2.1 Model-Performance-Tracker
- [x] Tests für Model-Performance-Tracker schreiben (`tests/unit/model_performance_tracker_test.rs`)
- [x] `ModelPerformanceTracker` implementieren (TDD) – `src/metrics/model_tracker.rs`
  - Tokens pro Sekunde tracken (`ModelMetrics::average_tokens_per_second()`)
  - Model-Response-Zeiten tracken (`record_response(model_id, response_time_ms, tokens, success)`, `average_response_time_ms()`)
  - Model-Verfügbarkeit tracken (`availability()` = success_count/request_count)
  - `ModelMetrics` pro Model (request_count, success_count, total_response_time_ms, total_tokens)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test model_performance_tracker`)

---

## Phase 16: Error Handling & Retry Logic

### 16.1 Error-Handler

**Abhängigkeiten**: 3.1 (Provider-Interface)

#### 16.1.1 Provider-Error-Handler
- [x] Tests für Error-Handler schreiben (`tests/unit/provider_error_handler_test.rs`)
- [x] `ProviderErrorHandler` implementieren (TDD) – `src/error_handling/provider_handler.rs`
  - Provider-spezifische Fehler behandeln (`handle_llm(&LLMError)` → ModelNotAvailable → Unavailable, ProcessingFailed → Internal)
  - gRPC-Status-Codes (`GrpcStatusCode`: Internal, Unavailable, InvalidArgument; für Mapping zu tonic::Code)
  - Generische Fehler (`handle_generic(msg)` → Internal); Logging durch Aufrufer (Rückgabe Code + Nachricht)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test provider_error_handler`)

### 16.2 Retry-Manager

**Abhängigkeiten**: 16.1 (Error-Handler)

#### 16.2.1 Retry-Manager
- [x] Tests für Retry-Manager schreiben (`tests/unit/retry_manager_test.rs`)
- [x] `RetryManager` implementieren (TDD) – `src/error_handling/retry.rs`
  - Exponential-Backoff-Retry (`delay_for_attempt(attempt)` = base_delay × 2^attempt, Cap 60s)
  - Max-Retry-Count (`should_retry(attempt)` = attempt < max_retries)
  - Retry-Delay berechnen (`new(max_retries, base_delay_ms)`, `delay_for_attempt` in Duration)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test retry_manager`)

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
- [x] Structured-Logging konfigurieren (tracing) – `src/logging/setup.rs`, `init_logging(default_filter)`; main.rs nutzt zentrales Setup
- [x] LLM-specific Log-Levels (via `RUST_LOG`, z. B. `geri::llm=debug`; Default `info` wenn nicht gesetzt)
- [ ] Log-Rotation konfigurieren – optional / spätere Phase (z. B. tracing-appender)

### 18.2 Context-Tracking

**Abhängigkeiten**: 18.1 (Structured Logging)

#### 18.2.1 Context-Logger
- [x] Tests für Context-Logging schreiben (`tests/unit/context_logger_test.rs`)
- [x] `ContextLogger` implementieren (TDD) – `src/logging/context_logger.rs`
  - Request-Context in Logs (`add_field(key, value)`, `to_log_string()` für strukturierte Log-Zeile)
  - Trace-IDs für Request-Tracking (`new(trace_id)`, `trace_id()`)
- [ ] Tests ausführen und bestehen (lokal/CI: `cargo test context_logger`)

---

## Phase 19: Documentation

### 19.1 API Documentation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 19.1.1 gRPC Service Documentation
- [x] gRPC-Service-Documentation erstellen (`docs/GRPC_SERVICES.md`)
- [x] Wolf-Service-API dokumentieren (ProcessPrompt, ProcessPromptRequest/Response)
- [x] Vision-Service-API dokumentieren (ProcessVision, ProcessVisionRequest/Response)

### 19.2 Provider Documentation

**Abhängigkeiten**: 4.1 (llama.cpp Provider), 5.1-5.3 (Cloud Providers)

#### 19.2.1 Provider-Integration-Guide
- [x] Provider-Integration-Guide erstellen (`docs/PROVIDER_INTEGRATION.md`)
- [x] API-Key-Setup dokumentieren (SecureKeyStorage, KeyRotationManager, Backend; keine Keys in Config)
- [x] Local-LLM-Setup dokumentieren (config: default_local_llm, vision_model; geplant: llama.cpp, BitNet.cpp)

---

## Phase 20: Testing & Quality Assurance

### 20.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 20.1.1 End-to-End Tests
- [x] E2E-Tests für LLM-Workflows schreiben (`tests/e2e_llm_workflow_test.rs`)
  - RAG-Context-Integration → LLM-Call → Response (`e2e_rag_context_integration_to_llm_response`: ProcessPrompt mit context)
  - Cloud-Limit-Fallback → Local-LLM (`e2e_cloud_limit_fallback_returns_local_model`: FallbackManager + lokale Kandidaten)
  - Model-Selection (best by score) (`e2e_model_selection_selects_best_by_efficiency_score`: ModelSelector wählt bestes Model)
  - WolfRequest → LLM-Call → WolfResponse (bereits in `grpc_server_test.rs`; RAG-Variante oben)
- [ ] E2E-Tests ausführen und bestehen (lokal/CI: `cargo test e2e_`)

### 20.2 Performance Testing

**Abhängigkeiten**: 15.1 (Metrics Collector)

#### 20.2.1 Performance Test Suite
- [x] Performance-Tests schreiben (`tests/performance_test.rs`)
  - Response-Time-Tests (< 500ms für lokale LLMs, < 2s für Cloud LLMs; simuliert via MetricsCollector)
  - Throughput-Tests (viele record_response in kurzer Zeit)
  - Streaming-Performance-Tests (StreamingManager collect_chunks mit vielen Chunks)
- [x] Performance-Tests ausführen und bestehen (Container: `cargo test --release --test performance_test`) ✅

### 20.3 Load Testing

**Abhängigkeiten**: 13.1 (Request-Queue)

#### 20.3.1 Load Test Suite
- [x] Load-Tests schreiben (`tests/load_test.rs`)
  - High-Concurrency-Tests (parallele Enqueue/Dequeue, LoadBalancer record_request)
  - Queue-Backlog-Tests (RequestQueueManager with_capacity, fill, dequeue FIFO)
  - Load-Balancing-Tests (überlasteter Provider wird abgewertet, Load pro Provider)
- [x] Load-Tests ausführen und bestehen (Container: `cargo test --release --test load_test`) ✅

---

## Verbleibende Punkte (Übersicht)

- [ ] **Phase 1.2.1–1.2.2** (Struktur/CI optional)
- [ ] **Phase 4** llama.cpp / BitNet.cpp (Local-LLM-Provider, Vision)
- [ ] **Phase 6** (ohne 6.1–6.2.1: Model-Discovery, weitere Model-Registry)
- [ ] **Phase 9.1**, **Phase 12.2–13**, **Phase 17** (optionale Erweiterungen)
- [x] **Phase 20** E2E-/Performance-/Load-Tests ausführen und bestehen (Container: alle 200+ Tests inkl. load_test, performance_test) ✅

*(Cloud-LLM-Provider, Vision, Efficiency-Score, LoadBalancer, Streaming, Security, Metrics: implementiert.)*

- [x] **Container-Build + Tests** – Lib-Compile behoben (async_trait/LLMProvider, SecureKeyBackend delete+Debug, Google GenerationConfig, main.rs/model_registry, gRPC- und Unit-Tests). 200 Tests im Container grün ✅.

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
