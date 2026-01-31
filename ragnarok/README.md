# Ragnarok - Terminal Platform

## Übersicht

Ragnarok ist eine **Platform** für Terminal-basierte Geräte, ähnlich wie Midgard (Desktop), Alfheim (Mobile), Asgard (Homeserver) und Jotunheim (IoT). Als Platform ist Ragnarok komplett platformspezifisch optimiert und kümmert sich um Connections (Terminal, TUI, etc.), konvertiert diese zu Anfragen an Services (Odin) und ruft Services via gRPC auf.

**Services sind unabhängig von Platformen**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind in Rust implementiert und unabhängig von Platformen. Platformen kommunizieren mit Services via gRPC.

Ragnarok bietet die gleichen Features wie Midgard/Alfheim/Asgard, nutzt aber eine TUI (Terminal User Interface) statt eines GUI-Frontends. Ragnarok nutzt Odin wie die anderen Projekte - der einzige Unterschied ist die TUI statt GUI.

## Zielplattformen

- Windows (10/11)
- macOS
- Linux (Ubuntu, Debian, Fedora, etc.)

## Projektstruktur

```
ragnarok/
├── Cargo.toml
├── src/
│   ├── main.rs          # Main Application
│   ├── lib.rs
│   ├── odin/            # Odin Service Integration
│   ├── tui/             # Terminal User Interface (TUI)
│   │   ├── components/
│   │   │   ├── chat.rs  # Chat-Interface
│   │   │   ├── status.rs # Status-Anzeige
│   │   │   ├── history.rs # History-View
│   │   │   └── config.rs # Config-View
│   │   ├── input.rs     # Input-Handling
│   │   └── renderer.rs  # TUI Renderer
│   ├── services/        # Service Integrations
│   │   ├── huginn.rs    # STT Service (optional)
│   │   ├── muninn.rs    # TTS Service (optional)
│   │   ├── freki.rs     # RAG Service
│   │   ├── geri.rs      # LLM Service
│   │   ├── thor.rs      # Action Executor
│   │   └── network.rs   # Optional: Heimnetz-Verbindung
│   ├── actions/         # Action Handlers
│   │   ├── device.rs
│   │   ├── file.rs
│   │   ├── network.rs
│   │   ├── application.rs
│   │   └── system.rs
│   ├── model/           # Model Management
│   │   ├── llama_cpp.rs # llama.cpp Integration (FFI)
│   │   └── config.rs
│   └── utils/
├── config/
├── models/              # Mitgeliefertes Model
└── tests/
```

## Features

### Core Features

- **Terminal-basiert**: Vollständige Funktionalität über TUI (Terminal User Interface)
- **Nutzt Odin**: Ragnarok nutzt Odin wie Midgard/Alfheim/Asgard - gleiche Architektur
- **TUI statt GUI**: Einziger Unterschied zu anderen Projekten: TUI statt GUI-Frontend
- **Coding-Agent**: Vollständige Coding-Funktionalität über Valkyries (via Odin → Thor → Brünnhilde)
- **Device-Steuerung**: Kann das Device steuern, auf dem es installiert ist (via Odin → Thor)
- **Claude Code Features**: Alle aktuellsten Features von Claude Code
- **Cursor Debug-Mode**: Support für Cursor Debug-Mode Features, möglicherweise verbessert

### TUI (Terminal User Interface)

**TUI als Frontend:**
- **Chat-Interface**: Interaktives Chat-Interface für Commands und Responses
- **Status-Anzeige**: Live-Status von laufenden Tasks und Services
- **History-View**: Anzeige der Command-History
- **Config-View**: Konfigurations-Interface
- **Input-Handling**: Text-Input und Keyboard-Navigation
- **Renderer**: TUI-Rendering-Engine (z.B. mit Ink, Blessed, oder ähnlich)

**TUI-Features:**
- **Text-Input**: User kann Commands als Text eingeben
- **Voice-Input**: Optional Voice-Input via Huginn (STT)
- **Live-Updates**: Live-Updates während Task-Ausführung
- **Multi-Panel**: Mehrere Panels für Chat, Status, History gleichzeitig
  - **Multi-Panel-Layout**: Multi-Panel-Layout-Management
  - **Layout-Management**: Automatisches Layout-Management für verschiedene Panel-Konfigurationen
  - **Terminal-Größen-Änderungen**: Automatische Anpassung bei Terminal-Größen-Änderungen
- **Keyboard-Navigation**: Vollständige Keyboard-Navigation
- **Responsive**: Passt sich Terminal-Größe an

**TUI-Komponenten-Struktur:**
- **Component-Libraries**: Wiederverwendbare TUI-Component-Libraries
- **TUI-Performance**: Optimierte TUI-Performance (Rendering-Optimierungen, Lazy-Loading)

## Platform-Architektur

### Platform-Rolle

**Ragnarok als Platform:**
- **Connections**: Ragnarok-Platform kümmert sich um Connections (Terminal, TUI, etc.)
- **Konvertierung**: Konvertiert Connections zu Anfragen an Services (Odin)
- **Platformspezifisch**: Komplett platformspezifische Implementierung (Terminal-Umgebung)
- **Service-Aufrufe**: Ruft Services (Odin, Thor, Freki, Geri, etc.) via gRPC auf

**Service-Unabhängigkeit:**
- **Services in Rust**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind unabhängig von Platformen
- **gRPC-Kommunikation**: Ragnarok-Platform kommuniziert mit Services via gRPC
- **Wiederverwendbar**: Services können von verschiedenen Platformen genutzt werden

### Service Integration

### Service-Discovery und Service-Lifecycle

**Service-Unabhängigkeit:**
- Services sind unabhängig von Platformen implementiert
- Ermöglicht flexible Entscheidungen, welche Services auf Ragnarok verfügbar sind
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

**Gleiche Architektur wie Midgard/Alfheim/Asgard:**
- **Odin**: Hauptprozess - koordiniert alle Services
  - **Odin-Integration**: Kommunikation zwischen Ragnarok und Odin
  - **TUI-spezifische Request-Optimierungen**: Optimierungen für TUI-spezifische Requests
  - **Odin-Ausfälle**: Robustes Error-Handling bei Odin-Ausfällen
- **Thor**: Action Executor und Tool-Calling-Agent
  - **Thor-Integration**: Darstellung von Actions in der TUI
  - **Action-Status-Anzeigen**: TUI-Anzeigen für Action-Status
  - **Action-Ergebnisse**: Anzeige von Action-Ergebnissen in der TUI
- **Brünnhilde (Valkyries)**: Coding-Agent (via Thor)
  - **Valkyries-Integration**: Darstellung von Coding-Aufgaben in der TUI
  - **Progress-Anzeigen**: Progress-Anzeigen für Valkyries
  - **Valkyrie-Ergebnisse**: Anzeige von Valkyrie-Ergebnissen in der TUI
- **Huginn & Muninn**: STT/TTS Service (optional, für Voice-Commands)
- **Freki**: RAG Service für Context-Enrichment
- **Geri**: LLM Service für Prompt-Processing
- **Bifrost**: Communication Service (optional, für Heimnetz)
- **Heimdall**: Security Service (optional, für Heimnetz)

#### Optional: Heimnetz-Verbindung
- **Optional**: User kann optional Verbindung zum Heimnetz aufbauen
- **Expliziter Aufruf**: Muss als `/`-Command explizit aufgerufen werden
  - **Heimnetz-Aktivierung**: Aktivierung der Heimnetz-Verbindung über `/`-Command
  - **Connection-Status-Anzeigen**: TUI-Anzeigen für Connection-Status
  - **Verbindungsprobleme**: Robustes Error-Handling bei Verbindungsproblemen
- **Bifrost**: Für Device-to-Device Communication (wenn verbunden)
  - **Bifrost-Integration**: Integration von Bifrost in Ragnarok
  - **TUI für Device-Verbindungen**: TUI-Komponenten für Device-Verbindungen
  - **Verbindungsprobleme**: Robustes Error-Handling bei Verbindungsproblemen
- **Heimdall**: Für Security und Authentication (wenn verbunden)

### Model Management

#### Mitgeliefertes Model
- **Standard-Model**: Sehr gutes, freies Tool-Calling Model (auch für kommerzielle Zwecke)
- **Herausragende Entscheidungsfindung**: Exzellente Entscheidungsfindung und Gesamtüberblick
- **Schlank**: Optimiert, damit es den Computer nicht lahmlegt
- **Tool-Calling**: Speziell für Tool-Calling optimiert
- **Empfohlenes Model**: Llama 3.1 8B oder ähnlich (zuverlässig, wenige Fehler)
- **Installation**: Automatische Installation beim ersten Start
- **Model-Updates**: Automatische Updates oder manuelle Update-Funktion

#### Alternative Model-Konfiguration
- **Lokales Model**: User kann auf anderes lokales Model routen
- **API-Keys**: User kann API-Keys und URL für Cloud-Models nutzen
- **Resource-Optimierung**: Wenn externes Model genutzt wird, wird mitgeliefertes Model nicht geladen
- **Konfigurierbar**: Über `ragnarok config` oder Konfigurationsdatei

#### LLM-Service-Integration
- **Geri (via gRPC)**: LLM Service für alle LLM-Anfragen
- **Keine direkte Model-Anbindung**: Ragnarok nutzt Geri Service, LLM-Provider-Details sind Geri-interne Konfiguration
- **Service-basiert**: Konsistent mit allen anderen Platformen

### Claude Code & Cursor Features

#### Claude Code Features
- **Alle Features**: Alle aktuellsten Features von Claude Code werden unterstützt
- **Persistent Execution**: Arbeitet nicht aufhören bis Task vollständig erledigt ist
- **Iterative Improvement**: Verbessert Code iterativ bis zur Vollständigkeit
- **Context-Aware**: Nutzt Codebase-Kontext intelligent
- **Feature-Prioritäten**: Priorisierung von Features basierend auf Wichtigkeit
- **Feature-Implementierung**: Detaillierte Implementierung von Claude Code Features

#### Cursor Debug-Mode Features
- **Debug-Mode Support**: Unterstützt Cursor Debug-Mode Features
- **Verbesserungen**: Möglicherweise Verbesserungen gegenüber Cursor Debug-Mode
- **Error-Debugging**: Intelligentes Error-Debugging und -Behebung
- **Step-by-Step Execution**: Step-by-Step Code-Execution mit Debugging
- **Debug-Features in TUI**: Darstellung von Debug-Features in der TUI

## TUI Interface

### TUI-Komponenten

**Chat-Interface:**
- **Text-Input**: User kann Commands als Text eingeben
- **Voice-Input**: Optional Voice-Input via Huginn (STT)
- **Response-Display**: Anzeige von Responses und Task-Ergebnissen
- **Live-Updates**: Live-Updates während Task-Ausführung

**Status-Panel:**
- **Service-Status**: Status aller Services (Odin, Thor, Geri, etc.)
- **Task-Status**: Status laufender Tasks
- **Progress-Bars**: Fortschrittsanzeigen für Tasks

**History-Panel:**
- **Command-History**: Anzeige der Command-History
- **Task-History**: Anzeige der Task-History
- **Scrollable**: Scrollbare History-Ansicht

**Config-Panel:**
- **Model-Konfiguration**: Konfiguration von LLM-Modellen
- **Service-Konfiguration**: Konfiguration von Services
- **Network-Konfiguration**: Konfiguration für Heimnetz-Verbindung

### Keyboard-Shortcuts

- `Ctrl+C`: Programm beenden
- `Tab`: Zwischen Panels wechseln
- `Enter`: Command senden
- `Esc`: Panel schließen
- `Ctrl+L`: Chat leeren
- `Ctrl+H`: History anzeigen
- `Ctrl+C`: Config öffnen

**Keyboard-Shortcuts-Implementierung:**
- **Keyboard-Shortcuts**: Implementierung von Keyboard-Shortcuts
- **Konfigurierbare Shortcuts**: Konfigurierbare Keyboard-Shortcuts
- **Shortcut-Konflikte**: Behandlung von Shortcut-Konflikten

**Input-Handling:**
- **Text-Input**: Text-Input-Handling in der TUI
- **Voice-Input-Support**: Optional Voice-Input-Support (via Huginn)
- **Input-Fehler**: Robustes Error-Handling bei Input-Fehlern

### Example Usage

```bash
# Start Ragnarok (öffnet TUI)
ragnarok

# In der TUI:
# - Text-Input für Commands
# - Live-Status-Anzeige
# - History-View
# - Config-View
```

## Service-Architektur

### Architektur (gleich wie Midgard/Alfheim/Asgard)

```
User (TUI)
  ↓
Odin (Main Process)
  ↓
├── Geri (LLM) ← Freki (RAG)
├── Huginn (STT, optional)
├── Muninn (TTS, optional)
├── Thor (Action Executor)
│   ↓
│   ├── Brünnhilde (Valkyries)
│   │   ↓
│   │   └── Valkyries (Frontend, Backend, Test, Docs)
│   │       ↓
│   │   └── Strukturierte Ergebnisse zurück an Thor
│   ↓
│   └── Tool-Calling: FILE_OPERATION, SYSTEM_COMMAND, etc.
├── Bifrost (optional, für Heimnetz)
└── Heimdall (optional, für Security)
```

### Unterschied zu Midgard/Alfheim/Asgard

**Einziger Unterschied: TUI statt GUI-Frontend**

- **Midgard/Alfheim/Asgard**: GUI-Frontend (React, Native UI, etc.)
- **Ragnarok**: TUI-Frontend (Terminal User Interface)
- **Gleiche Architektur**: Beide nutzen Odin, Thor, Brünnhilde, etc.
- **Gleiche Features**: Beide haben die gleichen Features, nur unterschiedliche UI

## LLM-Service-Integration

### Geri-Service (via gRPC)
- **LLM-Service**: Ragnarok nutzt Geri-Service für alle LLM-Anfragen
- **Model-Auswahl**: Model-Auswahl erfolgt über Geri-Konfiguration (nicht Ragnarok-spezifisch)
- **Service-basiert**: Konsistent mit allen anderen Platformen (Midgard, Alfheim, Asgard)

### Valkyries-Integration
- **Valkyries nutzen Geri**: Alle Valkyries (Brünnhilde, Frontend, Backend, etc.) kommunizieren mit Geri via gRPC
- **Model-Präferenzen**: Model-Präferenzen können über Geri-Anfragen angegeben werden
- **Keine direkte Model-Verwaltung**: Model-Verwaltung ist Geri-interne Verantwortung

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

### Ragnarok-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Model-Konfiguration (siehe Abschnitt "LLM-Konfiguration")
- TUI-Einstellungen
- Keyboard-Navigation-Einstellungen

**Chat-Management:**
- **Beliebig viele Chats**: Platform muss ermöglichen, quasi beliebig viele Chats zu starten
- **Chat-Leitung**: Chats können direkt an Götter geleitet werden (z.B. Frigg-Chat)
- **Chat-Flags**: Flags in Settings steuern, ob ein Chat direkt an einen Gott geleitet wird oder über Odin läuft

## Performance

### Performance-Optimierungen
- **Schlankes Design**: Optimiert, damit es den Computer nicht lahmlegt
- **Rust**: Implementierung in Rust (wie alle anderen Services)
- **Minimaler Overhead**: Direkte Model-Anbindung über llama.cpp
- **Resource-Management**: Intelligentes Resource-Management für optimale Performance
- **Lazy Loading**: Model wird nur geladen, wenn benötigt

### TUI-Performance
- **TUI-Performance-Optimierungen**: Optimierungen für TUI-Performance
- **Rendering-Optimierungen**: Optimierungen für TUI-Rendering
- **Terminal-Lag**: Behandlung von Terminal-Lag (Throttling, Debouncing)

### Model-Performance
- **Model-Inference-Performance**: Optimierungen für Model-Inference-Performance
- **Model-Caching**: Caching von Model-Responses für bessere Performance
- **Model-Loading-Zeit**: Behandlung von Model-Loading-Zeit (Progress-Anzeigen, Background-Loading)

### Performance-Metriken
- Schnelle Response-Zeiten (< 1s für einfache Commands)
- Effiziente Model-Inference (minimaler Memory-Overhead)
- Optimierte CLI-Performance (schnelle Command-Processing)

## Datenschutz

### Datenschutz-Features
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, keine unnötige Cloud-Übertragung
  - **Lokale Datenverarbeitung**: Bevorzugung von lokaler Datenverarbeitung
  - **TUI-Indikatoren**: TUI-Indikatoren für lokale vs. Cloud-Verarbeitung
  - **Datenschutz-Präferenzen**: UI für Datenschutz-Präferenzen
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **User Control**: User hat volle Kontrolle über seine Daten
- **Code-Privacy**: Code bleibt lokal, wird nicht an Dritte weitergegeben

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Security-Features
- **Sandboxing**: Sandboxing für Code-Execution zum Schutz vor schädlichem Code
  - **Sandboxing-Implementierung**: Detaillierte Sandboxing-Implementierung für Code-Execution
  - **Permission-Systeme**: Granulare Permission-Systeme für Code-Execution
  - **Sandbox-Escape-Versuche**: Erkennung und Behandlung von Sandbox-Escape-Versuchen
- **Input Validation**: Umfassende Validierung aller Inputs
- **Code Review**: Automatische Code-Review für Security-Issues
- **Secure Key Storage**: Sichere Speicherung von API-Keys
- **Permission Checking**: Prüfung von Permissions für Device-Operations
- **Audit Logging**: Logging aller Code-Änderungen für Security-Audits

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder API-Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Vulnerability Scanning**: Automatisches Scanning für bekannte Vulnerabilities im Code
- **Dependency Checking**: Prüfung von Dependencies auf Security-Issues

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Ragnarok sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Odin**: Main Process Service (wie Midgard/Alfheim/Asgard)
- **Thor**: Action Executor und Tool-Calling-Agent
- **Brünnhilde (Valkyries)**: Coding-Agent (via Thor, optional)
- **Geri**: LLM Service
- **Freki**: RAG Service
- **Huginn & Muninn**: STT/TTS Service (optional)
- **Bifrost**: Communication Service (optional, für Heimnetz)
- **Heimdall**: Security Service (optional, für Heimnetz)
- **TUI Library**: ratatui (tui-rs), crossterm oder ähnlich (Rust TUI Libraries)
- **llama.cpp**: Für Model-Anbindung (FFI-Bindings, optional wenn lokales Model genutzt wird)
- Git Library
- File System APIs
- Execution Environment
- CLI Framework

## Integration

- **Odin**: Main Process Service (wie Midgard/Alfheim/Asgard)
- **Thor**: Action Executor und Tool-Calling-Agent
- **Brünnhilde (Valkyries)**: Coding-Agent (via Thor)
- **Valkyries**: Alle Valkyries (Frontend, Backend, Test, Docs)
- **Geri**: LLM Service für Prompt-Processing
- **Freki**: RAG Service für Context-Enrichment
- **Huginn & Muninn**: STT/TTS Service (optional)
- **Bifrost**: Optional für Heimnetz-Verbindung
- **Heimdall**: Optional für Security (wenn Heimnetz verbunden)
- **TUI**: Terminal User Interface (statt GUI-Frontend)
- **Gemeinsame Pakete**: Nutzt die gleichen Pakete wie andere Projekte

## Device Interconnection (Phase 2) - Optional

**Hinweis**: Ragnarok kann optional eine Verbindung zum Heimnetz aufbauen. Die folgenden Features sind nur verfügbar, wenn die Heimnetz-Verbindung aktiviert ist.

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

**Workflow** (nur wenn Heimnetz verbunden)
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

## Network Expansion (Phase 4) - Optional

**Hinweis**: Ragnarok kann optional eine Verbindung zum Heimnetz aufbauen. Die folgenden Features sind nur verfügbar, wenn die Heimnetz-Verbindung aktiviert ist.

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

- **Programmiersprache**: Rust (wie alle anderen Services)
- **TUI Library**: ratatui (tui-rs), crossterm oder ähnlich (Rust TUI Libraries)
- **Terminal-basiert**: Vollständig über TUI, kein GUI-Frontend
- **Nutzt Odin**: Gleiche Architektur wie Midgard/Alfheim/Asgard
- **TUI statt GUI**: Einziger Unterschied: TUI-Frontend statt GUI-Frontend
- **Performance**: Rust für maximale Performance und Memory-Safety
- **Mitgeliefertes Model**: Sehr gutes, freies Tool-Calling Model (Llama 3.1 8B oder ähnlich)
- **llama.cpp Integration**: Direkte Anbindung über llama.cpp für minimale Overhead
- **Schlank**: Optimiert, damit es den Computer nicht lahmlegt
- **Claude Code Features**: Alle aktuellsten Features von Claude Code
- **Cursor Debug-Mode**: Support für Cursor Debug-Mode Features, möglicherweise verbessert
- **Optional Heimnetz**: User kann optional Verbindung zum Heimnetz aufbauen (expliziter `/`-Command)
- **Model-Konfiguration**: User kann auf anderes lokales Model routen oder API-Keys/URL nutzen
- **Resource-Optimierung**: Mitgeliefertes Model wird nicht geladen, wenn externes Model genutzt wird
- **LLM-Konfiguration pro Valkyrie**: Per Default nutzen alle Valkyries dasselbe LLM, aber jede Valkyrie kann individuell konfiguriert werden (gilt für alle Installationen)
- **Performance**: Muss optimiert sein für schnelle CLI-Performance und minimale Resource-Nutzung
- **Datenschutz**: Muss Privacy-by-Design implementieren und Code-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Code-Execution

