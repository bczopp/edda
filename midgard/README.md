# Midgard - Desktop Platform

## Übersicht

Midgard ist eine **Platform** für Desktop/Laptop-Geräte, ähnlich wie Alfheim (Mobile), Asgard (Homeserver), Ragnarok (Terminal) und Jotunheim (IoT). Als Platform ist Midgard komplett platformspezifisch optimiert und kümmert sich um Connections (Netzwerk, UI, etc.), konvertiert diese zu Anfragen an Services (Odin) und ruft Services via gRPC auf.

**Services sind unabhängig von Platformen**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind in Rust implementiert und unabhängig von Platformen. Platformen kommunizieren mit Services via gRPC.

**Tests ausführen:** Von `midgard/`: `docker compose -f docker-compose.test.yml run --rm midgard-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `midgard/**` läuft [.github/workflows/midgard.yml](../.github/workflows/midgard.yml) (Test im Container, Lint).

## Zielplattformen

- Windows (10/11)
- macOS
- Linux (Ubuntu, Debian, Fedora, etc.)

### Platform-spezifische Implementierung
- **Windows**: Native Windows-APIs, Windows-spezifische Features (Tray-Icons, Notifications, System-Integration)
- **macOS**: Native macOS-APIs, macOS-spezifische Features (Menu Bar, Notifications, System-Integration)
- **Linux**: Native Linux-APIs, Linux-spezifische Features (System Tray, Notifications, Desktop-Integration)
- **Platform-Updates**: Automatische Behandlung von Platform-Updates und API-Änderungen

## Projektstruktur

```
midgard/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── frontend/          # Frontend (TypeScript/React)
│   │   ├── components/
│   │   ├── pages/
│   │   └── ...
│   ├── platform/          # Platform-Logik (Rust)
│   │   ├── grpc_client/  # gRPC-Clients für Services
│   │   ├── services/      # Service-Integration
│   │   │   ├── odin.rs    # Odin Service Client
│   │   │   ├── huginn.rs  # STT Service Client
│   │   │   ├── muninn.rs  # TTS Service Client
│   │   │   ├── freki.rs   # RAG Service Client
│   │   │   ├── geri.rs    # LLM Service Client
│   │   │   └── thor.rs    # Action Executor Client
│   │   └── actions/       # Action Handlers
│   └── utils/
├── config/
├── resources/
└── tests/
```

## Features

### Core Features

- **User Input**: Frontend unterstützt sowohl Text- als auch Sprach-Input
  - **Text-Input**: User kann Commands als Text eingeben
  - **Voice-Input**: User kann Commands per Sprache übermitteln (via Huginn/Muninn STT/TTS)
  - **Flexibel**: User kann zwischen Text und Sprache wählen
- **LLM Integration**: Via Geri - Model-Auswahl basierend auf Konfiguration
- **RAG Support**: Via Freki
- **Action Execution**: Via Thor
- **Device-to-Device Communication**: Via Bifrost

### Desktop-Specific Features

#### System Integration
- File System Access
- Application Control
- System Settings
- Clipboard Integration

#### UI Components (Optional)
- **Optionales Frontend**: User kann optional ein Frontend nutzen, um nachzuvollziehen, was im System passiert
- **Desktop-UI-Framework**: Tauri oder Electron für Desktop-UI (TypeScript/React)
  - **Platform-spezifische UI**: Platform-spezifische UI-Implementierungen für Windows, macOS, Linux
  - **UI-Performance**: Optimierte UI-Performance mit Lazy-Loading für UI-Komponenten
  - **Memory-Usage**: Effizientes Memory-Management für UI-Komponenten
- **Input-Methoden**: Frontend unterstützt sowohl Text- als auch Sprach-Input
  - **Text-Eingabe**: Textfeld für manuelle Command-Eingabe
  - **Voice-Eingabe**: Mikrofon-Button für Sprach-Commands
  - **Wechselbar**: User kann jederzeit zwischen Text und Sprache wechseln
- **System-Integration**:
  - **Tray-Icons**: System Tray Integration für alle Platformen
  - **Notifications**: Platform-spezifische Notification-Support (Windows, macOS, Linux)
  - **System-Permissions**: Behandlung von System-Permissions (Mikrofon, Dateisystem, etc.)
  - **Permission-Requests**: UI für Permission-Requests und Permission-Verweigerungen
- **Settings UI**: Konfigurations-UI für alle Einstellungen
- **Status Dashboard**: Übersicht über System-Status und Services
- **Activity Monitoring**: Übersicht über laufende Actions und Services

#### Performance
- **Full Hardware Utilization**: Vollständige Nutzung der verfügbaren Hardware-Ressourcen
- **Multi-threading Support**: Multi-Threading-Unterstützung für parallele Verarbeitung
- **GPU Acceleration Support**: GPU-Beschleunigung für LLM-Inferenz
- **UI-Performance**: Optimierte UI-Performance mit Lazy-Loading und effizientem Rendering
- **Performance-Monitoring**: Monitoring von UI-Performance und -Lag
- **Memory-Usage**: Effizientes Memory-Management für minimale RAM-Nutzung

## Platform-Architektur

### Platform-Rolle

**Midgard als Platform:**
- **Connections**: Midgard-Platform kümmert sich um Connections (Netzwerk, UI, etc.)
- **Konvertierung**: Konvertiert Connections zu Anfragen an Services (Odin)
- **Platformspezifisch**: Komplett platformspezifische Implementierung (Windows, macOS, Linux)
- **Service-Aufrufe**: Ruft Services (Odin, Thor, Freki, Geri, etc.) via gRPC auf

**Service-Unabhängigkeit:**
- **Services in Rust**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind unabhängig von Platformen
- **gRPC-Kommunikation**: Midgard-Platform kommuniziert mit Services via gRPC
- **Wiederverwendbar**: Services können von verschiedenen Platformen genutzt werden

## Service Integration

### Service-Discovery und Service-Lifecycle

**Service-Unabhängigkeit:**
- Services sind unabhängig von Platformen implementiert
- Ermöglicht flexible Entscheidungen, welche Services auf Midgard verfügbar sind
- Services können je nach Bedarf und Hardware-Kapazität installiert werden

**Service-Discovery (Platform Capability Protocol):**
- **Einheitliches Protocol**: Alle Platformen (Midgard, Alfheim, Asgard, Ragnarok, Jotunheim) nutzen das gleiche Protocol
- **Einherjar Protocol**: Platform ruft `EinherjarProtocol.GetCapabilities()` für alle Services auf der Platform auf
- **Capability-Aggregation**: Platform aggregiert Capabilities von allen Services und propagiert sie an Odin
- **Service-Discovery**: Platform propagiert alle Methoden, die Odin als public ermittelt von allen Göttern, die auf der Platform vorhanden sind
- **Odin nutzt Einherjar Protocol**: Odin nutzt Einherjar Protocol zur Funktions-Entdeckung
- **Von außen wird niemals direkt mit einem Gott geredet**: Alle Kommunikation läuft über die Platform

**Service-Kommunikation:**
- **Innerhalb der Platform**: Services können via gRPC kommunizieren, wenn nötig. Direkte Aufrufe sind auch möglich, wenn das performanter ist. Platform entscheidet flexibel über Kommunikationsmethode.
- **Platformübergreifend**: Sowohl Bifrost als auch gRPC müssen unterstützt werden. Bifrost für Connection-Establishment, dann gRPC für Service-Kommunikation.

**Service-Lifecycle-Management:**
- Services werden als separate Prozesse gestartet (Microservices-Architektur)
- Platform startet und stoppt Services basierend auf Verfügbarkeit und Bedarf
- Health Checks werden implementiert für Service-Status-Überwachung
- Bei Service-Ausfall: Automatische Fallbacks, Restart-Strategie, Service-Fehler werden dem User kommuniziert

### Odin Integration
- **Main Process Orchestration**: Odin koordiniert alle Services auf Midgard
- **Event Handling**: Event-basierte Kommunikation zwischen Midgard und Odin
- **State Management**: Zustandsverwaltung für Odin-Requests und -Responses
- **Request-Queuing**: Request-Queuing für parallele Requests an Odin
- **Odin-Ausfälle**: Robustes Error-Handling bei Odin-Ausfällen (Fallback, Retry, User-Benachrichtigung)

### Huginn/Muninn Integration
- **Microphone Input**: Mikrofon-Input für Voice-Commands
- **Speaker Output**: Speaker-Output für TTS-Responses
- **Audio Device Management**: Verwaltung von Audio-Devices (Mikrofon, Speaker)
- **Voice-Input-UI**: UI-Komponenten für Voice-Input (Mikrofon-Button, Voice-Status)
- **Audio-Device-Änderungen**: Automatische Behandlung von Audio-Device-Änderungen (Plug/Unplug)

### Freki Integration
- Local Vector Database
- Document Indexing
- Context Retrieval

### Geri Integration
- **Model-Auswahl basierend auf Konfiguration**: 
  - Konfiguration kann vom Device selbst kommen
  - Oder vom verbundenen Server (Asgard)
  - User kann explizit Model wählen
  - Oder automatische Auswahl (beste Wahl für aktuelle Situation)
- **Unified Model Access**: Geri kann zu jedem Model verbinden (lokal oder Cloud), basierend auf Konfiguration
- **Keine Bevorzugung**: Cloud LLM Provider sind nur verfügbar, wenn User API-Keys/Credentials hinterlegt hat
- Model Management

### Thor Integration
- **File Operations**: Datei-Operationen über Thor
- **Application Control**: Anwendungssteuerung über Thor
- **System Commands**: System-Commands über Thor
- **Network Operations**: Netzwerk-Operationen über Thor
- **Action-Tracking**: UI für Action-Tracking und Action-Status-Anzeigen
- **Action-Ergebnisse**: Anzeige von Action-Ergebnissen in der UI

## Settings und Konfiguration

### Allgemeine Settings-Prinzipien

**Wichtig**: Diese Prinzipien gelten für alle Services und Platformen im Edda-System.

#### Settings-Format
- **Format**: Vermutlich JSON-Format (es sei denn im Rust-Kontext gibt es ein besseres Format, das ebenso einfach für Menschen zu verstehen ist)
- **Menschlich lesbar**: Settings-Dateien müssen für Menschen einfach zu verstehen und zu bearbeiten sein
- **Validierung**: Settings werden beim Laden validiert (Schema-Validierung)

#### Platform-Integration
- **Settings-Sammlung**: Platformen müssen alle Settings/Konfigurationsdateien sammeln, die auf dem Device bzw. auf der Platform aktuell verfügbar und aktiv sind
- **Frontend-Konfiguration**: Settings müssen über Settings im Frontend konfigurierbar gemacht werden
- **Zentrale Verwaltung**: Platform stellt zentrale Settings-Verwaltung zur Verfügung

#### Hot-Reload
- **Keine Neukompilierung**: Änderungen an den Settings sollen nicht dazu führen, dass das Projekt/der Service neu kompiliert werden muss
- **Runtime-Reload**: Die neuen Werte können einfach zur Laufzeit neu geladen werden
- **Service-Funktionen**: Services müssen entsprechende Funktionen zur Verfügung stellen (Hot-Reload, Settings-API, etc.)

#### Service-spezifische Settings
- **Projekt-spezifisch**: Was genau in einer Settings/Konfigurationsdatei steht, hängt sehr stark vom Service oder der Platform ab
- **Dokumentation**: Service-spezifische Settings müssen in der jeweiligen README dokumentiert werden
- **Beispiele**: Service-spezifische Settings-Beispiele sollten in der README enthalten sein

#### Settings-Befüllung bei Installation
- **Installation-Defaults**: Settings müssen bei der Installation (mindestens mit Default-Werten) befüllt werden
- **Jeder Gott hat LLM**: Jeder Gott hat ein LLM, um Dinge zu tun, aber auch bestimmten Code, der den Workflow darstellt
- **Default-Konfiguration**: Jeder Service/Plugin muss mit funktionsfähigen Default-Settings installiert werden können

### Midgard-spezifische Settings

**Chat-Management:**
- **Beliebig viele Chats**: Platform muss ermöglichen, quasi beliebig viele Chats zu starten
- **Chat-Leitung**: Chats können direkt an Götter geleitet werden (z.B. Frigg-Chat)
- **Chat-Flags**: Flags in Settings steuern, ob ein Chat direkt an einen Gott geleitet wird oder über Odin läuft

## Configuration

### Installation Configuration
- **Model Selection**: Model-Auswahl basierend auf Hardware-Kapazität
- **Service Selection**: Auswahl zwischen lokalen und Cloud-Services
- **Audio Device Configuration**: Konfiguration von Audio-Devices (Mikrofon, Speaker)
- **Network Configuration**: Netzwerk-Konfiguration für Device-to-Device Communication

### Runtime Configuration
- **Model Settings**: Konfiguration von LLM-Models (Model-Auswahl, Model-Updates)
  - **UI für Model-Auswahl**: UI-Komponenten für Model-Auswahl und -Konfiguration
  - **Model-Updates**: Automatische Behandlung von Model-Updates
- **Audio Settings**: Konfiguration von Audio-Einstellungen (Qualität, Sprache, Voice)
- **Security Settings**: Konfiguration von Security-Einstellungen (Permissions, Encryption)
- **Performance Settings**: Konfiguration von Performance-Einstellungen (Threading, GPU, etc.)

### Konfigurations-Management
- **Konfigurations-Speicherung**: Lokale Speicherung von Konfigurationen (verschlüsselt)
- **Konfigurations-Synchronisation**: Synchronisation von Konfigurationen zwischen Devices (optional)
- **Konfigurations-Konflikte**: Behandlung von Konfigurations-Konflikten bei Synchronisation

## Abhängigkeiten

### TypeScript/Frontend-Tooling

- **WICHTIG**: Für TypeScript/Frontend-Entwicklung muss zwingend `bun` verwendet werden (statt npm oder pnpm)
- **Integration**: `bun` wird als Package-Manager und Runtime verwendet
  - Integration erfolgt über `bun install` für Dependencies und `bun run` für Scripts
  - `bun`-spezifische Konfigurationen können in `package.json` oder `bunfig.toml` definiert werden
  - Dependencies werden über `bun` verwaltet (schneller als npm/pnpm)

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Midgard sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- Platform-specific APIs
- Audio Libraries
- UI Framework (Electron, Tauri, etc.)

## Integration

- **Odin**: Läuft als Hauptprozess auf Midgard
- **Services**: Alle Services (Huginn/Muninn, Freki, Geri, Thor) werden von Odin koordiniert
- **Bifrost**: Für Device-to-Device Communication
  - **Bifrost-Integration**: Integration von Bifrost für Device-Verbindungen
  - **UI für Device-Verbindungen**: UI-Komponenten für Device-Verbindungen und -Status
  - **Verbindungsprobleme**: Robustes Error-Handling bei Verbindungsproblemen (Retry, Fallback)
- **Heimdall**: Für Security und Authentication
- **Asgard**: Kann als Server im Netzwerk fungieren
- **Yggdrasil**: Für globale Device-Registry und User-Management

### Cross-Device Actions
- **UI-Darstellung**: UI-Komponenten für Cross-Device Actions (Action-Status, Action-Ergebnisse)
- **Action-Status-Anzeigen**: Echtzeit-Anzeige von Action-Status für Cross-Device Actions
- **Action-Ergebnisse**: Anzeige von Action-Ergebnissen von anderen Devices in der UI

## Datenschutz

### Datenschutz-Features
- **Lokale Datenverarbeitung**: Daten werden bevorzugt lokal verarbeitet
  - **UI-Indikatoren**: UI-Indikatoren für lokale vs. Cloud-Verarbeitung (Status-Anzeigen)
  - **Datenschutz-Präferenzen**: UI für Datenschutz-Präferenzen (lokale vs. Cloud-Verarbeitung)
- **Minimale Cloud-Nutzung**: Cloud-Services nur bei expliziter User-Erlaubnis
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten ohne Zustimmung
- **Datenverschlüsselung**: Sensible Daten werden verschlüsselt gespeichert
- **User Control**: User hat volle Kontrolle über seine Daten

### Compliance
- **GDPR-konform**: Einhaltung der GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Security-Features
- **Secure Storage**: Verschlüsselte Speicherung von Credentials und Tokens
- **TLS Encryption**: Alle Netzwerk-Verbindungen sind verschlüsselt
- **Authentication**: Sichere Authentifizierung über Heimdall
- **Permission System**: Granulares Permission-System für Actions
- **Sandboxing**: Sandboxing für unsichere Actions
- **Input Validation**: Umfassende Input-Validierung

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder API-Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Audit Logging**: Logging aller Security-relevanten Events

## Device Interconnection (Phase 2)

### DeviceIdentity System

**DeviceIdentity Management**
- **Device Registration**: Jedes Device erhält eine eindeutige ID (user-assigned)
- **Identity Storage**: Device-Identity wird lokal gespeichert
- **Identity Validation**: Validierung von Device-Identities
- **Identity Sharing**: Devices teilen ihre Identity mit anderen Devices

**Features**
- User-assigned Device IDs
- Device Metadata (Name, Type, Capabilities)
- Identity Persistence
- Identity Verification

**Data Structure**
- Device ID (user-assigned, unique)
- Device Name
- World Type (Midgard, Asgard, Alfheim, Jotunheim)
- Capabilities
- Hardware Specs
- Registration Timestamp

**Storage**
- Local SQLite Database
- Encrypted Storage
- Backup & Restore

### Device Discovery & Connection

**Workflow**
1. **Device A möchte sich mit Device B verbinden**
   - Device A sendet Discovery-Request
   - Device B antwortet mit Device-Identity

2. **Connection Establishment**
   - Device A initiiert Bifrost-Connection
   - Heimdall validiert beide Device-Identities
   - TLS-Handshake wird durchgeführt
   - Connection wird etabliert

3. **Device Communication**
   - Device A kann Messages an Device B senden
   - Device B kann Messages an Device A senden
   - Messages werden über Bifrost geroutet

### Cross-Device Action Execution

**Workflow**
1. **User gibt Command auf Device A**
   - Odin auf Device A verarbeitet Command
   - Odin entscheidet, dass Action auf Device B ausgeführt werden soll

2. **Connection Establishment**
   - Device A verbindet sich mit Device B über Bifrost (WebSocket)
   - Heimdall auf Device B prüft Permissions
   - gRPC-Verbindung wird über Bifrost etabliert

3. **Action Routing**
   - Device A sendet `ThorAction` via gRPC an Device B
   - Type-safe Kommunikation mit Protobuf
   - Bessere Performance als WebSocket für einzelne Requests

4. **Action Execution**
   - Thor auf Device B führt Action aus
   - `ThorResult` wird zurück an Device A via gRPC gesendet
   - Streaming möglich für lange Action-Executions

5. **Response**
   - Device A empfängt Result
   - User erhält Response
   - gRPC-Verbindung kann wiederverwendet werden für weitere Actions

## Network Expansion (Phase 4)

### WAN Connectivity

**IP-based Connections**
- **Public IP Support**: Devices können über öffentliche IPs verbunden werden
- **Dynamic IP Handling**: Umgang mit dynamischen IP-Adressen
- **NAT Traversal**: Unterstützung für NAT-Netzwerke
- **Port Forwarding**: Automatische oder manuelle Port-Forwarding-Konfiguration

**Connection Types**
- **Direct IP**: Direkte Verbindung über IP-Adresse (nur bei expliziter Erlaubnis bei Asgard)
- **Domain-based**: Verbindung über Domain-Name (nur bei expliziter Erlaubnis bei Asgard)
- **Relay through Server**: Verbindung über Relay-Server (Asgard/Yggdrasil) - Hauptmethode
- **Yggdrasil als Registry**: Hauptsächlich über Yggdrasil als zentrale Registry

### Enhanced Routing

**Routing Strategies**
- **Direct Routing**: Direkte Device-to-Device Verbindung wenn möglich
- **Relay Routing**: Routing über Server wenn direkte Verbindung nicht möglich
- **Hybrid Routing**: Kombination aus Direct und Relay

**Routing Features**
- **Path Optimization**: Optimierung der Routing-Pfade
- **Load Balancing**: Lastverteilung über mehrere Pfade
- **Failover**: Automatisches Failover bei Verbindungsausfall
- **Quality-based Routing**: Routing basierend auf Connection-Quality

### Connection Management

**Connection Types**
- **Local Connections**: Verbindungen im lokalen Netzwerk
- **WAN Connections**: Verbindungen über das Internet
- **Hybrid Connections**: Kombination aus Local und WAN

**Connection Features**
- **Connection Pooling**: Pool von Verbindungen
- **Connection Reuse**: Wiederverwendung von Verbindungen
- **Connection Monitoring**: Überwachung von Verbindungen
- **Automatic Reconnection**: Automatische Wiederverbindung (sofort + Exponential Backoff)
- **Error Recovery**: Robustes Error-Handling für Verbindungsfehler
- **Fallback-Routing**: Fallback zu alternativen Routen bei Fehlern

### NAT Traversal

**Automatisches NAT-Traversal**
- **Automatisch bevorzugt**: Automatisches NAT-Traversal wird stark bevorzugt
- **STUN**: STUN-Protokoll für NAT-Discovery
- **TURN**: TURN-Server für Relay wenn NAT-Traversal nicht möglich (Yggdrasil/Asgard als TURN-Server)
- **ICE**: ICE-Protokoll für optimalen Pfad
- **Fallback auf manuelle Konfiguration**: Falls automatisch nicht möglich, Fallback auf manuelle Port-Forwarding-Konfiguration

### Dynamic IP Handling

**Kombination: DDNS wenn konfiguriert, sonst Relay über Yggdrasil**
- **DDNS**: Dynamic DNS für Domain-Names (wenn User konfiguriert)
- **IP Update Service**: Service für IP-Updates
- **Connection Refresh**: Automatische Connection-Refresh bei IP-Änderung
- **Yggdrasil-Relay**: Falls DDNS nicht konfiguriert, automatisch über Yggdrasil-Relay
- **Sicherheit**: Muss sicher sein und nicht zu kompliziert

## Implementierungs-Notizen

- Sollte als Native Application oder Electron/Tauri App sein
- Muss System-Integration haben
- Sollte Offline-Funktionalität unterstützen
- Muss verschiedene Audio-Formate unterstützen
- Sollte GPU-Acceleration für LLMs haben
- Muss Security-Best-Practices folgen
- **Performance**: Muss optimiert sein für Desktop-Hardware
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss robuste Security-Mechanismen haben

