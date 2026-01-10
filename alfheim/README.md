# Alfheim - Mobile Client

## Übersicht

Alfheim ist der Client für Smartphone- und Tablet-Geräte. Er stellt eine optimierte Version der Edda-Funktionalität für mobile Geräte bereit.

## Zielplattformen

- iOS (iPhone, iPad)
- Android (Smartphone, Tablet)

## Projektstruktur

```
alfheim/
├── src/
│   ├── ios/              # iOS Implementation
│   │   ├── App/
│   │   ├── Services/
│   │   └── Utils/
│   ├── android/          # Android Implementation
│   │   ├── app/
│   │   ├── services/
│   │   └── utils/
│   └── shared/           # Shared Code (React Native, Flutter, etc.)
│       ├── services/
│       ├── ui/
│       └── utils/
├── config/
└── resources/
```

## Features

### Core Features

- **User Input**: Frontend unterstützt sowohl Text- als auch Sprach-Input
  - **Text-Input**: User kann Commands als Text eingeben
  - **Voice-Input**: User kann Commands per Sprache übermitteln (via Huginn/Muninn STT/TTS)
  - **Flexibel**: User kann zwischen Text und Sprache wählen
- **LLM Integration**: Via Geri - Model-Auswahl basierend auf Konfiguration
- **RAG Support**: Via Freki
- **Action Execution**: Via Thor (mobile-optimiert)
- **Device-to-Device Communication**: Via Bifrost

### Mobile-Specific Features

#### Mobile UI/UX (Optional)
- **Optionales Frontend**: User kann optional ein Frontend nutzen, um nachzuvollziehen, was im System passiert
- **Input-Methoden**: Frontend unterstützt sowohl Text- als auch Sprach-Input
  - **Text-Eingabe**: Touch-Keyboard für manuelle Command-Eingabe
  - **Voice-Eingabe**: Mikrofon-Button für Sprach-Commands
  - **Wechselbar**: User kann jederzeit zwischen Text und Sprache wechseln
- Touch-Optimized Interface
- Gesture Support
- Mobile Navigation
- Responsive Design
- Activity Monitoring: Übersicht über laufende Actions und Services

#### Mobile Integration
- App Control
- Notification Integration
- Background Processing
- Battery Optimization

#### Platform Features
- iOS: Siri Integration (optional)
- Android: Google Assistant Integration (optional)
- Push Notifications
- Location Services (optional)

## Service Integration

### Odin Integration
- Lightweight Main Process
- Mobile-Optimized Event Handling
- Battery-Efficient State Management

### Huginn/Muninn Integration
- Mobile Microphone Input
- Mobile Speaker/Headphone Output
- Audio Session Management
- Background Audio Support

### Freki Integration
- Cloud-based Vector Database
- Optimized for Mobile Bandwidth
- Caching Strategy

### Geri Integration
- **Model-Auswahl basierend auf Konfiguration**: 
  - Konfiguration kann vom Device selbst kommen
  - Oder vom verbundenen Desktop/Server (Midgard/Asgard)
  - User kann explizit Model wählen
  - Oder automatische Auswahl (beste Wahl für aktuelle Situation)
- **Keine Bevorzugung von Cloud LLMs**: 
  - Cloud LLM Provider (OpenAI, Anthropic, etc.) sind nur verfügbar, wenn User entsprechende API-Keys/Credentials hinterlegt hat
  - Lokale Models haben gleiche Priorität
  - Model-Auswahl basiert auf Requirements, nicht auf Provider-Typ
- **Unified Model Access**: Geri kann zu jedem Model verbinden (lokal oder Cloud), basierend auf Konfiguration

### Thor Integration
- Mobile-Optimized Actions
- App Control
- File Operations (limited)
- Network Operations

## Mobile Constraints & Optimizations

### Performance
- Battery Life Optimization
- Network Usage Minimization
- Memory Management
- CPU Usage Optimization

### Functionality
- Reduced Local Processing
- Cloud-First Approach
- Offline Mode (limited)
- Background Processing Limits

### User Experience
- Quick Response Times
- Minimal Data Usage
- Intuitive Mobile UI
- Accessibility Support

## Configuration

### Installation Configuration
- Cloud Service Selection
- Audio Quality (battery vs. quality)
- Network Usage Limits
- Background Processing Settings

### Runtime Configuration
- Model Selection (lokal oder Cloud, basierend auf Konfiguration)
- Audio Settings
- Notification Settings
- Battery Optimization Settings

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- Mobile Platform SDKs (iOS/Android)
- Audio Libraries (Mobile)
- UI Framework (React Native, Flutter, Native)

## Integration

- **Odin**: Läuft als Hauptprozess auf Alfheim
- **Services**: Alle Services werden von Odin koordiniert
- **Bifrost**: Für Device-to-Device Communication
- **Heimdall**: Für Security und Authentication
- **Midgard/Asgard**: Kann mit Desktop/Server verbunden werden
- **Yggdrasil**: Für globale Device-Registry und User-Management

## Datenschutz

### Datenschutz-Features
- **Minimale Datenübertragung**: Nur notwendige Daten werden übertragen
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, wo möglich
- **Keine Tracking-Daten**: Keine User-Tracking ohne explizite Zustimmung
- **Datenverschlüsselung**: Sensible Daten werden verschlüsselt gespeichert und übertragen
- **User Control**: User hat volle Kontrolle über seine Daten

### Compliance
- **GDPR-konform**: Einhaltung der GDPR-Anforderungen
- **App Store Compliance**: Einhaltung von iOS/Android Datenschutz-Richtlinien
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Security-Features
- **Secure Storage**: Verschlüsselte Speicherung von Credentials und Tokens (Keychain/Keystore)
- **TLS Encryption**: Alle Netzwerk-Verbindungen sind verschlüsselt
- **Authentication**: Sichere Authentifizierung über Heimdall
- **Permission System**: Granulares Permission-System für Actions
- **App Sandboxing**: Nutzung von Platform-Sandboxing (iOS/Android)
- **Input Validation**: Umfassende Input-Validierung

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder API-Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Certificate Pinning**: Optional für kritische Verbindungen
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

- Sollte als Native App (iOS/Android) oder Cross-Platform (React Native/Flutter) sein
- Muss Mobile-UI/UX-Best-Practices folgen
- Sollte Battery-Optimierung haben
- Muss Background-Processing-Limits beachten
- Sollte Offline-Mode unterstützen (limitierte Funktionalität)
- Muss App Store Guidelines folgen
- Sollte Push-Notifications unterstützen
- **Performance**: Muss optimiert sein für Mobile-Hardware und Battery-Life
- **Datenschutz**: Muss Privacy-by-Design implementieren und App Store Compliance erfüllen
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Mobile-Umgebungen

