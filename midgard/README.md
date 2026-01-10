# Midgard - Desktop/Laptop Client

## Übersicht

Midgard ist der Client für Desktop/Laptop-Geräte. Er stellt die vollständige Edda-Funktionalität für Desktop- und Laptop-Computer bereit.

## Zielplattformen

- Windows (10/11)
- macOS
- Linux (Ubuntu, Debian, Fedora, etc.)

## Projektstruktur

```
midgard/
├── src/
│   ├── main/              # Main Application
│   │   ├── odin.ts        # Odin Service Integration
│   │   ├── ui/            # User Interface
│   │   ├── config/        # Configuration
│   │   └── platform/      # Platform-specific Code
│   ├── services/          # Service Integrations
│   │   ├── huginn/        # STT Service
│   │   ├── muninn/        # TTS Service
│   │   ├── freki/         # RAG Service
│   │   ├── geri/          # LLM Service
│   │   └── thor/           # Action Executor
│   ├── actions/            # Action Handlers
│   │   ├── device/
│   │   ├── file/
│   │   ├── network/
│   │   ├── application/
│   │   └── system/
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
- **Local LLM Support**: Ollama, LM Studio
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
- **Input-Methoden**: Frontend unterstützt sowohl Text- als auch Sprach-Input
  - **Text-Eingabe**: Textfeld für manuelle Command-Eingabe
  - **Voice-Eingabe**: Mikrofon-Button für Sprach-Commands
  - **Wechselbar**: User kann jederzeit zwischen Text und Sprache wechseln
- System Tray Integration
- Notification Support
- Settings UI
- Status Dashboard
- Activity Monitoring: Übersicht über laufende Actions und Services

#### Performance
- Full Hardware Utilization
- Multi-threading Support
- GPU Acceleration Support

## Service Integration

### Odin Integration
- Main Process Orchestration
- Event Handling
- State Management

### Huginn/Muninn Integration
- Microphone Input
- Speaker Output
- Audio Device Management

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
- File Operations
- Application Control
- System Commands
- Network Operations

## Configuration

### Installation Configuration
- Model Selection (basierend auf Hardware)
- Service Selection (lokal vs. Cloud)
- Audio Device Configuration
- Network Configuration

### Runtime Configuration
- Model Settings
- Audio Settings
- Security Settings
- Performance Settings

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- Platform-specific APIs
- Audio Libraries
- UI Framework (Electron, Tauri, etc.)

## Integration

- **Odin**: Läuft als Hauptprozess auf Midgard
- **Services**: Alle Services (Huginn/Muninn, Freki, Geri, Thor) werden von Odin koordiniert
- **Bifrost**: Für Device-to-Device Communication
- **Heimdall**: Für Security und Authentication
- **Asgard**: Kann als Server im Netzwerk fungieren
- **Yggdrasil**: Für globale Device-Registry und User-Management

## Datenschutz

### Datenschutz-Features
- **Lokale Datenverarbeitung**: Daten werden bevorzugt lokal verarbeitet
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

2. **Action Routing**
   - Device A sendet `ThorAction` über Bifrost an Device B
   - Heimdall auf Device B prüft Permissions

3. **Action Execution**
   - Thor auf Device B führt Action aus
   - `ThorResult` wird zurück an Device A gesendet

4. **Response**
   - Device A empfängt Result
   - User erhält Response

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

