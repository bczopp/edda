# Asgard - Homeserver Platform

## Übersicht

Asgard ist eine **Platform** für Heimserver, ähnlich wie Midgard (Desktop), Alfheim (Mobile), Ragnarok (Terminal) und Jotunheim (IoT). Als Platform ist Asgard komplett platformspezifisch optimiert und kümmert sich um Connections (Netzwerk, UI, etc.), konvertiert diese zu Anfragen an Services (Odin) und ruft Services via gRPC auf.

**Services sind unabhängig von Platformen**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind in Rust implementiert und unabhängig von Platformen. Platformen kommunizieren mit Services via gRPC.

**Tests ausführen:** Von `asgard/`: `docker compose -f docker-compose.test.yml run --rm asgard-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `asgard/**` läuft die Pipeline [.github/workflows/asgard.yml](../.github/workflows/asgard.yml) (Test im Container, Lint).

## Projektstruktur

```
asgard/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── frontend/          # Frontend (TypeScript/React) - optional Web Dashboard
│   │   ├── components/
│   │   ├── pages/
│   │   └── ...
│   ├── server/            # Server Core (Rust)
│   │   ├── config/
│   │   └── platform/
│   ├── services/          # Server Services (Rust)
│   │   ├── device_registry/
│   │   ├── network_manager/
│   │   ├── routing/
│   │   └── storage/
│   ├── api/              # API Endpoints (Rust)
│   │   ├── devices/
│   │   ├── network/
│   │   └── admin/
│   └── utils/
├── config/
├── migrations/           # Database Migrations
└── tests/
```

### Server-spezifische Implementierung
- **Verschiedene Server-Umgebungen**: Unterstützung für verschiedene Server-Umgebungen (Linux, Docker, Cloud)
- **Server-spezifische Features**: Platform-spezifische Features je nach Server-Umgebung
- **Server-Updates**: Automatische Behandlung von Server-Updates und API-Änderungen

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
  - **API-Implementierung**: API-Framework (REST, GraphQL)
  - **API-Versionierung**: API-Versionierung für kompatible Updates
  - **API-Authentication**: Sichere API-Authentication (Token, OAuth, etc.)
  - **API-Endpoints**: Strukturierte API-Endpoints
  - **API-Dokumentation**: Umfassende API-Dokumentation (OpenAPI, GraphQL Schema)
  - **API-Fehler**: Robustes Error-Handling für API-Fehler

## API-Spezifikation

### API-Authentifizierung

**Token-basierte Authentifizierung:**
- **Token-Format**: JWT (JSON Web Token) oder Heimdall-Token
- **Token-Header**: `Authorization: Bearer <token>`
- **Token-Generierung**: Token wird nach erfolgreicher Authentifizierung generiert (via Heimdall)
- **Token-Validierung**: Jeder Request wird auf gültiges Token geprüft
- **Token-Expiration**: Token haben Ablaufzeit (konfigurierbar)
- **Refresh-Token**: Refresh-Token für Token-Erneuerung (optional)

**Authentifizierungs-Workflow:**
1. **Device-Authentifizierung**: Device authentifiziert sich via Heimdall (Challenge-Response)
2. **Token-Generierung**: Heimdall generiert Token für Device
3. **Token-Usage**: Device sendet Token in `Authorization`-Header bei jedem API-Request
4. **Token-Validierung**: Asgard validiert Token bei jedem Request (via Heimdall oder lokal)
5. **Token-Refresh**: Bei abgelaufenem Token kann Refresh-Token verwendet werden

**OAuth-Integration (Optional):**
- **OAuth-Provider**: Unterstützung für OAuth-Provider (optional)
- **OAuth-Flow**: Standard OAuth 2.0 Flow
- **OAuth-Token**: OAuth-Token wird in Heimdall-Token konvertiert

**API-Key-Authentifizierung (Optional):**
- **API-Key**: Statische API-Keys für Server-zu-Server-Kommunikation
- **API-Key-Header**: `X-API-Key: <api-key>`
- **API-Key-Validierung**: API-Keys werden lokal validiert

### Request/Response-Formate

**Request-Format:**
- **Content-Type**: `application/json`
- **Accept**: `application/json`
- **Encoding**: UTF-8
- **Request-Body**: JSON-Format für POST/PUT-Requests

**Response-Format:**
- **Content-Type**: `application/json`
- **Encoding**: UTF-8
- **Response-Body**: JSON-Format

**Standard-Response-Struktur:**
```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "timestamp": "2024-01-01T00:00:00Z"
}
```

**Error-Response-Struktur:**
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error message",
    "details": { ... }
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

**HTTP-Status-Codes:**
- `200 OK`: Erfolgreiche Request
- `201 Created`: Ressource erfolgreich erstellt
- `400 Bad Request`: Ungültige Request (Validierungsfehler)
- `401 Unauthorized`: Authentifizierung fehlgeschlagen
- `403 Forbidden`: Keine Berechtigung
- `404 Not Found`: Ressource nicht gefunden
- `429 Too Many Requests`: Rate-Limit überschritten
- `500 Internal Server Error`: Server-Fehler

### Rate-Limits

**Rate-Limit-Strategie:**
- **Token-basiert**: Rate-Limits werden pro Token/Device angewendet
- **IP-basiert**: Zusätzliche IP-basierte Rate-Limits (für DDoS-Schutz)
- **Endpoint-spezifisch**: Verschiedene Rate-Limits für verschiedene Endpoints

**Rate-Limit-Werte (Beispiele):**
- **Standard-Endpoints**: 100 Requests/Minute pro Device
- **Device-Registry-Endpoints**: 50 Requests/Minute pro Device
- **Network-Management-Endpoints**: 20 Requests/Minute pro Device
- **Admin-Endpoints**: 10 Requests/Minute pro Device

**Rate-Limit-Headers:**
- `X-RateLimit-Limit`: Maximale Anzahl von Requests
- `X-RateLimit-Remaining`: Verbleibende Requests
- `X-RateLimit-Reset`: Zeitpunkt, wann Rate-Limit zurückgesetzt wird

### API-Endpoints

**Device Management:**
- `GET /api/v1/devices` - List all devices
  - **Auth**: Required (Token)
  - **Response**: `{ "devices": [ ... ] }`
- `GET /api/v1/devices/:id` - Get device details
  - **Auth**: Required (Token)
  - **Response**: `{ "device": { ... } }`
- `POST /api/v1/devices` - Register device
  - **Auth**: Required (Token)
  - **Request**: `{ "device_id": "...", "name": "...", "type": "..." }`
  - **Response**: `{ "device": { ... } }`
- `PUT /api/v1/devices/:id` - Update device
  - **Auth**: Required (Token)
  - **Request**: `{ "name": "...", "type": "..." }`
  - **Response**: `{ "device": { ... } }`
- `DELETE /api/v1/devices/:id` - Unregister device
  - **Auth**: Required (Token)
  - **Response**: `{ "success": true }`

**Network Management:**
- `GET /api/v1/networks` - List all networks
  - **Auth**: Required (Token)
  - **Response**: `{ "networks": [ ... ] }`
- `GET /api/v1/networks/:id` - Get network details
  - **Auth**: Required (Token)
  - **Response**: `{ "network": { ... } }`
- `POST /api/v1/networks` - Create network
  - **Auth**: Required (Token)
  - **Request**: `{ "name": "...", "devices": [ ... ] }`
  - **Response**: `{ "network": { ... } }`
- `PUT /api/v1/networks/:id` - Update network
  - **Auth**: Required (Token)
  - **Request**: `{ "name": "...", "devices": [ ... ] }`
  - **Response**: `{ "network": { ... } }`
- `DELETE /api/v1/networks/:id` - Delete network
  - **Auth**: Required (Token)
  - **Response**: `{ "success": true }`

**Connection Management:**
- `GET /api/v1/connections` - List all connections
  - **Auth**: Required (Token)
  - **Response**: `{ "connections": [ ... ] }`
- `GET /api/v1/connections/:id` - Get connection details
  - **Auth**: Required (Token)
  - **Response**: `{ "connection": { ... } }`
- `POST /api/v1/connections` - Create connection
  - **Auth**: Required (Token)
  - **Request**: `{ "device_id": "...", "target_device_id": "..." }`
  - **Response**: `{ "connection": { ... } }`
- `DELETE /api/v1/connections/:id` - Close connection
  - **Auth**: Required (Token)
  - **Response**: `{ "success": true }`

**Admin Endpoints (Optional):**
- `GET /api/v1/admin/stats` - Get server statistics
  - **Auth**: Required (Token, Admin-Role)
  - **Response**: `{ "stats": { ... } }`
- `GET /api/v1/admin/logs` - Get server logs
  - **Auth**: Required (Token, Admin-Role)
  - **Query-Params**: `?level=...&limit=...&offset=...`
  - **Response**: `{ "logs": [ ... ] }`
- **Web Dashboard**: Web-Interface für Server-Verwaltung (optional, ermöglicht User nachzuvollziehen, was passiert)
  - **Dashboard-Implementierung**: Web-Framework für Dashboard (React, Vue, etc.)
  - **Real-time-Updates**: Real-time-Updates über WebSockets
  - **Dashboard-Performance**: Optimierte Dashboard-Performance (Lazy-Loading, Caching)
  - **Input-Methoden**: Web Dashboard unterstützt sowohl Text- als auch Sprach-Input
    - **Text-Eingabe**: Textfeld für manuelle Command-Eingabe
    - **Voice-Eingabe**: Mikrofon-Button für Sprach-Commands (via Huginn/Muninn STT/TTS)
    - **Wechselbar**: User kann jederzeit zwischen Text und Sprache wechseln

## Platform-Architektur

### Platform-Rolle

**Asgard als Platform:**
- **Connections**: Asgard-Platform kümmert sich um Connections (Netzwerk, UI, etc.)
- **Konvertierung**: Konvertiert Connections zu Anfragen an Services (Odin)
- **Platformspezifisch**: Komplett platformspezifische Implementierung (Server-Umgebung)
- **Service-Aufrufe**: Ruft Services (Odin, Thor, Freki, Geri, etc.) via gRPC auf

**Service-Unabhängigkeit:**
- **Services in Rust**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind unabhängig von Platformen
- **gRPC-Kommunikation**: Asgard-Platform kommuniziert mit Services via gRPC
- **Wiederverwendbar**: Services können von verschiedenen Platformen genutzt werden

## Service Integration

### Service-Discovery und Service-Lifecycle

**Service-Unabhängigkeit:**
- Services sind unabhängig von Platformen implementiert
- Ermöglicht flexible Entscheidungen, welche Services auf Asgard verfügbar sind
- Services können je nach Bedarf und Server-Kapazität installiert werden

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

### Device Registry Service
- **Device-Registration**: Registriert alle Devices im Netzwerk
  - **Device-Validation**: Validierung von Device-Registrierungen
  - **Device-Duplikate**: Behandlung von Device-Duplikaten (Konflikt-Auflösung)
- **Device-Metadaten**: Verwaltung von Device-Metadaten
- **Device-Status**: Tracking von Device-Status
- **Device-Discovery-Support**: Unterstützung für Device-Discovery
- **Capability-Synchronisation**: Wenn Asgard im Netzwerk vorhanden ist, übernimmt Asgard die Capability-Synchronisation
  - **Capability-Validation**: Validierung von Capabilities
  - **Capability-Konflikte**: Behandlung von Capability-Konflikten
- **Leitender Server**: Bei mehreren Asgard-Servern im gleichen Netz ist der älteste der leitende Server und übernimmt die Synchronisation
  - **Failover-Mechanismen**: Automatische Failover-Mechanismen bei Server-Ausfällen
  - **Server-Ausfälle**: Robustes Error-Handling bei Server-Ausfällen

### Network Manager Service
- **Network-ID-Verwaltung**: Verwaltung von Network IDs
  - **Network-ID-Generierung**: Automatische Generierung von Network IDs
  - **Network-ID-Validation**: Validierung von Network IDs
  - **Network-ID-Konflikte**: Behandlung von Network-ID-Konflikten
- **Device-Topologie**: Verwaltung von Device-Topologie
- **Network-Health-Monitoring**: Monitoring der Network-Health
- **Network-Configuration**: Konfiguration von Networks

### Routing Service
- Message Routing zwischen Devices
- Relay-Funktionalität
- Broadcast/Multicast Support
- Load Balancing

### Lock Management Service
- **Distributed Locking**: Verwaltet Locks für geteilte Resources im lokalen Netzwerk
  - **Lock-Coordination**: Koordination von Locks zwischen Asgard-Servern
  - **Lock-Konflikte**: Behandlung von Lock-Konflikten (Priorität, Timeout)
- **Lock-Registry**: Zentrale Registry für alle aktiven Locks
- **Lock-Expiration**: Verwaltet Lock-Expiration (Timeout)
- **Deadlock-Detection**: Erkennt Deadlocks und löst sie auf
  - **Deadlock-Resolution**: Automatische Deadlock-Resolution-Mechanismen
  - **Deadlock-Prevention**: Präventive Maßnahmen zur Deadlock-Vermeidung
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
- **Erstes Device**: Jedes Device (außer Jotunheim) muss in einem Netzwerk agieren, auch ohne Server oder Yggdrasil-Anmeldung
- **Automatische Generierung**: Network ID wird automatisch generiert, wenn Device aktiviert wird (auch ohne Server)
- **Lokale Generierung**: Network ID kann lokal generiert werden, muss nicht von Yggdrasil kommen

### Network ID Synchronisation
- **Automatisch über Yggdrasil**: Wenn User bei Yggdrasil angemeldet ist, werden Network IDs automatisch synchronisiert
  - **Yggdrasil-Synchronisation**: Automatische Synchronisation über Yggdrasil
  - **Synchronisations-Konflikte**: Behandlung von Synchronisations-Konflikten
- **Lokales Netzwerk (mDNS/Bonjour)**: Devices können sich auch über lokales Netzwerk synchronisieren
  - **Lokale Synchronisation**: Automatische lokale Synchronisation über mDNS/Bonjour
  - **Synchronisations-Konflikte**: Behandlung von lokalen Synchronisations-Konflikten
- **Außerhalb des Heimnetzes**: Wenn Device außerhalb des Heimnetzes ist und sich bei Yggdrasil anmeldet, wird Network ID übergeben
- **Heimnetz-Beitritt**: Device kann auch direkt ins Heimnetz kommen

### Multi-Network Support
- **User kann mehrere Networks haben**: User kann mehrere Networks haben, jedes Device gehört zu einem Network
- **Network-Auswahl**: Device muss Network wählen, in dem es agieren soll
- **Automatische Auswahl**: Network-Auswahl kann automatisiert werden oder durch Agent geändert werden
- **Network-Wechsel**: Device kann zwischen Networks wechseln (je nach Aufgabe)
- **Network-Switching-Mechanismen**: Automatische Network-Switching-Mechanismen
- **Network-Membership-Validation**: Validierung von Network-Membership bei Network-Wechsel

### Network Membership Validation
- **Hybrid-Ansatz**: Lokale Validation mit Yggdrasil als Fallback
- **Lokale Validation**: Device validiert Network-Membership lokal (schnell)
- **Yggdrasil-Fallback**: Bei Unsicherheit oder außerhalb des Heimnetzes wird Yggdrasil konsultiert
- **Credential-basiert**: Validation basiert auf User-Credentials

## Enhanced Bifrost

### Network Discovery
- **Erweiterte Device-Discovery**: Erweiterte Discovery über das Netzwerk
- **Discovery-Optimierungen**: Optimierungen für schnelle Device-Discovery
- **Discovery-Timeouts**: Behandlung von Discovery-Timeouts

### Relay Routing
- **Relay-Routing über Asgard**: Routing über Asgard-Server
- **Routing-Optimierungen**: Optimierungen für effizientes Relay-Routing
- **Relay-Ausfälle**: Robustes Error-Handling bei Relay-Ausfällen (Fallback, Retry)

### Message Queuing
- **Message-Queuing für Offline-Devices**: Queue für Offline-Devices
- **Queue-Size-Limits**: Konfigurierbare Queue-Size-Limits
- **Queue-Overflow**: Behandlung von Queue-Overflow (Eviction, Notification)

### Features
- **Network Discovery**: Erweiterte Discovery über das Netzwerk
- **Relay Routing**: Routing über Server
- **Message Queuing**: Queue für Offline-Devices
- **Connection Pooling**: Optimierte Connection-Verwaltung

## Database Integration

### Database-Auswahl
- **PostgreSQL**: Für Production-Server mit hohem Durchsatz
- **SQLite**: Für einfache Setups oder Development
- **Database-Migrations**: Automatische Database-Migrations für Schema-Updates

### Database-Performance
- **Query-Optimierungen**: Optimierte Datenbankabfragen mit Indizes
- **Database-Load**: Effiziente Behandlung von hohem Database-Load
- **Connection-Pooling**: Connection-Pooling für bessere Performance

## Database Schema

### Devices Table
```sql
CREATE TABLE devices (
    device_id UUID PRIMARY KEY,
    network_id UUID,
    world_type VARCHAR(50) NOT NULL,
    capabilities JSONB,
    hardware_spec JSONB,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMP NOT NULL DEFAULT NOW(),
    status VARCHAR(20) NOT NULL DEFAULT 'active'
);

-- Indizes
CREATE INDEX idx_devices_network_id ON devices(network_id);
CREATE INDEX idx_devices_world_type ON devices(world_type);
CREATE INDEX idx_devices_status ON devices(status);
CREATE INDEX idx_devices_last_seen ON devices(last_seen);
CREATE INDEX idx_devices_capabilities ON devices USING GIN(capabilities);
```

### Connections Table
```sql
CREATE TABLE connections (
    connection_id UUID PRIMARY KEY,
    source_device_id UUID NOT NULL REFERENCES devices(device_id),
    target_device_id UUID NOT NULL REFERENCES devices(device_id),
    established_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_heartbeat TIMESTAMP NOT NULL DEFAULT NOW(),
    status VARCHAR(20) NOT NULL DEFAULT 'active'
);

-- Indizes
CREATE INDEX idx_connections_source_device ON connections(source_device_id);
CREATE INDEX idx_connections_target_device ON connections(target_device_id);
CREATE INDEX idx_connections_status ON connections(status);
CREATE INDEX idx_connections_last_heartbeat ON connections(last_heartbeat);
CREATE UNIQUE INDEX idx_connections_unique ON connections(source_device_id, target_device_id) WHERE status = 'active';
```

### Network Table
```sql
CREATE TABLE networks (
    network_id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    owner_device_id UUID NOT NULL REFERENCES devices(device_id)
);

-- Indizes
CREATE INDEX idx_networks_owner_device ON networks(owner_device_id);
CREATE INDEX idx_networks_name ON networks(name);
```

**Indizes-Strategie:**
- **Primary Keys**: Automatisch indexiert
- **Foreign Keys**: Indizes für alle Foreign Keys für schnelle Joins
- **Frequently Queried Columns**: Indizes für häufig abgefragte Spalten (status, last_seen, etc.)
- **Composite Indizes**: Für häufige Query-Patterns (z.B. source_device_id + target_device_id)
- **GIN Indizes**: Für JSONB-Spalten (capabilities) für schnelle JSON-Queries
- **Partial Indizes**: Für spezifische Filter (z.B. nur aktive Connections)

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

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Asgard sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- Database (PostgreSQL/SQLite)
- Web Framework (Express, Fastify, etc.)
- WebSocket Library
- API Framework
- **TypeScript/Frontend-Tooling**: `bun` wird als Package-Manager und Runtime verwendet (statt npm oder pnpm)
  - Integration erfolgt über `bun install` für Dependencies und `bun run` für Scripts
  - `bun`-spezifische Konfigurationen können in `package.json` oder `bunfig.toml` definiert werden
  - Dependencies werden über `bun` verwaltet (schneller als npm/pnpm)

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

### Asgard-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Device-Registry-Einstellungen
- Network-Management-Einstellungen
- Lock-Management-Einstellungen
- Database-Konfiguration

**Chat-Management:**
- **Beliebig viele Chats**: Platform muss ermöglichen, quasi beliebig viele Chats zu starten
- **Chat-Leitung**: Chats können direkt an Götter geleitet werden (z.B. Frigg-Chat)
- **Chat-Flags**: Flags in Settings steuern, ob ein Chat direkt an einen Gott geleitet wird oder über Odin läuft

## Integration

- **Odin**: Läuft als Hauptprozess auf Asgard
- **Services**: Alle Services werden von Odin koordiniert
- **Bifrost**: Für Device-to-Device Communication mit erweiterten Features
- **Heimdall**: Für Security und Authentication
- **Midgard/Alfheim**: Clients können sich mit Asgard verbinden
- **Jotunheim**: IoT-Devices können über Asgard gesteuert werden
- **Yggdrasil**: Für globale Device-Registry und User-Management

## Performance

### Server-Performance
- **Server-Performance-Optimierungen**: Optimierungen für Server-Performance
- **Performance-Monitoring**: Monitoring von Server-Performance
- **Server-Load**: Effiziente Behandlung von hohem Server-Load (Load Balancing, Scaling)

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
  - **Server-Datenverarbeitung**: Server-optimierte lokale Datenverarbeitung
  - **Server-spezifische Datenschutz-Features**: Server-spezifische Datenschutz-Features
  - **Datenschutz-Präferenzen**: UI für Datenschutz-Präferenzen
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

### Server-Security
- **Server-Security-Implementierung**: Umfassende Server-Security-Implementierung
- **Firewall-Integration**: Integration mit Firewall-Systemen
- **Security-Threats**: Robustes Error-Handling für Security-Threats (Intrusion Detection, Prevention)

### API-Security
- **API-Security-Implementierung**: Sichere API-Implementierung
- **Rate-Limiting**: Rate-Limiting für API-Endpoints zum Schutz vor DDoS
- **API-Angriffe**: Robustes Error-Handling für API-Angriffe (Rate Limiting, Blocking)

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

**Data Structure**
- Device ID (user-assigned, unique)
- Device Name
- World Type (Midgard, Asgard, Alfheim, Jotunheim)
- Capabilities
- Hardware Specs
- Registration Timestamp

**Storage**
- Local SQLite Database (oder PostgreSQL für Server)
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

**NAT-Traversal-Implementierung**
- **STUN, TURN, ICE**: Implementierung von STUN, TURN und ICE für NAT-Traversal
- **Automatische NAT-Discovery**: Automatische NAT-Discovery für optimale Konfiguration
- **NAT-Traversal-Fehler**: Robustes Error-Handling bei NAT-Traversal-Fehlern (Fallback, Retry)

**Automatisches NAT-Traversal**
- **Automatisch bevorzugt**: Automatisches NAT-Traversal wird stark bevorzugt
- **STUN**: STUN-Protokoll für NAT-Discovery
- **TURN**: TURN-Server für Relay wenn NAT-Traversal nicht möglich (Asgard als TURN-Server)
- **ICE**: ICE-Protokoll für optimalen Pfad
- **Asgard braucht NAT-Traversal**: Asgard sollte auch NAT-Traversal-Funktionalität haben
- **Fallback auf manuelle Konfiguration**: Falls automatisch nicht möglich, Fallback auf manuelle Port-Forwarding-Konfiguration

**Port-Forwarding**
- **Automatische Port-Forwarding-Konfiguration**: Automatische Konfiguration von Port-Forwarding (UPnP, NAT-PMP)
- **Manuelle Konfiguration als Fallback**: Manuelle Konfiguration als Fallback wenn automatisch nicht möglich
- **Router-Kompatibilität**: Behandlung von Router-Kompatibilitätsproblemen

### Dynamic IP Handling

**Kombination: DDNS wenn konfiguriert, sonst Relay über Yggdrasil**
- **DDNS**: Dynamic DNS für Domain-Names (wenn User konfiguriert)
- **IP Update Service**: Service für IP-Updates
- **Connection Refresh**: Automatische Connection-Refresh bei IP-Änderung
- **Yggdrasil-Relay**: Falls DDNS nicht konfiguriert, automatisch über Yggdrasil-Relay
- **Sicherheit**: Muss sicher sein und nicht zu kompliziert

### Configuration

**IP Configuration**
- **Static IP**: Feste IP-Adresse konfigurieren
- **Dynamic IP**: Dynamische IP mit Update-Mechanismus
- **Domain Name**: Domain-Name für Device
- **Port Configuration**: Port-Konfiguration für Services

**Network Configuration**
- **Network ID**: Network ID für Software-Netzwerk
- **Server Configuration**: Konfiguration von Asgard/Yggdrasil-Servern
- **Relay Configuration**: Konfiguration von Relay-Servern
- **Firewall Rules**: Firewall-Regeln für Verbindungen

### WAN Connection Establishment

1. **Device A möchte sich mit Device B verbinden (WAN)**
   - Device A hat IP/Domain von Device B
   - Device A initiiert WAN-Connection

2. **Connection Attempt**
   - Device A versucht direkte Verbindung
   - Falls nicht möglich: Relay über Server

3. **Connection Establishment**
   - TLS Handshake
   - Device Authentication
   - Connection wird etabliert

4. **Ongoing Communication**
   - Messages werden über WAN-Connection geroutet
   - Connection wird überwacht
   - Bei Ausfall: Automatische Wiederverbindung

### Enhanced Routing Workflow

1. **Message muss geroutet werden**
   - Source und Target Device werden identifiziert
   - Routing-Strategie wird gewählt

2. **Path Selection**
   - Verfügbare Pfade werden evaluiert
   - Optimaler Pfad wird gewählt
   - Fallback-Pfade werden vorbereitet

3. **Message Routing**
   - Message wird über gewählten Pfad geroutet
   - Bei Fehler: Fallback-Pfad wird verwendet
   - Routing-Status wird überwacht

## Implementierungs-Notizen

- Sollte als Service/Daemon laufen
- Muss Persistent Storage haben
- Sollte Web-Dashboard haben
- Muss API für Administration haben
- Sollte Monitoring und Logging haben
- Muss Backup & Restore unterstützen
- Sollte skalierbar sein
- **Muss robustes Error-Handling für Netzwerk-Fehler haben**: Umfassendes Error-Handling für alle Netzwerk-Szenarien
- **Sollte verschiedene NAT-Traversal-Strategien unterstützen**: STUN, TURN, ICE, manuelle Konfiguration
- **Muss Connection-Quality-Monitoring haben**: Überwachung der Connection-Quality für optimale Routing-Entscheidungen
- **Sollte automatisches Failover haben**: Automatisches Failover bei Verbindungsausfall
- **Muss Security-Best-Practices für WAN-Connections folgen**: TLS, Certificate Validation, Firewall Integration, Intrusion Detection
- **Sollte User-Feedback für Connection-Status haben**: User sollte Connection-Status und Routing-Informationen sehen können
- **Performance**: Muss optimiert sein für Server-Hardware und hohen Durchsatz
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Server-Umgebungen

