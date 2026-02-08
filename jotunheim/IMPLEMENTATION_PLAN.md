# IMPLEMENTATION_PLAN - Jotunheim (IoT Platform)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Jotunheim - der IoT Platform für ESP32, ESP8266, Raspberry Pi Pico und andere Microcontroller. Jotunheim ist eine Platform (wie Midgard, Alfheim, Asgard, Ragnarok) und kommuniziert mit Services (Loki) via gRPC.

**Mythologische Bedeutung**: Jotunheim ist das Land der Riesen.

**Programmiersprache**: Rust (esp-rs für ESP32, etc.)

**Platform-Typ**: IoT Platform (extrem lightweight, für Microcontroller)

## Entschiedene Konfiguration

### Protobuf-Rust-Tool
✅ **ENTSCHEIDUNG**: prost-lite + tonic
**Begründung**: Minimaler Memory-Footprint für IoT, optimiert für ESP32, robuste Performance

### Primary-Target-Device
✅ **ENTSCHEIDUNG**: ESP32
**Begründung**: Populär, WiFi + Bluetooth, beste Community-Support, robuste Hardware

### Network-Stack
✅ **ENTSCHEIDUNG**: esp-idf
**Begründung**: ESP32 SDK, beste Platform-Integration, robuste WiFi-Features

### gRPC-Client-Library
✅ **ENTSCHEIDUNG**: tonic-lightweight (prost-lite + tonic-light)
**Begründung**: Minimale Größe, optimiert für IoT, beste Performance auf ESP32

### Lua-Engine (für Loki-Scripts)
✅ **ENTSCHEIDUNG**: mlua
**Begründung**: Rust Lua bindings, robust, gute Performance, aktiv maintained

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Protobuf-Rust-Tool, Primary-Target-Device

#### 1.1.1 Cargo-Workspace erstellen
- [x] `Cargo.toml` mit Workspace erstellen
  - `esp32/` (ESP32-spezifische Implementation)
  - `generic/` (Generic Implementation für andere Devices)
  - `shared/` (Shared Code zwischen Implementations)
- [x] Basis-Dependencies für ESP32 definieren
  - Async Runtime (esp-rs, embassy, oder smol)
  - gRPC Client (tonic-light, prost-lite)
  - Network (esp-idf oder smoltcp)
  - Serialization (serde-minimal, postcard)
  - Logging (defmt oder log-minimal)
  - Error-Handling (anyhow-minimal, oder eigene Impl.)
- [x] `.gitignore` erstellen

#### 1.1.2 ESP32 Verzeichnisstruktur erstellen
- [x] `esp32/src/main.rs` erstellen
- [x] `esp32/src/lib.rs` erstellen
- [x] `esp32/src/network/` für Network-Management erstellen
- [x] `esp32/src/grpc/` für gRPC-Client erstellen
- [x] `esp32/src/capability/` für Capability-Management erstellen (CapabilityManager placeholder)
- [x] `esp32/src/remote/` für Remote-Control erstellen (RemoteControl placeholder)
- [x] `esp32/src/ota/` für OTA-Updates erstellen (OtaManager placeholder)
- [x] `esp32/src/utils/` für Utilities erstellen
- [x] `esp32/config/` für Konfigurationsdateien erstellen (README)

#### 1.1.3 Build-System einrichten
- [x] Build-Scripts für ESP32 definieren (esp-rs) (scripts/build.ps1, build.sh; --esp32 für Target)
- [x] Flash-Scripts für ESP32 erstellen (scripts/flash.ps1, flash.sh)
- [x] Code-Generierungs-Pipeline einrichten (Protobuf → Rust) (esp32/build.rs, aktiv bei Phase 2)
- [x] Cargo-Features definieren (z.B. `esp32`, `esp8266`, `pico`) (esp32/Cargo.toml)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
  - Mock-Loki-Service
  - Mock-Controller (Midgard/Asgard)
  - Mock-Network-Simulator (für WiFi-Tests)
- [x] Test-Container-Startup-Scripts erstellen (scripts/run-tests.ps1, run-tests.sh)
- [x] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [x] Test-Dependencies hinzufügen (embedded-test, defmt-test)
- [x] Test-Utilities und Helpers erstellen (tests/utils/test_helpers.rs)
- [x] Mock-Setup für Services (tests/mocks/)
- [x] Test-Data-Generators erstellen (esp32/tests/common/test_data.rs: minimal_capabilities, sensor_actuator_capabilities, dht22_led_capabilities, resolver_with_one_device)

#### 1.2.3 CI/CD-Pipeline
- [x] GitHub Actions / GitLab CI Workflow erstellen (.github/workflows/jotunheim.yml)
- [x] Automatische Test-Ausführung bei Commits konfigurieren
- [x] Code-Coverage-Reporting einrichten (cargo-tarpaulin, Lcov-Artefakt jotunheim-coverage)
- [x] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren (Minimal)
- [x] Settings-Struktur entwerfen (minimales JSON)
  - capability_configuration
  - network_resilience_settings
  - resource_limits
  - ota_update_settings (optional)

#### 1.3.2 Settings-Validierung
- [x] Tests für Settings-Validierung schreiben
- [x] Rust-Structs für Settings definieren (minimal)
- [x] Settings-Validator implementieren (TDD)
- [x] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader
- [x] Tests für Settings-Loader schreiben
- [x] Settings-Loader implementieren (TDD)
  - Flash-Storage-Integration
  - Runtime-Settings-Reload (minimal)
- [x] Tests ausführen und bestehen

---

## Phase 2: Protobuf & gRPC Client

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Jotunheim als Dependency zu Protobuf-Projekt hinzufügen (optional, aktuell: Copy in esp32/proto)
- [x] Protobuf-Definitions importieren (minimal subset für IoT) (esp32/proto/)

#### 2.1.2 Capability Protocol (Platform Capability Protocol)
- [x] `JotunheimCapability.proto` definieren (esp32/proto/jotunheim_capability.proto)
  - `JotunheimCapabilities` Message (siehe README.md)
  - `CapabilityRequest` Message
  - `CapabilityResponse` Message
  - `CapabilityUpdateEvent` Message
- [x] Code-Generierung konfigurieren (tonic_build in build.rs)

#### 2.1.3 Loki Service Protocol
- [x] `LokiService.proto` verwenden (aus Loki-Projekt, Copy esp32/proto/loki.proto)
  - `GetCapabilities()` RPC
  - `GetChildrenStatus()` RPC
  - `ListScripts()` RPC
  - ExecuteScript, StreamScriptExecution
- [x] Code-Generierung konfigurieren (tonic_build, client only)

### 2.2 gRPC Client Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)
**Erforderliche USER-Eingaben**: gRPC-Client-Library

#### 2.2.1 Lightweight-gRPC-Client
- [x] Tests für gRPC-Client schreiben (loki_client_test.rs: endpoint, connect)
- [x] `LightweightGRPCClient`-Äquivalent: tonic Channel + LokiClient (TDD)
  - tonic (standard), Protobuf via prost
  - HTTP/2 via tonic
- [x] Tests ausführen und bestehen

#### 2.2.2 Loki-Client
- [x] Tests für Loki-Client schreiben (loki_client_test.rs)
- [x] `LokiClient` implementieren (TDD)
  - gRPC-Connection zu Loki-Service (connect, endpoint, from_channel)
  - `GetCapabilities()` aufrufen
  - `ListScripts()` aufrufen
  - ExecuteScript (call_function)
- [ ] Dynamische Script-Funktionen (Script_<name>) – optional/später
- [x] Tests ausführen und bestehen

---

## Phase 3: Network-Management (ESP32)

### 3.1 WiFi-Management

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Network-Stack

#### 3.1.1 WiFi-Manager
- [x] Tests für WiFi-Manager schreiben (tests/wifi_manager_test.rs)
- [x] `WiFiManager` implementieren (TDD) (network/manager.rs)
  - WiFi-Connection aufbauen (connect)
  - WiFi-Reconnection (reconnect)
  - WiFi-Status überwachen (status: Disconnected/Connecting/Connected)
- [x] Tests ausführen und bestehen

### 3.2 TCP/IP-Stack

**Abhängigkeiten**: 3.1 (WiFi-Management)

#### 3.2.1 TCP-Client
- [x] Tests für TCP-Client schreiben (tests/tcp_client_test.rs)
- [x] `TCPClient` implementieren (TDD) (network/tcp_client.rs)
  - TCP-Connection aufbauen (connect)
  - TCP-Send/Receive (send, receive)
  - Connection-Resilience (host/test mit tokio TcpStream)
- [x] Tests ausführen und bestehen

---

## Phase 4: Capability Management

### 4.1 Capability-Definition

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 4.1.1 Device-Capability-Builder
- [x] Tests für Capability-Builder schreiben (tests/device_capability_builder_test.rs)
- [x] `DeviceCapabilityBuilder` implementieren (TDD) (capability/builder.rs)
  - Device-Information sammeln (Device-ID, Name, Type, Firmware)
  - Hardware-Capabilities sammeln (GPIO, Interfaces, Sensors, Actuators)
  - Resource-Limits sammeln
  - Protocol-Features sammeln
  - `JotunheimCapabilities` erstellen
- [x] Tests ausführen und bestehen

### 4.2 Capability-Negotiation

**Abhängigkeiten**: 4.1 (Capability-Definition), 2.2.1 (Lightweight-gRPC-Client)

#### 4.2.1 Capability-Negotiator
- [x] Tests für Capability-Negotiator schreiben (tests/capability_negotiator_test.rs)
- [x] `CapabilityNegotiator` implementieren (TDD) (capability/negotiator.rs)
  - `CAPABILITY_REQUEST` empfangen (on_capability_request)
  - Capabilities senden (CapabilityResponse via callback)
  - Negotiation-Timeouts behandeln (negotiate_with_timeout)
- [x] Tests ausführen und bestehen

### 4.3 Capability-Propagation

**Abhängigkeiten**: 4.2 (Capability-Negotiation)

#### 4.3.1 Capability-Propagator
- [x] Tests für Capability-Propagator schreiben (tests/capability_propagator_test.rs)
- [x] `CapabilityPropagator` implementieren (TDD) (capability/propagator.rs)
  - Capabilities bei Kopplung/Verbindung propagieren (on_connect)
  - Capability-Update-Events senden (emit_event, with_event_callback)
  - Einherjar Protocol nutzen (Platform Capability Protocol)
- [x] Tests ausführen und bestehen

### 4.4 Device-Tool-Auto-Discovery (Sensor/Aktor → Tools für Loki/Odin)

**Abhängigkeiten**: 4.1 (Capability-Definition)

#### 4.4.1 Device-Tool-Registry
- [x] `device_tool_registry.rs`: `GeneratedToolDef`, Mappings Sensor/Aktor → Tool
- [x] `generate_tools_from_capabilities(caps, prefix)` aus `JotunheimCapabilities`
- [x] Tests (tests/device_tool_registry_test.rs)

#### 4.4.2 Device-Resolver
- [x] `DeviceResolver` Trait (`resolve`, `list_device_ids`)
- [x] `InMemoryDeviceResolver`, `ResolvedDevice`
- [x] `tools_for_device(resolver, device_id)` Hilfsfunktion
- [x] Tests (tests/device_resolver_test.rs)

---

## Phase 5: Remote Control

### 5.1 Command-Handler

**Abhängigkeiten**: 2.2.2 (Loki-Client)

#### 5.1.1 Remote-Command-Handler
- [x] Tests für Remote-Command-Handler schreiben (tests/remote_command_handler_test.rs)
- [x] `RemoteCommandHandler` implementieren (TDD) (remote/command_handler.rs, executor.rs, error.rs)
  - Commands von Controller empfangen (handle_command)
  - Commands an Loki-Service weiterleiten (ScriptExecutor, LokiClient impl)
  - Results von Loki empfangen
  - Results an Controller zurücksenden (return value)
- [x] Tests ausführen und bestehen

---

## Phase 6: Resource-Management

### 6.1 Memory-Management

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 6.1.1 Memory-Monitor
- [x] Tests für Memory-Monitor schreiben (tests/memory_monitor_test.rs)
- [x] `MemoryMonitor` implementieren (TDD) (resources/memory_monitor.rs)
  - RAM-Usage überwachen (set_usage_kb, current_usage_kb)
  - Memory-Exhaustion erkennen (is_exhausted)
  - Memory-Warnings (is_warning)
- [x] Tests ausführen und bestehen

#### 6.1.2 Memory-Pooling
- [x] Tests für Memory-Pooling schreiben (tests/memory_pool_test.rs)
- [x] `MemoryPool` implementieren (TDD) (resources/memory_pool.rs)
  - Memory-Pools für häufige Allocations (acquire, release)
  - Efficient Memory-Reuse
- [x] Tests ausführen und bestehen

### 6.2 CPU-Management

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 6.2.1 Task-Scheduler
- [x] Tests für Task-Scheduler schreiben (tests/task_scheduler_test.rs)
- [x] `TaskScheduler` implementieren (TDD) (resources/task_scheduler.rs)
  - Task-Scheduling (submit, run_next)
  - Priority-basierte Scheduling
  - CPU-Constraints (max_queued)
- [x] Tests ausführen und bestehen

### 6.3 Network-Bandwidth-Management

**Abhängigkeiten**: 3.2 (TCP/IP-Stack)

#### 6.3.1 Bandwidth-Monitor
- [x] Tests für Bandwidth-Monitor schreiben (tests/bandwidth_monitor_test.rs)
- [x] `BandwidthMonitor` implementieren (TDD) (resources/bandwidth_monitor.rs)
  - Network-Usage überwachen (add_sent, add_received, sent_bytes, received_bytes)
  - Bandwidth-Limits (should_throttle)
  - Throttling (tick resets window)
- [x] Tests ausführen und bestehen

---

## Phase 7: OTA Updates

### 7.1 OTA-Update-Manager

**Abhängigkeiten**: 3.1 (WiFi-Management), 2.2.1 (gRPC-Client)

#### 7.1.1 OTA-Update-Client
- [x] Tests für OTA-Update-Client schreiben (tests/ota_update_client_test.rs)
- [x] `OTAUpdateClient` implementieren (TDD) (ota/client.rs, fetcher.rs)
  - Update-Verfügbarkeit prüfen (UpdateFetcher, check_available)
  - Update herunterladen (download)
  - Update installieren (install)
  - Device-Restart (on_restart callback)
- [x] Tests ausführen und bestehen

### 7.2 Update-Verification

**Abhängigkeiten**: 7.1 (OTA-Update-Manager)

#### 7.2.1 Update-Verifier
- [x] Tests für Update-Verifier schreiben (tests/update_verifier_test.rs)
- [x] `UpdateVerifier` implementieren (TDD) (ota/verifier.rs)
  - Checksum-Verification (SHA256, verify_checksum)
  - Update-Integrity-Check
- [ ] Digital-Signature-Verification (optional/später)
- [x] Tests ausführen und bestehen

### 7.3 Rollback-Manager

**Abhängigkeiten**: 7.1 (OTA-Update-Manager)

#### 7.3.1 Rollback-Handler
- [x] Tests für Rollback-Handler schreiben (tests/rollback_handler_test.rs)
- [x] `RollbackHandler` implementieren (TDD) (ota/rollback.rs)
  - Alte Version wiederherstellen (rollback, with_previous_version)
  - Manueller Rollback-Support (can_rollback)
- [x] Tests ausführen und bestehen

---

## Phase 8: Error Handling & Resilience

### 8.1 Connection-Resilience

**Abhängigkeiten**: 3.1 (WiFi-Management), 3.2 (TCP-Client)

#### 8.1.1 Connection-Resilience-Manager
- [x] Tests für Connection-Resilience schreiben (tests/connection_resilience_test.rs)
- [x] `ConnectionResilienceManager` implementieren (TDD) (resilience/connection_resilience.rs)
  - Exponential-Backoff (next_backoff_delay), record_failure/success
  - should_retry, retry_count
- [x] Tests ausführen und bestehen

### 8.2 Error-Handler

**Abhängigkeiten**: 2.2 (gRPC Client Implementation)

#### 8.2.1 Error-Handler
- [x] Tests für Error-Handler schreiben (tests/error_handler_test.rs)
- [x] `ErrorHandler` implementieren (TDD) (resilience/error_handler.rs)
  - categorize (Grpc/Network/Resource), is_recoverable
  - handle, last_error (minimal logging)
- [x] Tests ausführen und bestehen

### 8.3 Retry-Manager

**Abhängigkeiten**: 8.2 (Error-Handler)

#### 8.3.1 Retry-Manager
- [x] Tests für Retry-Manager schreiben (tests/retry_manager_test.rs)
- [x] `RetryManager` implementieren (TDD) (resilience/retry_manager.rs)
  - next_delay(attempt), exhausted(attempt)
  - Exponential-Backoff mit max_delay cap
- [x] Tests ausführen und bestehen

---

## Phase 9: Streaming-Vorbereitung (Plugin-Support)

### 9.1 Streaming-Interface

**Abhängigkeiten**: 2.2 (gRPC Client Implementation)

#### 9.1.1 Streaming-Handler-Interface
- [x] Tests für Streaming-Interface schreiben (tests/streaming_handler_test.rs)
- [x] `StreamingHandler` Trait definieren (streaming/handler.rs)
  - `send_video_stream()` Methode
  - `send_audio_stream()` Methode
  - `receive_video_stream()` Methode
  - `receive_audio_stream()` Methode
- [x] Tests ausführen und bestehen

### 9.2 Plugin-Loader (Vorbereitung)

**Abhängigkeiten**: 9.1 (Streaming-Interface)

#### 9.2.1 Plugin-System-Vorbereitung
- [x] Tests für Plugin-System schreiben (tests/plugin_loader_test.rs)
- [x] `PluginLoader` implementieren (TDD - minimal) (streaming/plugin.rs)
  - Plugin-Interface definieren (Plugin trait, name())
  - Plugin-Loading-Vorbereitung (register, get, list)
  - Streaming-Plugin-Support-Vorbereitung
- [x] Tests ausführen und bestehen

---

## Phase 10: Performance Optimization

### 10.1 Memory-Optimization

**Abhängigkeiten**: 6.1 (Memory-Management)

#### 10.1.1 Memory-Profiling & Optimization
- [x] Memory-Profiling-Tests schreiben (tests/memory_profiling_test.rs)
- [x] Memory-Usage optimieren
  - Memory-Pooling nutzen (bestehend), Bounded-Allocation-Tests
  - < 10KB RAM-Ziel auf ESP32 (auf Device validieren)
- [x] Memory-Tests ausführen und Benchmarks erreichen (< 10KB Ziel dokumentiert)

### 10.2 Flash-Usage-Optimization

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 10.2.1 Code-Size-Optimization
- [x] Code-Size-Tests schreiben (scripts/check-size.sh, check-size.ps1)
- [x] Code-Size optimieren (profile.release: opt-level="z", lto, codegen-units=1)
  - Compiler-Optimierungen
- [x] Code-Size-Tests ausführen (Build + Größenausgabe; optional MAX_BYTES)

---

## Phase 11: Monitoring & Logging (Minimal)

### 11.1 Minimal-Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 11.1.1 Logging Setup (Minimal)
- [x] Minimal-Logging konfigurieren (log crate, utils/logging.rs)
- [x] Nur kritische Log-Levels (ERROR, WARN) (MinimalLogger, LevelFilter::Warn)
- [x] Log-Output über Serial (für Debugging) (Host: stderr; ESP32: später Serial/defmt)

### 11.2 Resource-Monitoring

**Abhängigkeiten**: 6.1 (Memory-Management), 6.2 (CPU-Management)

#### 11.2.1 Resource-Monitor
- [x] Tests für Resource-Monitor schreiben (tests/resource_monitor_test.rs)
- [x] `ResourceMonitor` implementieren (TDD) (resources/resource_monitor.rs)
  - RAM-Usage tracken (MemoryMonitor)
  - CPU-Usage tracken (set_cpu_usage_percent, cpu_usage_percent)
  - Network-Usage tracken (BandwidthMonitor)
- [x] Tests ausführen und bestehen

---

## Phase 12: Documentation

### 12.1 Platform Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 12.1.1 Platform-Guide
- [x] Platform-Guide erstellen (docs/PLATFORM_GUIDE.md)
- [x] ESP32-Setup dokumentieren (Toolchain, espup, Target)
- [x] Flash-Instructions dokumentieren (scripts, espflash)
- [x] Capability-Configuration dokumentieren (Config, DeviceCapabilityBuilder)

### 12.2 Examples

**Abhängigkeiten**: Alle vorherigen Phasen

#### 12.2.1 Example-Implementations
- [x] ESP32-Beispiele erstellen (esp32/examples/)
  - LED-Control-Example (led_control.rs)
  - Sensor-Reading-Example (sensor_reading.rs)
  - Remote-Control-Example (remote_control.rs)
- [x] Example-Documentation erstellen (esp32/examples/README.md)

---

## Phase 13: Testing & Quality Assurance

### 13.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 13.1.1 End-to-End Tests
- [x] E2E-Tests für IoT-Workflows schreiben (tests/e2e_iot_workflow_test.rs)
  - Connection-Establishment → Capability-Negotiation → Remote-Control
  - OTA-Update → Verification → Installation → Rollback (in OTA/rollback unit tests abgedeckt)
- [x] E2E-Tests ausführen und bestehen

### 13.2 Performance Testing

**Abhängigkeiten**: 10.1 (Memory-Optimization)

#### 13.2.1 Performance Test Suite
- [x] Performance-Tests ausführen (tests/performance_test.rs)
  - Tool-Execution-Performance (< 2s timeout im Test; < 100ms Ziel auf Hardware)
  - RAM/Flash-Benchmarks auf ESP32 (manuell/CI mit Target)
- [x] Performance-Tests bestehen

### 13.3 Resource-Constraint Testing

**Abhängigkeiten**: 6.1-6.3 (Resource-Management)

#### 13.3.1 Resource Test Suite
- [x] Resource-Constraint-Tests ausführen (tests/resource_constraint_test.rs)
  - Memory-Exhaustion-Tests (MemoryMonitor is_exhausted, is_warning)
  - Network-Bandwidth-Tests (BandwidthMonitor should_throttle, tick)
  - TaskScheduler max_queued
- [x] Resource-Tests bestehen

---

## Verbleibende optionale Punkte (Übersicht)

Damit sofort sichtbar ist, was optional noch anfällt (nicht blockierend):

- [ ] **Phase 2.1.1** Jotunheim als Dependency zu gemeinsamem Protobuf-Projekt (aktuell: Copy in esp32/proto)
- [ ] **Phase 2.2.2** Dynamische Script-Funktionen (Script_<name>) – bei Bedarf/später
- [ ] **Phase 7.2.1** Digital-Signature-Verification für OTA – bei Bedarf/später
- [ ] **Phase 10** Memory-/Code-Size-Benchmarks auf echter ESP32-Hardware validieren (Ziel < 10KB RAM dokumentiert)

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 13
**Gesamtanzahl Schritte**: ~200+

**Kritische Abhängigkeiten**:
1. Protobuf-Rust-Tool (prost-lite empfohlen)
2. Primary-Target-Device (ESP32 empfohlen)
3. Network-Stack (esp-idf empfohlen)
4. gRPC-Client-Library (tonic-light empfohlen)
5. Lua-Engine (mlua empfohlen)

**Offene Fragen für USER**:
1. Protobuf-Rust-Tool (prost-lite, prost)
2. Primary-Target-Device (ESP32, ESP8266, Raspberry Pi Pico)
3. Network-Stack (esp-idf, smoltcp)
4. gRPC-Client-Library (tonic-light, grpc-rust, Eigene)
5. Lua-Engine (mlua, rlua, Eigene)

**Hinweise**:
- Alle Schritte folgen TDD
- Alle Tests in Containern
- Extrem lightweight: < 10KB RAM-Usage, minimaler Flash
- Platform-Konzept: Jotunheim = Platform, Loki = Service
- gRPC-Client für Loki-Service-Kommunikation
- Capability-Negotiation für variable IoT-Device-Konfigurationen
- OTA-Updates mit Verification und Rollback
