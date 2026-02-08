# Alfheim - Mobile Platform

## Übersicht

Alfheim ist eine **Platform** für Smartphone- und Tablet-Geräte, ähnlich wie Midgard (Desktop), Asgard (Homeserver), Ragnarok (Terminal) und Jotunheim (IoT). Als Platform ist Alfheim komplett platformspezifisch optimiert und kümmert sich um Connections (Netzwerk, UI, etc.), konvertiert diese zu Anfragen an Services (Odin) und ruft Services via gRPC auf.

**Services sind unabhängig von Platformen**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind in Rust implementiert und unabhängig von Platformen. Platformen kommunizieren mit Services via gRPC.

**Tests ausführen:** Von `alfheim/`: `docker compose -f docker-compose.test.yml run --rm alfheim-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `alfheim/**` läuft [.github/workflows/alfheim.yml](../.github/workflows/alfheim.yml) (Test im Container).

## Zielplattformen

- iOS (iPhone, iPad)
- Android (Smartphone, Tablet)

### Platform-spezifische Implementierung
- **iOS**: Native iOS-APIs, iOS-spezifische Features (Siri Integration, Keychain, Background Tasks, Notifications)
- **Android**: Native Android-APIs, Android-spezifische Features (Google Assistant Integration, Keystore, Background Services, Notifications)
- **Platform-Updates**: Automatische Behandlung von Platform-Updates und API-Änderungen

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
- **Native-UI-Framework**: React Native, Flutter oder Native (iOS/Android)
  - **Platform-spezifische UI**: Platform-spezifische UI-Implementierungen für iOS und Android
  - **Mobile-UI-Performance**: Optimierte Mobile-UI-Performance mit Lazy-Loading und effizientem Rendering
- **Input-Methoden**: Frontend unterstützt sowohl Text- als auch Sprach-Input
  - **Text-Eingabe**: Touch-Keyboard für manuelle Command-Eingabe
  - **Voice-Eingabe**: Mikrofon-Button für Sprach-Commands
  - **Wechselbar**: User kann jederzeit zwischen Text und Sprache wechseln
- **Touch-Optimized Interface**: Touch-optimierte Benutzeroberfläche
- **Gesture Support**: Unterstützung für Gesten (Swipe, Pinch, etc.)
- **Mobile Navigation**: Mobile-optimierte Navigation
- **Responsive Design**: Responsives Design für verschiedene Bildschirmgrößen
- **Activity Monitoring**: Übersicht über laufende Actions und Services

#### Mobile Integration
- **App Control**: Steuerung von Apps auf dem Device
- **Notification Integration**: Platform-spezifische Notification-Integration (iOS/Android)
  - **Notification-Permissions**: Behandlung von Notification-Permissions und Permission-Verweigerungen
  - **Platform-spezifische Notification-Features**: iOS- und Android-spezifische Notification-Features
- **Background Processing**: Background-Processing für Mobile
  - **Background-Tasks**: Background-Tasks auf iOS vs. Android
  - **Background-Processing-Limits**: Behandlung von Background-Processing-Limits
  - **Background-Task-Restrictions**: Behandlung von Background-Task-Restrictions (iOS/Android)
- **Battery Optimization**: Battery-Optimierung für Mobile
  - **Battery-Usage**: Optimierung von Battery-Usage
  - **Battery-Monitoring**: Monitoring von Battery-Usage
  - **Battery-Saver-Modi**: Behandlung von Battery-Saver-Modi
- **Mobile-Permissions**: Verwaltung von Mobile-Permissions (Mikrofon, Kamera, Dateisystem, etc.)
  - **Permission-Requests**: UI für Permission-Requests
  - **Permission-Verweigerungen**: Behandlung von Permission-Verweigerungen
- **App-Integration**: App-Integration für Mobile
  - **Deep-Links**: Deep-Link-Unterstützung für App-Integration
  - **Share-Extensions**: Share-Extensions für iOS/Android
  - **App-Lifecycle**: Behandlung von App-Lifecycle (Foreground/Background)

#### Platform Features
- iOS: Siri Integration (optional)
- Android: Google Assistant Integration (optional)
- Push Notifications
- Location Services (optional)

## Platform-Architektur

### Platform-Rolle

**Alfheim als Platform:**
- **Connections**: Alfheim-Platform kümmert sich um Connections (Netzwerk, UI, etc.)
- **Konvertierung**: Konvertiert Connections zu Anfragen an Services (Odin)
- **Platformspezifisch**: Komplett platformspezifische Implementierung (iOS, Android)
- **Service-Aufrufe**: Ruft Services (Odin, Thor, Freki, Geri, etc.) via gRPC auf

**Service-Unabhängigkeit:**
- **Services in Rust**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind unabhängig von Platformen
- **gRPC-Kommunikation**: Alfheim-Platform kommuniziert mit Services via gRPC
- **Wiederverwendbar**: Services können von verschiedenen Platformen genutzt werden

## Service Integration

### Service-Discovery und Service-Lifecycle

**Service-Unabhängigkeit:**
- Services sind unabhängig von Platformen implementiert
- Ermöglicht flexible Entscheidungen, welche Services auf Alfheim verfügbar sind
- Services können je nach Bedarf und Hardware-Kapazität installiert werden (mobil-optimiert)

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
- Platform startet und stoppt Services basierend auf Verfügbarkeit und Bedarf (battery-aware)
- Health Checks werden implementiert für Service-Status-Überwachung
- Bei Service-Ausfall: Automatische Fallbacks, Restart-Strategie, Service-Fehler werden dem User kommuniziert

### Odin Integration
- **Lightweight Main Process**: Leichtgewichtiger Hauptprozess für Mobile
- **Mobile-Optimized Event Handling**: Mobile-optimiertes Event-Handling
- **Battery-Efficient State Management**: Battery-effizientes State-Management
- **Mobile-spezifische Request-Optimierungen**: Optimierungen für Mobile-Network-Limits
- **Mobile-Network-Limits**: Behandlung von Mobile-Network-Limits (Data-Usage, Bandwidth)

### Huginn/Muninn Integration
- **Mobile Microphone Input**: Mobile-optimierter Mikrofon-Input
- **Mobile Speaker/Headphone Output**: Mobile-optimierter Speaker/Headphone-Output
- **Audio Session Management**: Audio-Session-Management für Mobile
- **Background Audio Support**: Background-Audio-Support für Mobile
- **Mobile-spezifische Audio-Handling**: Mobile-spezifische Audio-Handling-Features
- **Mobile-Audio-Interruptions**: Behandlung von Mobile-Audio-Interruptions (Anrufe, Alarme, etc.)

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
- **Mobile-Optimized Actions**: Mobile-optimierte Actions
- **App Control**: App-Steuerung auf Mobile
- **File Operations (limited)**: Begrenzte Datei-Operationen auf Mobile
- **Network Operations**: Netzwerk-Operationen auf Mobile
- **Mobile-spezifische Action-UI**: Mobile-spezifische UI für Actions und Action-Ergebnisse

## Mobile Constraints & Optimizations

### Performance
- **Battery Life Optimization**: Optimierung der Battery-Life
  - **Battery-Usage**: Optimierung von Battery-Usage
  - **Battery-Monitoring**: Monitoring von Battery-Usage
  - **Battery-Saver-Modi**: Behandlung von Battery-Saver-Modi
- **Network Usage Minimization**: Minimierung von Network-Usage
  - **Mobile-Network-Usage**: Optimierung von Mobile-Network-Usage
  - **Network-Usage-Monitoring**: Monitoring von Network-Usage
  - **Mobile-Network-Limits**: Behandlung von Mobile-Network-Limits
- **Memory Management**: Effizientes Memory-Management für Mobile
- **CPU Usage Optimization**: Optimierung von CPU-Usage für Mobile

### Functionality
- **Reduced Local Processing**: Reduzierte lokale Verarbeitung auf Mobile
- **Cloud-First Approach**: Cloud-First-Ansatz für Mobile
- **Offline Mode (limited)**: Begrenzter Offline-Mode für Mobile
- **Background Processing Limits**: Behandlung von Background-Processing-Limits

### User Experience
- **Quick Response Times**: Schnelle Response-Zeiten für Mobile
- **Minimal Data Usage**: Minimale Data-Usage für Mobile
- **Intuitive Mobile UI**: Intuitive Mobile-UI
- **Accessibility Support**: Accessibility-Unterstützung für Mobile

### Mobile-Resource-Management
- **Mobile-Resource-Usage**: Überwachung von Mobile-Resource-Usage (Memory, CPU, Battery, Network)
- **Mobile-Resource-Limits**: Behandlung von Mobile-Resource-Limits
- **Mobile-Resource-Exhaustion**: Behandlung von Mobile-Resource-Exhaustion (Fallback, Throttling)

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

### Alfheim-spezifische Settings

**Chat-Management:**
- **Beliebig viele Chats**: Platform muss ermöglichen, quasi beliebig viele Chats zu starten
- **Chat-Leitung**: Chats können direkt an Götter geleitet werden (z.B. Frigg-Chat)
- **Chat-Flags**: Flags in Settings steuern, ob ein Chat direkt an einen Gott geleitet wird oder über Odin läuft

## Configuration

### Installation Configuration
- **Cloud Service Selection**: Auswahl von Cloud-Services
- **Audio Quality (battery vs. quality)**: Audio-Qualität (Battery vs. Quality)
- **Network Usage Limits**: Network-Usage-Limits für Mobile
- **Background Processing Settings**: Background-Processing-Einstellungen

### Runtime Configuration
- **Model Selection**: Model-Auswahl (lokal oder Cloud, basierend auf Konfiguration)
  - **Mobile-spezifische Model-Auswahl**: Mobile-spezifische UI für Model-Auswahl
  - **Model-Updates**: Behandlung von Model-Updates auf Mobile
- **Audio Settings**: Audio-Einstellungen für Mobile
- **Notification Settings**: Notification-Einstellungen für Mobile
- **Battery Optimization Settings**: Battery-Optimierungs-Einstellungen

### Mobile-Konfiguration
- **Konfigurations-Speicherung**: Mobile-spezifische Speicherung von Konfigurationen (verschlüsselt)
- **Mobile-spezifische Konfigurations-UI**: Mobile-optimierte UI für Konfigurationen
- **Konfigurations-Synchronisation**: Synchronisation von Konfigurationen zwischen Devices (optional)

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Alfheim sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- Mobile Platform SDKs (iOS/Android)
- Audio Libraries (Mobile)
- UI Framework (React Native, Flutter, Native)
- **TypeScript/Frontend-Tooling**: `bun` wird als Package-Manager und Runtime verwendet (npm wird nicht verwendet)
  - Container: offizielles `oven/bun`-Image (kein Node/npm)
  - Integration: `bun install` für Dependencies, `bun run` / `bun test` für Scripts
  - Konfiguration: `package.json` oder `bunfig.toml`
  - Dependencies ausschließlich über `bun` (schneller als npm/pnpm)

## Integration

- **Odin**: Läuft als Hauptprozess auf Alfheim
- **Services**: Alle Services werden von Odin koordiniert
- **Bifrost**: Für Device-to-Device Communication
  - **Bifrost-Integration**: Mobile-optimierte Bifrost-Integration
  - **Mobile-Network-Optimierungen**: Mobile-Network-Optimierungen für Bifrost
  - **Mobile-Network-Wechsel**: Behandlung von Mobile-Network-Wechseln (WiFi ↔ Mobile Data)
- **Heimdall**: Für Security und Authentication
- **Midgard/Asgard**: Kann mit Desktop/Server verbunden werden
- **Yggdrasil**: Für globale Device-Registry und User-Management

### Cross-Device Actions
- **Mobile-spezifische Action-UI**: Mobile-optimierte UI für Cross-Device Actions
- **Action-Ergebnisse**: Anzeige von Action-Ergebnissen von anderen Devices auf Mobile

## Datenschutz

### Datenschutz-Features
- **Minimale Datenübertragung**: Nur notwendige Daten werden übertragen
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, wo möglich
  - **Mobile-Datenverarbeitung**: Mobile-optimierte lokale Datenverarbeitung
  - **Mobile-spezifische Datenschutz-Features**: Mobile-spezifische Datenschutz-Features
  - **Mobile-Datenschutz-Präferenzen**: UI für Mobile-Datenschutz-Präferenzen
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
- **Secure Storage**: Verschlüsselte Speicherung von Credentials und Tokens
  - **Mobile-Secure-Storage**: Platform-spezifische Secure-Storage-APIs (iOS Keychain, Android Keystore)
  - **Mobile-Key-Management**: Verwaltung von Keys auf Mobile
- **TLS Encryption**: Alle Netzwerk-Verbindungen sind verschlüsselt
- **Authentication**: Sichere Authentifizierung über Heimdall
- **Permission System**: Granulares Permission-System für Actions
- **App Sandboxing**: Nutzung von Platform-Sandboxing (iOS/Android)
  - **Platform-spezifische Sandbox-Features**: iOS- und Android-spezifische Sandbox-Features
  - **Sandbox-Restrictions**: Behandlung von Sandbox-Restrictions
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

