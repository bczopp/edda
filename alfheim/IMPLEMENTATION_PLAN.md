# IMPLEMENTATION_PLAN - Alfheim (Mobile Platform)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Alfheim - der Mobile Platform für iOS und Android. Alfheim ist eine Platform, die sich um Connections (Netzwerk, UI, etc.) kümmert, diese zu Anfragen an Services (Odin) konvertiert und Services via gRPC aufruft.

## Entschiedene Konfiguration

### Framework-Entscheidung
✅ **Entscheidung**: React Native
**Begründung**: Cross-Platform (iOS + Android), TypeScript-Support, große Community, beste Robustheit durch etablierte Libraries

### Package Manager
✅ **Entscheidung**: bun (nicht npm!)
**Begründung**: 10-100x schneller als npm, native TypeScript-Support, bessere Performance

### Audio-Integration
✅ **Entscheidung**: Phase 1 - Text + Voice-Input
**Begründung**: Vollständige User-Experience von Anfang an, Huginn/Muninn Integration in Phase 1


### Protobuf-TypeScript-Tool
✅ **Entscheidung**: ts-proto
**Begründung**: Moderne, typsichere Generierung, beste React Native Integration

### Voice-Assistant-Integration
✅ **Siri-Integration in Phase 1**: Ja
✅ **Google Assistant-Integration in Phase 1**: Ja
**Begründung**: Native Platform-Integration für beste User-Experience

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Framework-Entscheidung (siehe oben)

#### 1.1.1 Repository-Struktur erstellen
- [x] Verzeichnisstruktur nach Framework-Wahl erstellen
  - Wenn React Native: `src/ios/`, `src/android/`, `src/shared/`
  - Wenn Flutter: `lib/`, `android/`, `ios/`
  - Wenn Native: `ios/`, `android/`, `shared/` (C++ für gemeinsamen Code)
  - Vorhanden: `src/`, `src/components/`, `src/grpc/`, `src/services/`, `src/utils/`

#### 1.1.2 Package-Manager-Konfiguration
- [x] `package.json` erstellen (mit `bun` als Package-Manager)
- [ ] `bunfig.toml` erstellen für `bun`-spezifische Konfigurationen
- [x] `.gitignore` erstellen
- [x] Basis-Dependencies definieren

#### 1.1.3 Build-System einrichten
- [ ] Build-Scripts in `package.json` definieren
- [ ] Platform-spezifische Build-Konfigurationen erstellen (iOS/Android)
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → TypeScript/Dart/Swift/Kotlin)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
- [ ] Test-Container für iOS/Android-Emulation einrichten
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [x] Test-Runner: Bun Test Runner (`bun test`, `bun:test`) – kein Jest (Jest kollidiert mit Bun-Runtime)
- [x] Test-Utilities und Helpers erstellen (`tests/utils/test_helpers.ts`)
- [x] Erste Unit-Tests (TDD): `config.test.ts`, `test_helpers.test.ts`, `platformService.test.ts`
- [ ] Mock-Setup für Services (Odin, Freki, Geri, Thor, etc.) – bei Bedarf

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design
- [ ] Settings-Schema definieren (JSON-Format)
- [ ] Settings-Struktur entwerfen (Alfheim-spezifisch)
  - Service-Endpoints
  - Model-Auswahl-Konfiguration
  - Audio-Settings (falls Huginn/Muninn in Phase 1)
  - Battery-Optimization-Settings
  - Network-Usage-Settings
  - Chat-Management-Settings

#### 1.3.2 Settings-Validierung
- [ ] JSON-Schema für Settings-Validierung erstellen
- [ ] Settings-Validator implementieren (TDD)
- [ ] Tests für Settings-Validierung schreiben und ausführen

#### 1.3.3 Settings-Loader
- [ ] Settings-Loader implementieren (TDD)
- [ ] Hot-Reload-Mechanismus implementieren (TDD)
- [ ] Default-Settings definieren
- [ ] Tests für Settings-Loader schreiben und ausführen

---

## Phase 2: Protobuf & gRPC Integration

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Protobuf-TypeScript-Tool-Auswahl (siehe oben)

#### 2.1.1 Shared Protobuf-Projekt erstellen
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein (z.B. `edda-protocols`)
- [ ] Neues Projekt für Protobuf-Definitions erstellen (wenn nicht vorhanden)
- [ ] Alfheim als Dependency zu Protobuf-Projekt hinzufügen

#### 2.1.2 Platform Capability Protocol
- [ ] `EinherjarProtocol.proto` definieren (wenn nicht vorhanden)
  - `GetCapabilities()` RPC
  - `Capability` Message
  - `ServiceMetadata` Message
- [ ] Code-Generierung für Platform Capability Protocol

#### 2.1.3 Service-spezifische Protocols
- [ ] `OdinService.proto` definieren (oder aus bestehendem Projekt importieren)
- [ ] `ThorService.proto` definieren (Action Execution)
- [ ] `FrekiService.proto` definieren (RAG Service)
- [ ] `GeriService.proto` definieren (LLM Service)
- [ ] Code-Generierung für alle Service-Protocols

### 2.2 gRPC Client-Implementierung

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Client Basis-Klasse
- [ ] Tests für gRPC-Base-Client schreiben (Mocking)
- [ ] `GrpcClientBase` implementieren (TDD)
  - Connection Management
  - TLS-Support
  - Retry-Logik
  - Error-Handling
  - Timeout-Handling
- [ ] Tests ausführen und bestehen

#### 2.2.2 Service-spezifische Clients
- [ ] Tests für `OdinClient` schreiben
- [ ] `OdinClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] Tests für `ThorClient` schreiben
- [ ] `ThorClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] Tests für `FrekiClient` schreiben
- [ ] `FrekiClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] Tests für `GeriClient` schreiben
- [ ] `GeriClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 2.2.3 Client-Pooling & Wiederverwendung
- [ ] Tests für Connection-Pooling schreiben
- [ ] Connection-Pooling implementieren (TDD)
- [ ] Tests für Connection-Reuse schreiben
- [ ] Connection-Reuse implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 3: Service Discovery & Capability Management

### 3.1 Einherjar Protocol Integration

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung)

#### 3.1.1 Service Discovery Manager
- [ ] Tests für Service-Discovery schreiben
- [ ] `ServiceDiscoveryManager` implementieren (TDD)
  - Service-Liste verwalten
  - Services dynamisch erkennen
  - Service-Status überwachen (Health Checks)
- [ ] Tests ausführen und bestehen

#### 3.1.2 Capability Aggregation
- [ ] Tests für Capability-Aggregation schreiben
- [ ] `CapabilityAggregator` implementieren (TDD)
  - Capabilities von allen Services sammeln
  - Capabilities zu Odin propagieren
  - Capability-Updates verarbeiten
- [ ] Tests ausführen und bestehen

#### 3.1.3 Service Lifecycle Management
- [ ] Tests für Service-Lifecycle schreiben
- [ ] `ServiceLifecycleManager` implementieren (TDD)
  - Services starten/stoppen
  - Battery-aware Service-Management
  - Service-Fehlerbehandlung
  - Automatische Restart-Strategie
- [ ] Tests ausführen und bestehen

---

## Phase 4: Odin Integration

### 4.1 Odin Service Integration

**Abhängigkeiten**: 3.1 (Einherjar Protocol Integration)

#### 4.1.1 Odin Main Process Wrapper
- [ ] Tests für Odin-Wrapper schreiben
- [ ] `OdinProcessWrapper` implementieren (TDD)
  - Odin als Hauptprozess starten
  - Odin-Status überwachen
  - Odin-Restart bei Fehler
- [ ] Tests ausführen und bestehen

#### 4.1.2 Odin Request Handler
- [ ] Tests für Request-Handler schreiben
- [ ] `OdinRequestHandler` implementieren (TDD)
  - User-Requests zu Odin-Requests konvertieren
  - Request-Queuing
  - Request-Priorisierung (Battery-aware)
- [ ] Tests ausführen und bestehen

#### 4.1.3 Odin Response Handler
- [ ] Tests für Response-Handler schreiben
- [ ] `OdinResponseHandler` implementieren (TDD)
  - Odin-Responses verarbeiten
  - Response-Routing (UI-Updates, Notifications)
  - Error-Responses behandeln
- [ ] Tests ausführen und bestehen

### 4.2 State Management

**Abhängigkeiten**: 4.1 (Odin Service Integration)

#### 4.2.1 State Manager Design
- [ ] State-Schema definieren
  - User-Session-State
  - Service-State
  - UI-State
  - Battery-State

#### 4.2.2 State Manager Implementierung
- [ ] Tests für State-Manager schreiben
- [ ] `StateManager` implementieren (TDD)
  - State-Persistence (verschlüsselt)
  - State-Updates
  - State-Synchronisation
- [ ] Tests ausführen und bestehen

---

## Phase 5: UI Implementation (Minimal)

### 5.1 UI Grundstruktur

**Abhängigkeiten**: 4.2 (State Management)
**Erforderliche USER-Eingaben**: Framework-Entscheidung, Audio-Integration-Entscheidung

#### 5.1.1 Navigation Setup
- [ ] Navigation-Structure definieren
- [ ] Tests für Navigation schreiben
- [ ] Navigation implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 5.1.2 Main Screen (Chat Interface)
- [ ] Tests für Main-Screen schreiben
- [ ] Main-Screen UI implementieren (TDD)
  - Message-Liste (Scrollable)
  - Input-Bereich (Text)
  - Send-Button
- [ ] Tests ausführen und bestehen

#### 5.1.3 Input Handling
- [ ] Tests für Text-Input-Handler schreiben
- [ ] Text-Input-Handler implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] **Falls Audio in Phase 1**: Tests für Voice-Input-Handler schreiben
- [ ] **Falls Audio in Phase 1**: Voice-Input-Handler implementieren (TDD)
- [ ] **Falls Audio in Phase 1**: Mikrofon-Button UI implementieren
- [ ] Tests ausführen und bestehen

### 5.2 Settings UI

**Abhängigkeiten**: 3.1 (Service Discovery), 5.1 (UI Grundstruktur)

#### 5.2.1 Settings Screen
- [ ] Tests für Settings-Screen schreiben
- [ ] Settings-Screen UI implementieren (TDD)
  - Service-Endpoints-Konfiguration
  - Model-Auswahl
  - Audio-Settings (falls Huginn/Muninn in Phase 1)
  - Battery-Optimization-Settings
  - Network-Usage-Settings
- [ ] Tests ausführen und bestehen

#### 5.2.2 Settings Persistence
- [ ] Tests für Settings-Speicherung schreiben
- [ ] Settings-Speicherung implementieren (TDD)
  - Verschlüsselung (Platform-spezifisch: iOS Keychain, Android Keystore)
  - Settings-Backup
  - Settings-Restore
- [ ] Tests ausführen und bestehen

---

## Phase 6: Service Integration (Geri, Freki, Thor)

### 6.1 Geri Integration (LLM Service)

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung), 4.1 (Odin Integration)

#### 6.1.1 Geri Client Integration
- [ ] Tests für Geri-Integration schreiben
- [ ] Geri-Client in Odin-Request-Handler integrieren (TDD)
- [ ] Tests ausführen und bestehen

#### 6.1.2 Model Selection Logic
❓ **HINWEIS**: Abhängig von USER-Antwort zu "Model-Auswahl Standard"
- [ ] Tests für Model-Selection schreiben
- [ ] Model-Selection-Logik implementieren (TDD)
  - Default-Model (basierend auf Konfiguration)
  - User-explizite Auswahl
  - Automatische Auswahl (beste Wahl für Situation)
- [ ] Tests ausführen und bestehen

### 6.2 Freki Integration (RAG Service)

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung), 4.1 (Odin Integration)

#### 6.2.1 Freki Client Integration
- [ ] Tests für Freki-Integration schreiben
- [ ] Freki-Client in Odin-Request-Handler integrieren (TDD)
- [ ] Tests ausführen und bestehen

#### 6.2.2 Caching Strategy für Mobile
- [ ] Tests für Caching schreiben
- [ ] Caching-Logik implementieren (TDD)
  - Cache-Size-Limits (Mobile-Memory-Limits)
  - Cache-Eviction-Strategy
  - Cache-Persistence
- [ ] Tests ausführen und bestehen

### 6.3 Thor Integration (Action Executor)

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung), 4.1 (Odin Integration)

#### 6.3.1 Thor Client Integration
- [ ] Tests für Thor-Integration schreiben
- [ ] Thor-Client in Odin-Request-Handler integrieren (TDD)
- [ ] Tests ausführen und bestehen

#### 6.3.2 Mobile-spezifische Actions
- [ ] Tests für Mobile-Actions schreiben
- [ ] Mobile-Action-Handler implementieren (TDD)
  - App-Control
  - File-Operations (limited)
  - Network-Operations
- [ ] Tests ausführen und bestehen

---

## Phase 7: Huginn/Muninn Integration (STT/TTS)

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung), 5.1 (UI Implementation)
**Erforderliche USER-Eingaben**: Audio-Integration-Entscheidung (Phase 1 oder später)

❓ **HINWEIS**: Diese Phase wird nur ausgeführt, wenn USER entscheidet, dass Audio in Phase 1 integriert werden soll.

### 7.1 Huginn Integration (STT - Speech-to-Text)

#### 7.1.1 Audio Input Manager
- [ ] Tests für Audio-Input schreiben
- [ ] `AudioInputManager` implementieren (TDD)
  - Mikrofon-Zugriff (Platform-spezifisch)
  - Audio-Session-Management
  - Audio-Recording
  - Audio-Interruptions behandeln (Anrufe, Alarme)
- [ ] Tests ausführen und bestehen

#### 7.1.2 Huginn Client Integration
- [ ] Tests für Huginn-Integration schreiben
- [ ] Huginn-Client in Audio-Input-Handler integrieren (TDD)
- [ ] Tests ausführen und bestehen

### 7.2 Muninn Integration (TTS - Text-to-Speech)

#### 7.2.1 Audio Output Manager
- [ ] Tests für Audio-Output schreiben
- [ ] `AudioOutputManager` implementieren (TDD)
  - Speaker/Headphone-Output
  - Audio-Session-Management
  - Background-Audio-Support
- [ ] Tests ausführen und bestehen

#### 7.2.2 Muninn Client Integration
- [ ] Tests für Muninn-Integration schreiben
- [ ] Muninn-Client in Response-Handler integrieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 8: Mobile Optimizations

### 8.1 Battery Optimization

**Abhängigkeiten**: 4.2 (State Management), 6.3 (Thor Integration)

#### 8.1.1 Battery Monitor
- [ ] Tests für Battery-Monitor schreiben
- [ ] `BatteryMonitor` implementieren (TDD)
  - Battery-Level überwachen
  - Battery-Saver-Modi erkennen
  - Battery-Usage-Statistiken
- [ ] Tests ausführen und bestehen

#### 8.1.2 Battery-aware Task Scheduling
- [ ] Tests für Task-Scheduling schreiben
- [ ] `BatteryAwareScheduler` implementieren (TDD)
  - Task-Priorisierung basierend auf Battery-Level
  - Task-Deferral bei niedrigem Battery-Level
  - Background-Task-Management
- [ ] Tests ausführen und bestehen

### 8.2 Network Optimization

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung)

#### 8.2.1 Network Monitor
- [ ] Tests für Network-Monitor schreiben
- [ ] `NetworkMonitor` implementieren (TDD)
  - Network-Type erkennen (WiFi, Mobile Data, etc.)
  - Network-Quality überwachen (Bandwidth, Latency)
  - Network-Wechsel erkennen (WiFi ↔ Mobile Data)
- [ ] Tests ausführen und bestehen

#### 8.2.2 Network-aware Request Handling
- [ ] Tests für Network-aware-Requests schreiben
- [ ] Network-aware-Request-Handler implementieren (TDD)
  - Request-Priorisierung basierend auf Network-Type
  - Request-Deferral bei schlechtem Netzwerk
  - Request-Retry bei Network-Fehler
- [ ] Tests ausführen und bestehen

### 8.3 Memory Optimization

**Abhängigkeiten**: 4.2 (State Management), 6.2 (Freki Integration)

#### 8.3.1 Memory Monitor
- [ ] Tests für Memory-Monitor schreiben
- [ ] `MemoryMonitor` implementieren (TDD)
  - Memory-Usage überwachen
  - Memory-Warnings erkennen
- [ ] Tests ausführen und bestehen

#### 8.3.2 Memory-aware Resource Management
- [ ] Tests für Memory-aware-Management schreiben
- [ ] Memory-aware-Resource-Manager implementieren (TDD)
  - Cache-Size-Limits dynamisch anpassen
  - Resource-Cleanup bei Memory-Warnings
- [ ] Tests ausführen und bestehen

---

## Phase 9: Platform-Specific Features

### 9.1 iOS-spezifische Features

**Abhängigkeiten**: 5.1 (UI Implementation)

#### 9.1.1 iOS Keychain Integration
- [ ] Tests für Keychain-Integration schreiben
- [ ] Keychain-Wrapper implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.1.2 iOS Background Tasks
- [ ] Tests für Background-Tasks schreiben
- [ ] Background-Task-Manager implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.1.3 iOS Notifications
- [ ] Tests für Notifications schreiben
- [ ] Notification-Manager implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.1.4 iOS Siri Integration (Optional)
❓ **FRAGE AN USER**: Soll Siri-Integration bereits in Phase 1 implementiert werden?
- [ ] Falls ja: Tests für Siri-Integration schreiben
- [ ] Falls ja: Siri-Shortcuts implementieren (TDD)
- [ ] Falls ja: Tests ausführen und bestehen

### 9.2 Android-spezifische Features

**Abhängigkeiten**: 5.1 (UI Implementation)

#### 9.2.1 Android Keystore Integration
- [ ] Tests für Keystore-Integration schreiben
- [ ] Keystore-Wrapper implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.2.2 Android Background Services
- [ ] Tests für Background-Services schreiben
- [ ] Background-Service-Manager implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.2.3 Android Notifications
- [ ] Tests für Notifications schreiben
- [ ] Notification-Manager implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.2.4 Android Google Assistant Integration (Optional)
❓ **FRAGE AN USER**: Soll Google-Assistant-Integration bereits in Phase 1 implementiert werden?
- [ ] Falls ja: Tests für Google-Assistant-Integration schreiben
- [ ] Falls ja: Google-Assistant-Actions implementieren (TDD)
- [ ] Falls ja: Tests ausführen und bestehen

---

## Phase 10: Permissions & Security

### 10.1 Permission Management

**Abhängigkeiten**: 9.1 (iOS Features), 9.2 (Android Features)

#### 10.1.1 Permission Manager
- [ ] Tests für Permission-Manager schreiben
- [ ] `PermissionManager` implementieren (TDD)
  - Permission-Requests (Mikrofon, Kamera, etc.)
  - Permission-Status überwachen
  - Permission-Verweigerungen behandeln
- [ ] Tests ausführen und bestehen

#### 10.1.2 Permission UI
- [ ] Tests für Permission-UI schreiben
- [ ] Permission-Request-UI implementieren (TDD)
- [ ] Permission-Settings-UI implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 10.2 Heimdall Integration (Security Service)

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung)

#### 10.2.1 Heimdall Client Integration
- [ ] Tests für Heimdall-Integration schreiben
- [ ] Heimdall-Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.2.2 Authentication
- [ ] Tests für Authentication schreiben
- [ ] Authentication-Manager implementieren (TDD)
  - User-Login
  - Token-Management
  - Token-Refresh
- [ ] Tests ausführen und bestehen

#### 10.2.3 Secure Storage
- [ ] Tests für Secure-Storage schreiben
- [ ] Secure-Storage-Manager implementieren (TDD)
  - Credentials verschlüsselt speichern (Platform-spezifisch)
  - API-Keys verschlüsselt speichern
- [ ] Tests ausführen und bestehen

---

## Phase 11: Bifrost Integration (Device-to-Device Communication)

### 11.1 Bifrost Client Integration

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung), 10.2 (Heimdall Integration)

#### 11.1.1 Bifrost WebSocket Client
- [ ] Tests für Bifrost-Client schreiben
- [ ] `BifrostClient` implementieren (TDD)
  - WebSocket-Connection
  - Connection-Establishment
  - Connection-Monitoring
  - Automatic-Reconnection
- [ ] Tests ausführen und bestehen

#### 11.1.2 Device Discovery
- [ ] Tests für Device-Discovery schreiben
- [ ] Device-Discovery-Manager implementieren (TDD)
  - Discovery-Requests senden
  - Discovery-Responses verarbeiten
- [ ] Tests ausführen und bestehen

### 11.2 DeviceIdentity System

**Abhängigkeiten**: 10.2 (Heimdall Integration)

#### 11.2.1 DeviceIdentity Manager
- [ ] Tests für DeviceIdentity-Manager schreiben
- [ ] `DeviceIdentityManager` implementieren (TDD)
  - Device-Identity erstellen (user-assigned)
  - Device-Identity speichern (verschlüsselt)
  - Device-Metadata verwalten (Name, Type, Capabilities)
- [ ] Tests ausführen und bestehen

#### 11.2.2 DeviceIdentity Validation
- [ ] Tests für DeviceIdentity-Validation schreiben
- [ ] DeviceIdentity-Validator implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 11.3 Cross-Device Actions

**Abhängigkeiten**: 11.1 (Bifrost Client Integration), 6.3 (Thor Integration)

#### 11.3.1 Remote Action Execution
- [ ] Tests für Remote-Action-Execution schreiben
- [ ] `RemoteActionExecutor` implementieren (TDD)
  - ThorAction an Remote-Device senden (via gRPC über Bifrost)
  - ThorResult von Remote-Device empfangen
  - Streaming-Support für lange Actions
- [ ] Tests ausführen und bestehen

#### 11.3.2 Action Result Handling
- [ ] Tests für Action-Result-Handling schreiben
- [ ] Action-Result-Handler implementieren (TDD)
  - Result-UI-Updates
  - Result-Notifications
- [ ] Tests ausführen und bestehen

---

## Phase 12: Chat Management

### 12.1 Chat System

**Abhängigkeiten**: 4.1 (Odin Integration), 5.1 (UI Implementation)

#### 12.1.1 Chat Manager
- [ ] Tests für Chat-Manager schreiben
- [ ] `ChatManager` implementieren (TDD)
  - Neue Chats erstellen
  - Chat-Liste verwalten
  - Chat-History speichern (verschlüsselt)
- [ ] Tests ausführen und bestehen

#### 12.1.2 Chat Routing
- [ ] Tests für Chat-Routing schreiben
- [ ] Chat-Routing-Logik implementieren (TDD)
  - Chat direkt an Gott leiten (z.B. Frigg-Chat)
  - Chat über Odin leiten (Standard)
  - Flags in Settings für Chat-Routing
- [ ] Tests ausführen und bestehen

#### 12.1.3 Chat UI
- [ ] Tests für Chat-UI schreiben
- [ ] Chat-List-UI implementieren (TDD)
- [ ] Chat-Detail-UI implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 13: Activity Monitoring

### 13.1 Activity Monitor

**Abhängigkeiten**: 6.3 (Thor Integration), 3.1 (Service Discovery)

#### 13.1.1 Activity Tracker
- [ ] Tests für Activity-Tracker schreiben
- [ ] `ActivityTracker` implementieren (TDD)
  - Laufende Actions überwachen
  - Service-Status überwachen
  - Activity-History speichern
- [ ] Tests ausführen und bestehen

#### 13.1.2 Activity UI
- [ ] Tests für Activity-UI schreiben
- [ ] Activity-Monitor-UI implementieren (TDD)
  - Laufende Actions anzeigen
  - Service-Status anzeigen
  - Activity-History anzeigen
- [ ] Tests ausführen und bestehen

---

## Phase 14: Offline Mode (Limited)

### 14.1 Offline Detection

**Abhängigkeiten**: 8.2 (Network Optimization)

#### 14.1.1 Offline Manager
- [ ] Tests für Offline-Manager schreiben
- [ ] `OfflineManager` implementieren (TDD)
  - Offline-Status erkennen
  - Offline-UI anzeigen
- [ ] Tests ausführen und bestehen

### 14.2 Offline Functionality

**Abhängigkeiten**: 14.1 (Offline Detection), 12.1 (Chat System)

#### 14.2.1 Offline Message Queue
- [ ] Tests für Offline-Message-Queue schreiben
- [ ] Offline-Message-Queue implementieren (TDD)
  - Messages queuen wenn offline
  - Messages senden wenn online
- [ ] Tests ausführen und bestehen

#### 14.2.2 Local-only Features
- [ ] Tests für Local-Features schreiben
- [ ] Local-Feature-Handler implementieren (TDD)
  - Chat-History anzeigen (lokal gespeichert)
  - Settings bearbeiten (lokal gespeichert)
- [ ] Tests ausführen und bestehen

---

## Phase 15: Installation & Deployment

### 15.1 Installation Flow

**Abhängigkeiten**: 1.3 (Projekt-Konfiguration), 10.2 (Heimdall Integration)
**Erforderliche USER-Eingaben**: Keine (Model-Auswahl ist Geri-interne Konfiguration)

#### 15.1.1 First-Run Setup
- [ ] Tests für First-Run-Setup schreiben
- [ ] First-Run-Setup-UI implementieren (TDD)
  - Welcome-Screen
  - Service-Endpoint-Konfiguration
  - Permission-Requests
- [ ] Tests ausführen und bestehen

#### 15.1.2 Default Settings Population
- [ ] Tests für Default-Settings schreiben
- [ ] Default-Settings-Population implementieren (TDD)
  - Default-Werte für alle Settings
  - Validierung der Default-Werte
- [ ] Tests ausführen und bestehen

### 15.2 App Store Deployment

**Abhängigkeiten**: Alle vorherigen Phasen

#### 15.2.1 iOS App Store Vorbereitung
- [ ] App Store Metadata erstellen
- [ ] Screenshots erstellen
- [ ] App-Icons erstellen
- [ ] Privacy Policy erstellen
- [ ] App Store Compliance prüfen

#### 15.2.2 Android Play Store Vorbereitung
- [ ] Play Store Metadata erstellen
- [ ] Screenshots erstellen
- [ ] App-Icons erstellen
- [ ] Privacy Policy erstellen
- [ ] Play Store Compliance prüfen

#### 15.2.3 Release Build
- [ ] iOS Release Build konfigurieren
- [ ] Android Release Build konfigurieren
- [ ] Signing konfigurieren (iOS/Android)
- [ ] Release Notes erstellen

---

## Phase 16: Yggdrasil Integration (Device Registry)

### 16.1 Yggdrasil Client Integration

**Abhängigkeiten**: 2.2 (gRPC Client-Implementierung), 11.2 (DeviceIdentity System)

#### 16.1.1 Yggdrasil Client
- [ ] Tests für Yggdrasil-Client schreiben
- [ ] `YggdrasilClient` implementieren (TDD)
  - Device-Registration bei Yggdrasil
  - User-Management über Yggdrasil
- [ ] Tests ausführen und bestehen

#### 16.1.2 Device Registration
- [ ] Tests für Device-Registration schreiben
- [ ] Device-Registration-Flow implementieren (TDD)
  - Device bei Yggdrasil registrieren
  - Device-Metadata zu Yggdrasil senden
- [ ] Tests ausführen und bestehen

---

## Phase 17: Documentation

### 17.1 Code Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 17.1.1 API Documentation
- [ ] Alle Public APIs dokumentieren
- [ ] Code-Comments hinzufügen für komplexe Logik
- [ ] JSDoc/TSDoc für TypeScript (oder entsprechende Dokumentation für Flutter/Native)

#### 17.1.2 Architecture Documentation
- [ ] Architecture-Diagramm erstellen
- [ ] Component-Diagramme erstellen
- [ ] Sequence-Diagramme für wichtige Workflows

### 17.2 User Documentation

**Abhängigkeiten**: 15.1 (Installation Flow)

#### 17.2.1 Installation Guide
- [ ] Installations-Anleitung erstellen
- [ ] Screenshots für Installation-Steps
- [ ] Troubleshooting-Section

#### 17.2.2 User Guide
- [ ] User-Guide für Alfheim erstellen
- [ ] Feature-Beschreibungen
- [ ] Tutorials für wichtige Features

---

## Phase 18: Testing & Quality Assurance

### 18.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 18.1.1 End-to-End Tests
- [ ] E2E-Tests für komplette User-Workflows schreiben
  - User-Input → Odin → Geri → Response
  - User-Input → Odin → Thor → Action-Execution
  - Cross-Device-Actions
- [ ] E2E-Tests ausführen und bestehen

#### 18.1.2 Performance Testing
- [ ] Performance-Tests schreiben
  - Startup-Time
  - Response-Time
  - Memory-Usage
  - Battery-Usage
  - Network-Usage
- [ ] Performance-Tests ausführen und Benchmarks erreichen

### 18.2 Security Testing

**Abhängigkeiten**: 10.2 (Heimdall Integration)

#### 18.2.1 Security Audit
- [ ] Security-Audit durchführen
- [ ] Vulnerability-Scanning
- [ ] Penetration-Testing (optional)

#### 18.2.2 Privacy Audit
- [ ] Privacy-Audit durchführen
- [ ] GDPR-Compliance prüfen
- [ ] App Store Privacy Compliance prüfen

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 18
**Gesamtanzahl Schritte**: ~200+

**Kritische Abhängigkeiten**:
1. Framework-Entscheidung (beeinflusst gesamte Implementierung)
2. Audio-Integration-Entscheidung (beeinflusst Phase 7 und UI)
3. Model-Auswahl-Standard (beeinflusst Installation und Konfiguration)
4. Protobuf-TypeScript-Tool (beeinflusst Code-Generierung)

**Offene Fragen für USER**:
1. Framework-Wahl (React Native, Flutter, Native)
2. Audio-Integration in Phase 1? (Ja/Nein)
4. Protobuf-TypeScript-Tool (ts-proto, protoc-gen-ts, @grpc/grpc-js)
5. Siri-Integration in Phase 1? (Ja/Nein)
6. Google-Assistant-Integration in Phase 1? (Ja/Nein)

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- Alle Tests müssen in Containern laufen (keine lokalen Dependencies)
- Alle Schritte sind kleinstmöglich aufgeteilt
- Abhängigkeiten zwischen Phasen sind klar definiert
- Offene Fragen sind klar markiert (❓)
