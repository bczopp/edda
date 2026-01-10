# Asgard - Server Implementation

## Übersicht

Asgard ist die Server-Implementation für Heimserver. Er stellt erweiterte Server-Funktionalität bereit und kann als zentraler Hub für das lokale Netzwerk fungieren.

## Projektstruktur

```
asgard/
├── src/
│   ├── server/            # Server Core
│   │   ├── main.ts
│   │   ├── config/
│   │   └── platform/
│   ├── services/          # Server Services
│   │   ├── device-registry/
│   │   ├── network-manager/
│   │   ├── routing/
│   │   └── storage/
│   ├── api/              # API Endpoints
│   │   ├── devices/
│   │   ├── network/
│   │   └── admin/
│   └── utils/
├── config/
├── migrations/           # Database Migrations
└── tests/
```

## Features

### Core Server Features

- **Device Registry**: Zentrale Registry für alle Devices im Netzwerk
- **Network Management**: Verwaltung des Software-Netzwerks (Nine Realms)
- **Message Routing**: Routing von Messages zwischen Devices
- **Connection Management**: Verwaltung von Device-Verbindungen

### Advanced Features

- **Network Discovery**: Erweiterte Device-Discovery
- **Load Balancing**: Lastverteilung für Requests
- **Caching**: Caching von häufig verwendeten Daten
- **Analytics**: Netzwerk-Analytics und Monitoring

### Server-Specific Features

- **Persistent Storage**: Datenbank für Device-Registry, etc.
- **Background Services**: Hintergrund-Services für Wartung
- **API Server**: REST/GraphQL API für Administration
- **Web Dashboard**: Web-Interface für Server-Verwaltung (optional, ermöglicht User nachzuvollziehen, was passiert)
  - **Input-Methoden**: Web Dashboard unterstützt sowohl Text- als auch Sprach-Input
    - **Text-Eingabe**: Textfeld für manuelle Command-Eingabe
    - **Voice-Eingabe**: Mikrofon-Button für Sprach-Commands (via Huginn/Muninn STT/TTS)
    - **Wechselbar**: User kann jederzeit zwischen Text und Sprache wechseln

## Service Integration

### Device Registry Service
- Registriert alle Devices im Netzwerk
- Verwaltet Device-Metadaten
- Trackt Device-Status
- Device-Discovery-Support
- **Capability-Synchronisation**: Wenn Asgard im Netzwerk vorhanden ist, übernimmt Asgard die Capability-Synchronisation
- **Leitender Server**: Bei mehreren Asgard-Servern im gleichen Netz ist der älteste der leitende Server und übernimmt die Synchronisation

### Network Manager Service
- Verwaltet Network IDs
- Verwaltet Device-Topologie
- Network-Health-Monitoring
- Network-Configuration

### Routing Service
- Message Routing zwischen Devices
- Relay-Funktionalität
- Broadcast/Multicast Support
- Load Balancing

### Lock Management Service
- **Distributed Locking**: Verwaltet Locks für geteilte Resources im lokalen Netzwerk
- **Lock-Registry**: Zentrale Registry für alle aktiven Locks
- **Lock-Expiration**: Verwaltet Lock-Expiration (Timeout)
- **Deadlock-Detection**: Erkennt Deadlocks und löst sie auf
- **Priority-Management**: Verwaltet Prioritäten für Lock-Requests

### Storage Service
- Persistent Storage für Device-Data
- Backup & Restore
- Data Migration
- Query Interface

## Network ID System

### Network ID Structure
- **Network ID**: Eindeutige ID für das Software-Netzwerk
- **Device Mapping**: Mapping von Device-IDs zu Network-IDs
- **Topology**: Netzwerk-Topologie-Verwaltung

### Network ID Generation
- **Erstes Device**: Jedes Device (außer Jötnar) muss in einem Netzwerk agieren, auch ohne Server oder Yggdrasil-Anmeldung
- **Automatische Generierung**: Network ID wird automatisch generiert, wenn Device aktiviert wird (auch ohne Server)
- **Lokale Generierung**: Network ID kann lokal generiert werden, muss nicht von Yggdrasil kommen

### Network ID Synchronisation
- **Automatisch über Yggdrasil**: Wenn User bei Yggdrasil angemeldet ist, werden Network IDs automatisch synchronisiert
- **Lokales Netzwerk (mDNS/Bonjour)**: Devices können sich auch über lokales Netzwerk synchronisieren
- **Außerhalb des Heimnetzes**: Wenn Device außerhalb des Heimnetzes ist und sich bei Yggdrasil anmeldet, wird Network ID übergeben
- **Heimnetz-Beitritt**: Device kann auch direkt ins Heimnetz kommen

### Multi-Network Support
- **User kann mehrere Networks haben**: User kann mehrere Networks haben, jedes Device gehört zu einem Network
- **Network-Auswahl**: Device muss Network wählen, in dem es agieren soll
- **Automatische Auswahl**: Network-Auswahl kann automatisiert werden oder durch Agent geändert werden
- **Network-Wechsel**: Device kann zwischen Networks wechseln (je nach Aufgabe)

### Network Membership Validation
- **Hybrid-Ansatz**: Lokale Validation mit Yggdrasil als Fallback
- **Lokale Validation**: Device validiert Network-Membership lokal (schnell)
- **Yggdrasil-Fallback**: Bei Unsicherheit oder außerhalb des Heimnetzes wird Yggdrasil konsultiert
- **Credential-basiert**: Validation basiert auf User-Credentials

## Enhanced Bifrost

### Features
- **Network Discovery**: Erweiterte Discovery über das Netzwerk
- **Relay Routing**: Routing über Server
- **Message Queuing**: Queue für Offline-Devices
- **Connection Pooling**: Optimierte Connection-Verwaltung

## Database Schema

### Devices Table
- device_id (PK)
- network_id
- world_type
- capabilities (JSON)
- hardware_spec (JSON)
- registered_at
- last_seen
- status

### Connections Table
- connection_id (PK)
- source_device_id
- target_device_id
- established_at
- last_heartbeat
- status

### Network Table
- network_id (PK)
- name
- created_at
- owner_device_id

## API Endpoints

### Device Management
- `GET /api/devices` - List all devices
- `GET /api/devices/:id` - Get device details
- `POST /api/devices` - Register device
- `PUT /api/devices/:id` - Update device
- `DELETE /api/devices/:id` - Unregister device

### Network Management
- `GET /api/networks` - List networks
- `GET /api/networks/:id` - Get network details
- `POST /api/networks` - Create network
- `PUT /api/networks/:id` - Update network

### Routing
- `POST /api/routing/send` - Send message
- `GET /api/routing/status` - Get routing status

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- Database (PostgreSQL/SQLite)
- Web Framework (Express, Fastify, etc.)
- WebSocket Library
- API Framework

## Integration

- **Odin**: Läuft als Hauptprozess auf Asgard
- **Services**: Alle Services werden von Odin koordiniert
- **Bifrost**: Für Device-to-Device Communication mit erweiterten Features
- **Heimdall**: Für Security und Authentication
- **Midgard/Alfheim**: Clients können sich mit Asgard verbinden
- **Jötnar**: IoT-Devices können über Asgard gesteuert werden
- **Yggdrasil**: Für globale Device-Registry und User-Management

## Performance

### Performance-Optimierungen
- **Effizientes Routing**: Optimiertes Message-Routing für minimale Latenz
- **Connection Pooling**: Effizientes Connection-Pooling für WebSocket-Verbindungen
- **Caching**: Intelligentes Caching für Device-Registry und häufig verwendete Daten
- **Database Optimization**: Optimierte Datenbankabfragen mit Indizes
- **Load Balancing**: Lastverteilung für parallele Requests
- **Async Processing**: Asynchrone Verarbeitung für bessere Performance

### Performance-Metriken
- Niedrige Latenz für Message-Routing (< 10ms lokal)
- Hoher Durchsatz für parallele Connections
- Effiziente Device-Discovery (< 1s für lokale Devices)

## Datenschutz

### Datenschutz-Features
- **Lokale Datenverarbeitung**: Daten werden lokal verarbeitet, keine unnötige Cloud-Übertragung
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Datenverschlüsselung**: Sensible Daten werden verschlüsselt gespeichert
- **User Control**: User hat volle Kontrolle über seine Daten
- **Keine Tracking-Daten**: Keine User-Tracking ohne Zustimmung

### Compliance
- **GDPR-konform**: Einhaltung der GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Security-Features
- **Secure Storage**: Verschlüsselte Speicherung von Credentials und Device-Daten
- **TLS Encryption**: Alle Verbindungen sind verschlüsselt (TLS 1.3)
- **Authentication**: Sichere Authentifizierung über Heimdall
- **Authorization**: Granulares Permission-System für Device-Zugriff
- **Network Isolation**: Optionale Netzwerk-Isolation für erhöhte Sicherheit
- **Input Validation**: Umfassende Input-Validierung für alle API-Endpoints

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Audit Logging**: Vollständiges Logging aller Security-relevanten Events
- **Rate Limiting**: Rate Limiting für API-Endpoints zum Schutz vor DDoS

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
- **Direct IP**: Direkte Verbindung über IP-Adresse (nur bei expliziter Erlaubnis)
- **Domain-based**: Verbindung über Domain-Name (nur bei expliziter Erlaubnis)
- **Relay through Server**: Verbindung über Relay-Server (Asgard/Yggdrasil) - Hauptmethode
- **Asgard als Relay**: Asgard kann als Relay-Server fungieren

### Enhanced Routing

**Routing Strategies**
- **Direct Routing**: Direkte Device-to-Device Verbindung wenn möglich
- **Relay Routing**: Routing über Asgard wenn direkte Verbindung nicht möglich
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

### Relay-Funktionalität

**Asgard als Relay**
- **Asgard als Relay**: Asgard kann als Relay-Server fungieren
- **Automatische Auswahl**: System wählt automatisch besten Relay-Server
- **Automatisch bevorzugt**: Automatisch versuchen, Relay bei Bedarf

**Relay-Workflow**
- **Automatisch versuchen**: System versucht automatisch direkte Verbindung
- **Relay bei Bedarf**: Falls direkte Verbindung nicht möglich, automatisch über Relay
- **User kann erzwingen**: User kann Relay-Modus explizit erzwingen

### NAT Traversal

**Automatisches NAT-Traversal**
- **Automatisch bevorzugt**: Automatisches NAT-Traversal wird stark bevorzugt
- **STUN**: STUN-Protokoll für NAT-Discovery
- **TURN**: TURN-Server für Relay wenn NAT-Traversal nicht möglich (Asgard als TURN-Server)
- **ICE**: ICE-Protokoll für optimalen Pfad
- **Asgard braucht NAT-Traversal**: Asgard sollte auch NAT-Traversal-Funktionalität haben
- **Fallback auf manuelle Konfiguration**: Falls automatisch nicht möglich, Fallback auf manuelle Port-Forwarding-Konfiguration

### Dynamic IP Handling

**Kombination: DDNS wenn konfiguriert, sonst Relay über Yggdrasil**
- **DDNS**: Dynamic DNS für Domain-Names (wenn User konfiguriert)
- **IP Update Service**: Service für IP-Updates
- **Connection Refresh**: Automatische Connection-Refresh bei IP-Änderung
- **Yggdrasil-Relay**: Falls DDNS nicht konfiguriert, automatisch über Yggdrasil-Relay
- **Sicherheit**: Muss sicher sein und nicht zu kompliziert

## Implementierungs-Notizen

- Sollte als Service/Daemon laufen
- Muss Persistent Storage haben
- Sollte Web-Dashboard haben
- Muss API für Administration haben
- Sollte Monitoring und Logging haben
- Muss Backup & Restore unterstützen
- Sollte skalierbar sein
- **Performance**: Muss optimiert sein für Server-Hardware und hohen Durchsatz
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Server-Umgebungen

