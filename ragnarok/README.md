# Ragnarok - Terminal Agent

## Übersicht

Ragnarok ist ein Terminal-basierter Agent für Coding und Device-Steuerung. Er bietet die gleichen Features wie Midgard/Alfheim/Asgard, nutzt aber eine TUI (Terminal User Interface) statt eines GUI-Frontends. Ragnarok nutzt Odin wie die anderen Projekte - der einzige Unterschied ist die TUI statt GUI.

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
- **Keyboard-Navigation**: Vollständige Keyboard-Navigation
- **Responsive**: Passt sich Terminal-Größe an

### Service Integration

**Gleiche Architektur wie Midgard/Alfheim/Asgard:**
- **Odin**: Hauptprozess - koordiniert alle Services
- **Thor**: Action Executor und Tool-Calling-Agent
- **Brünnhilde (Valkyries)**: Coding-Agent (via Thor)
- **Huginn & Muninn**: STT/TTS Service (optional, für Voice-Commands)
- **Freki**: RAG Service für Context-Enrichment
- **Geri**: LLM Service für Prompt-Processing
- **Bifrost**: Communication Service (optional, für Heimnetz)
- **Heimdall**: Security Service (optional, für Heimnetz)

#### Optional: Heimnetz-Verbindung
- **Optional**: User kann optional Verbindung zum Heimnetz aufbauen
- **Expliziter Aufruf**: Muss als `/`-Command explizit aufgerufen werden
- **Bifrost**: Für Device-to-Device Communication (wenn verbunden)
- **Heimdall**: Für Security und Authentication (wenn verbunden)

### Model Management

#### Mitgeliefertes Model
- **Standard-Model**: Sehr gutes, freies Tool-Calling Model (auch für kommerzielle Zwecke)
- **Herausragende Entscheidungsfindung**: Exzellente Entscheidungsfindung und Gesamtüberblick
- **Schlank**: Optimiert, damit es den Computer nicht lahmlegt
- **Tool-Calling**: Speziell für Tool-Calling optimiert
- **Empfohlenes Model**: Llama 3.1 8B oder ähnlich (zuverlässig, wenige Fehler)

#### Alternative Model-Konfiguration
- **Lokales Model**: User kann auf anderes lokales Model routen
- **API-Keys**: User kann API-Keys und URL für Cloud-Models nutzen
- **Resource-Optimierung**: Wenn externes Model genutzt wird, wird mitgeliefertes Model nicht geladen
- **Konfigurierbar**: Über `ragnarok config` oder Konfigurationsdatei

#### Model-Anbindung
- **llama.cpp**: Primäre Anbindung über llama.cpp für minimale Overhead
- **Direkte Anbindung**: So wenig Overhead wie möglich
- **Rust**: Implementierung in Rust (wie alle anderen Services)

### Claude Code & Cursor Features

#### Claude Code Features
- **Alle Features**: Alle aktuellsten Features von Claude Code werden unterstützt
- **Persistent Execution**: Arbeitet nicht aufhören bis Task vollständig erledigt ist
- **Iterative Improvement**: Verbessert Code iterativ bis zur Vollständigkeit
- **Context-Aware**: Nutzt Codebase-Kontext intelligent

#### Cursor Debug-Mode Features
- **Debug-Mode Support**: Unterstützt Cursor Debug-Mode Features
- **Verbesserungen**: Möglicherweise Verbesserungen gegenüber Cursor Debug-Mode
- **Error-Debugging**: Intelligentes Error-Debugging und -Behebung
- **Step-by-Step Execution**: Step-by-Step Code-Execution mit Debugging

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

## LLM-Konfiguration

### Standard-Konfiguration
- **Per Default**: Alle Valkyries nutzen dasselbe LLM (konfigurierbar)
- **Mitgeliefertes Model**: Standardmäßig wird das mitgelieferte Model verwendet
- **Einheitliche Model-Auswahl**: Brünhild und alle Sub-Valkyries verwenden standardmäßig das gleiche Model
- **Resource-Optimierung**: Wenn externes Model konfiguriert wird, wird mitgeliefertes Model nicht geladen

### Individuelle Konfiguration
- **Konfigurierbar**: Jede Valkyrie kann ein eigenes LLM konfiguriert bekommen
- **Use-Case-spezifisch**: Verschiedene Valkyries können verschiedene Models nutzen (z.B. spezialisierte Coding-Models)
- **Konfiguration**: Über `ragnarok config` oder Konfigurationsdatei
- **Gilt auch außerhalb von Ragnarok**: Diese Konfigurationsmöglichkeit gilt für alle Valkyries-Installationen

### Konfigurierbare Modelle
- **Lokales Model**: Anderes lokales Model über llama.cpp
- **Cloud-Model**: Cloud-Model über API-Keys und URL
- **Pro Valkyrie**: Jede Valkyrie kann eigenes Model haben (konfigurierbar)

### Beispiel-Konfiguration
```json
{
  "defaultLLM": "llama-3.1-8b",
  "valkyries": {
    "brünhild": "llama-3.1-8b",
    "frontend": "llama-3.1-8b",
    "backend": "deepseek-coder-7b",
    "test": "llama-3.1-8b",
    "docs": "llama-3.1-8b"
  }
}
```

## Performance

### Performance-Optimierungen
- **Schlankes Design**: Optimiert, damit es den Computer nicht lahmlegt
- **Rust**: Implementierung in Rust (wie alle anderen Services)
- **Minimaler Overhead**: Direkte Model-Anbindung über llama.cpp
- **Resource-Management**: Intelligentes Resource-Management für optimale Performance
- **Lazy Loading**: Model wird nur geladen, wenn benötigt

### Performance-Metriken
- Schnelle Response-Zeiten (< 1s für einfache Commands)
- Effiziente Model-Inference (minimaler Memory-Overhead)
- Optimierte CLI-Performance (schnelle Command-Processing)

## Datenschutz

### Datenschutz-Features
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, keine unnötige Cloud-Übertragung
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

- **Edda Core Library**: DTOs, Protocols, Utils (Go)
- **Odin**: Main Process Service (wie Midgard/Alfheim/Asgard)
- **Thor**: Action Executor und Tool-Calling-Agent
- **Brünnhilde (Valkyries)**: Coding-Agent (via Thor)
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

