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
- [ ] `esp32/src/capability/` für Capability-Management erstellen
- [ ] `esp32/src/remote/` für Remote-Control erstellen
- [ ] `esp32/src/ota/` für OTA-Updates erstellen
- [x] `esp32/src/utils/` für Utilities erstellen
- [ ] `esp32/config/` für Konfigurationsdateien erstellen

#### 1.1.3 Build-System einrichten
- [ ] Build-Scripts für ESP32 definieren (esp-rs)
- [ ] Flash-Scripts für ESP32 erstellen
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren (z.B. `esp32`, `esp8266`, `pico`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
  - Mock-Loki-Service
  - Mock-Controller (Midgard/Asgard)
  - Mock-Network-Simulator (für WiFi-Tests)
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (embedded-test, defmt-test)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Services
- [ ] Test-Data-Generators erstellen

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren (Minimal)
- [ ] Settings-Struktur entwerfen (minimales JSON)
  - capability_configuration
  - network_resilience_settings
  - resource_limits
  - ota_update_settings (optional)

#### 1.3.2 Settings-Validierung
- [ ] Tests für Settings-Validierung schreiben
- [ ] Rust-Structs für Settings definieren (minimal)
- [ ] Settings-Validator implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
  - Flash-Storage-Integration
  - Runtime-Settings-Reload (minimal)
- [ ] Tests ausführen und bestehen

---

## Phase 2: Protobuf & gRPC Client

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Jotunheim als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren (minimal subset für IoT)

#### 2.1.2 Capability Protocol (Platform Capability Protocol)
- [ ] `JotunheimCapability.proto` definieren
  - `JotunheimCapabilities` Message (siehe README.md)
  - `CapabilityRequest` Message
  - `CapabilityResponse` Message
  - `CapabilityUpdateEvent` Message
- [ ] Code-Generierung konfigurieren (Protobuf-Lite)

#### 2.1.3 Loki Service Protocol
- [ ] `LokiService.proto` verwenden (aus Loki-Projekt)
  - `GetCapabilities()` RPC
  - `GetChildrenStatus()` RPC
  - `ListScripts()` RPC
  - Dynamische Script-Funktionen (`Script_<script_name>`)
- [ ] Code-Generierung konfigurieren (Protobuf-Lite)

### 2.2 gRPC Client Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)
**Erforderliche USER-Eingaben**: gRPC-Client-Library

#### 2.2.1 Lightweight-gRPC-Client
- [ ] Tests für gRPC-Client schreiben
- [ ] `LightweightGRPCClient` implementieren (TDD)
  - tonic-light oder eigene minimal-Impl.
  - Protobuf-Lite-Support
  - HTTP/2-minimal
- [ ] Tests ausführen und bestehen

#### 2.2.2 Loki-Client
- [ ] Tests für Loki-Client schreiben
- [ ] `LokiClient` implementieren (TDD)
  - gRPC-Connection zu Loki-Service
  - `GetCapabilities()` aufrufen
  - `ListScripts()` aufrufen
  - Dynamische Script-Funktionen aufrufen
- [ ] Tests ausführen und bestehen

---

## Phase 3: Network-Management (ESP32)

### 3.1 WiFi-Management

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Network-Stack

#### 3.1.1 WiFi-Manager
- [ ] Tests für WiFi-Manager schreiben
- [ ] `WiFiManager` implementieren (TDD)
  - WiFi-Connection aufbauen
  - WiFi-Reconnection (automatisch)
  - WiFi-Status überwachen
- [ ] Tests ausführen und bestehen

### 3.2 TCP/IP-Stack

**Abhängigkeiten**: 3.1 (WiFi-Management)

#### 3.2.1 TCP-Client
- [ ] Tests für TCP-Client schreiben
- [ ] `TCPClient` implementieren (TDD)
  - TCP-Connection aufbauen
  - TCP-Send/Receive
  - Connection-Resilience
- [ ] Tests ausführen und bestehen

---

## Phase 4: Capability Management

### 4.1 Capability-Definition

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 4.1.1 Device-Capability-Builder
- [ ] Tests für Capability-Builder schreiben
- [ ] `DeviceCapabilityBuilder` implementieren (TDD)
  - Device-Information sammeln (Device-ID, Name, Type, Firmware)
  - Hardware-Capabilities sammeln (GPIO, Interfaces, Sensors, Actuators)
  - Resource-Limits sammeln
  - Protocol-Features sammeln
  - `JotunheimCapabilities` erstellen
- [ ] Tests ausführen und bestehen

### 4.2 Capability-Negotiation

**Abhängigkeiten**: 4.1 (Capability-Definition), 2.2.1 (Lightweight-gRPC-Client)

#### 4.2.1 Capability-Negotiator
- [ ] Tests für Capability-Negotiator schreiben
- [ ] `CapabilityNegotiator` implementieren (TDD)
  - `CAPABILITY_REQUEST` empfangen
  - Capabilities senden
  - Negotiation-Timeouts behandeln
- [ ] Tests ausführen und bestehen

### 4.3 Capability-Propagation

**Abhängigkeiten**: 4.2 (Capability-Negotiation)

#### 4.3.1 Capability-Propagator
- [ ] Tests für Capability-Propagator schreiben
- [ ] `CapabilityPropagator` implementieren (TDD)
  - Capabilities bei Kopplung/Verbindung propagieren
  - Capability-Update-Events senden
  - Einherjar Protocol nutzen (Platform Capability Protocol)
- [ ] Tests ausführen und bestehen

---

## Phase 5: Remote Control

### 5.1 Command-Handler

**Abhängigkeiten**: 2.2.2 (Loki-Client)

#### 5.1.1 Remote-Command-Handler
- [ ] Tests für Remote-Command-Handler schreiben
- [ ] `RemoteCommandHandler` implementieren (TDD)
  - Commands von Controller empfangen
  - Commands an Loki-Service weiterleiten
  - Results von Loki empfangen
  - Results an Controller zurücksenden
- [ ] Tests ausführen und bestehen

---

## Phase 6: Resource-Management

### 6.1 Memory-Management

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 6.1.1 Memory-Monitor
- [ ] Tests für Memory-Monitor schreiben
- [ ] `MemoryMonitor` implementieren (TDD)
  - RAM-Usage überwachen
  - Memory-Exhaustion erkennen
  - Memory-Warnings
- [ ] Tests ausführen und bestehen

#### 6.1.2 Memory-Pooling
- [ ] Tests für Memory-Pooling schreiben
- [ ] `MemoryPool` implementieren (TDD)
  - Memory-Pools für häufige Allocations
  - Efficient Memory-Reuse
- [ ] Tests ausführen und bestehen

### 6.2 CPU-Management

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 6.2.1 Task-Scheduler
- [ ] Tests für Task-Scheduler schreiben
- [ ] `TaskScheduler` implementieren (TDD)
  - Task-Scheduling
  - Priority-basierte Scheduling
  - CPU-Constraints beachten
- [ ] Tests ausführen und bestehen

### 6.3 Network-Bandwidth-Management

**Abhängigkeiten**: 3.2 (TCP/IP-Stack)

#### 6.3.1 Bandwidth-Monitor
- [ ] Tests für Bandwidth-Monitor schreiben
- [ ] `BandwidthMonitor` implementieren (TDD)
  - Network-Usage überwachen
  - Bandwidth-Limits
  - Throttling bei hoher Last
- [ ] Tests ausführen und bestehen

---

## Phase 7: OTA Updates

### 7.1 OTA-Update-Manager

**Abhängigkeiten**: 3.1 (WiFi-Management), 2.2.1 (gRPC-Client)

#### 7.1.1 OTA-Update-Client
- [ ] Tests für OTA-Update-Client schreiben
- [ ] `OTAUpdateClient` implementieren (TDD)
  - Update-Verfügbarkeit prüfen (Asgard/Yggdrasil)
  - Update herunterladen
  - Update installieren
  - Device-Restart
- [ ] Tests ausführen und bestehen

### 7.2 Update-Verification

**Abhängigkeiten**: 7.1 (OTA-Update-Manager)

#### 7.2.1 Update-Verifier
- [ ] Tests für Update-Verifier schreiben
- [ ] `UpdateVerifier` implementieren (TDD)
  - Digital-Signature-Verification
  - Checksum-Verification (SHA256)
  - Update-Integrity-Check
- [ ] Tests ausführen und bestehen

### 7.3 Rollback-Manager

**Abhängigkeiten**: 7.1 (OTA-Update-Manager)

#### 7.3.1 Rollback-Handler
- [ ] Tests für Rollback-Handler schreiben
- [ ] `RollbackHandler` implementieren (TDD)
  - Automatischer Rollback (wenn möglich)
  - Alte Version wiederherstellen
  - Manueller Rollback-Support
- [ ] Tests ausführen und bestehen

---

## Phase 8: Error Handling & Resilience

### 8.1 Connection-Resilience

**Abhängigkeiten**: 3.1 (WiFi-Management), 3.2 (TCP-Client)

#### 8.1.1 Connection-Resilience-Manager
- [ ] Tests für Connection-Resilience schreiben
- [ ] `ConnectionResilienceManager` implementieren (TDD)
  - Automatische Reconnection
  - Exponential-Backoff-Retry
  - Connection-Monitoring
- [ ] Tests ausführen und bestehen

### 8.2 Error-Handler

**Abhängigkeiten**: 2.2 (gRPC Client Implementation)

#### 8.2.1 Error-Handler
- [ ] Tests für Error-Handler schreiben
- [ ] `ErrorHandler` implementieren (TDD)
  - gRPC-Error-Handling
  - Network-Error-Handling
  - Resource-Error-Handling
  - Fehler-Logging (minimal)
- [ ] Tests ausführen und bestehen

### 8.3 Retry-Manager

**Abhängigkeiten**: 8.2 (Error-Handler)

#### 8.3.1 Retry-Manager
- [ ] Tests für Retry-Manager schreiben
- [ ] `RetryManager` implementieren (TDD)
  - Exponential-Backoff-Retry
  - Max-Retry-Count
  - Retry-Delay berechnen
- [ ] Tests ausführen und bestehen

---

## Phase 9: Streaming-Vorbereitung (Plugin-Support)

### 9.1 Streaming-Interface

**Abhängigkeiten**: 2.2 (gRPC Client Implementation)

#### 9.1.1 Streaming-Handler-Interface
- [ ] Tests für Streaming-Interface schreiben
- [ ] `StreamingHandler` Trait definieren
  - `send_video_stream()` Methode
  - `send_audio_stream()` Methode
  - `receive_video_stream()` Methode
  - `receive_audio_stream()` Methode
- [ ] Tests ausführen und bestehen

### 9.2 Plugin-Loader (Vorbereitung)

**Abhängigkeiten**: 9.1 (Streaming-Interface)

#### 9.2.1 Plugin-System-Vorbereitung
- [ ] Tests für Plugin-System schreiben
- [ ] `PluginLoader` implementieren (TDD - minimal)
  - Plugin-Interface definieren
  - Plugin-Loading-Vorbereitung (für zukünftige Plugins)
  - Streaming-Plugin-Support-Vorbereitung
- [ ] Tests ausführen und bestehen

---

## Phase 10: Performance Optimization

### 10.1 Memory-Optimization

**Abhängigkeiten**: 6.1 (Memory-Management)

#### 10.1.1 Memory-Profiling & Optimization
- [ ] Memory-Profiling-Tests schreiben
- [ ] Memory-Usage optimieren
  - Stack-Usage minimieren
  - Heap-Allocations minimieren
  - Memory-Pooling nutzen
- [ ] Memory-Tests ausführen und Benchmarks erreichen (< 10KB)

### 10.2 Flash-Usage-Optimization

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 10.2.1 Code-Size-Optimization
- [ ] Code-Size-Tests schreiben
- [ ] Code-Size optimieren
  - Unused-Code entfernen
  - Dependencies minimieren
  - Compiler-Optimierungen
- [ ] Code-Size-Tests ausführen

---

## Phase 11: Monitoring & Logging (Minimal)

### 11.1 Minimal-Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 11.1.1 Logging Setup (Minimal)
- [ ] Minimal-Logging konfigurieren (defmt oder log-minimal)
- [ ] Nur kritische Log-Levels (ERROR, WARN)
- [ ] Log-Output über Serial (für Debugging)

### 11.2 Resource-Monitoring

**Abhängigkeiten**: 6.1 (Memory-Management), 6.2 (CPU-Management)

#### 11.2.1 Resource-Monitor
- [ ] Tests für Resource-Monitor schreiben
- [ ] `ResourceMonitor` implementieren (TDD)
  - RAM-Usage tracken
  - CPU-Usage tracken
  - Network-Usage tracken
- [ ] Tests ausführen und bestehen

---

## Phase 12: Documentation

### 12.1 Platform Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 12.1.1 Platform-Guide
- [ ] Platform-Guide erstellen
- [ ] ESP32-Setup dokumentieren
- [ ] Flash-Instructions dokumentieren
- [ ] Capability-Configuration dokumentieren

### 12.2 Examples

**Abhängigkeiten**: Alle vorherigen Phasen

#### 12.2.1 Example-Implementations
- [ ] ESP32-Beispiele erstellen
  - LED-Control-Example
  - Sensor-Reading-Example
  - Remote-Control-Example
- [ ] Example-Documentation erstellen

---

## Phase 13: Testing & Quality Assurance

### 13.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 13.1.1 End-to-End Tests
- [ ] E2E-Tests für IoT-Workflows schreiben
  - Connection-Establishment → Capability-Negotiation → Remote-Control
  - OTA-Update → Verification → Installation → Rollback
- [ ] E2E-Tests ausführen und bestehen

### 13.2 Performance Testing

**Abhängigkeiten**: 10.1 (Memory-Optimization)

#### 13.2.1 Performance Test Suite
- [ ] Performance-Tests ausführen
  - RAM-Usage-Tests (< 10KB)
  - Flash-Usage-Tests
  - Tool-Execution-Performance-Tests (< 100ms)
- [ ] Performance-Tests bestehen

### 13.3 Resource-Constraint Testing

**Abhängigkeiten**: 6.1-6.3 (Resource-Management)

#### 13.3.1 Resource Test Suite
- [ ] Resource-Constraint-Tests ausführen
  - Memory-Exhaustion-Tests
  - CPU-Exhaustion-Tests
  - Network-Bandwidth-Tests
- [ ] Resource-Tests bestehen

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
