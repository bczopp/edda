# Jotunheim - IoT Platform

## Übersicht

Jotunheim ist eine **Platform** für IoT-Devices (inkl. ESP32/Microcontroller), ähnlich wie Midgard (Desktop), Alfheim (Mobile), Asgard (Homeserver) und Ragnarok (Terminal). Als Platform ist Jotunheim komplett platformspezifisch optimiert und kümmert sich um Connections (Netzwerk, etc.), konvertiert diese zu Anfragen an Services (Odin/Loki) und ruft Services via gRPC auf.

**Services sind unabhängig von Platformen**: Alle Services (Odin, Loki, Thor, Freki, Geri, etc.) sind in Rust implementiert und unabhängig von Platformen. Platformen kommunizieren mit Services via gRPC.

**Tests ausführen:** Von `jotunheim/`: `docker compose -f docker-compose.test.yml run --rm jotunheim-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `jotunheim/**` läuft die Pipeline [.github/workflows/jotunheim.yml](../.github/workflows/jotunheim.yml) (Test im Container, Lint).

## Zielplattformen

- ESP32 (esp-rs, Rust)
- ESP8266 (Rust)
- Raspberry Pi Pico (Rust)
- Andere Microcontroller mit WiFi/Network (Rust)

### IoT-Platform-Implementierung
- **ESP32**: ESP32-spezifische Optimierungen (esp-rs, WiFi, Bluetooth)
- **ESP8266**: ESP8266-spezifische Optimierungen (WiFi, minimaler Footprint)
- **Raspberry Pi Pico**: Raspberry Pi Pico-spezifische Optimierungen (WiFi, mehr Features)
- **Device-spezifische Optimierungen**: Platform-spezifische Optimierungen je nach Device-Typ
- **Device-Updates**: Automatische Behandlung von Device-Updates (OTA, Firmware)

## Projektstruktur

```
jotunheim/
├── src/
│   ├── esp32/           # ESP32 Implementation (Rust)
│   │   ├── main.rs
│   │   ├── services/
│   │   └── utils/
│   ├── generic/         # Generic Implementation (Rust)
│   │   ├── protocol/
│   │   ├── services/
│   │   └── utils/
│   └── shared/         # Shared Code (Rust)
│       ├── protocol/
│       └── utils/
├── Cargo.toml
├── config/
└── examples/
```

## Features

### Core Features

- **Remote Control**: Device kann von anderen Devices gesteuert werden
- **Toolcalling Protocol**: Token-effizientes Toolcalling-Protokoll
- **Capability Negotiation**: Device teilt Capabilities mit
- **Streaming Support**: Streaming für große Datenmengen
- **Drahtlose Stream-Übertragung**: Drahtlose Übertragung von Streams (von Haus aus bei Jotunheim)
- **Bidirektionale Streams**: Video und Audiostreams bidirektional senden können
- **Plugin-Vorbereitung**: Vorbereitung für extra Plugins für erweiterte Streaming-Features

### IoT-Specific Features

- **Extrem Lightweight**: Extrem lightweight - User sollen noch custom scripts auf solche Devices kopieren können
- **Minimal Footprint**: Geringer Speicher- und CPU-Verbrauch
- **Leichtgewichtige Sprachen**: Nur wirklich leichtgewichtige, performante Sprachen unterstützen
- **Low Power**: Energie-effiziente Operation
- **Network Resilience**: Robustes Error-Handling für Netzwerk-Fehler
- **OTA Updates**: Over-the-Air Updates (optional)
- **Streaming-Vorbereitung**: Vorbereitung für Video/Audio-Streaming-Plugins

## Platform-Architektur

### Platform-Rolle

**Jotunheim als Platform:**
- **Connections**: Jotunheim-Platform kümmert sich um Connections (Netzwerk, etc.)
- **Konvertierung**: Konvertiert Connections zu Anfragen an Services (Odin/Loki)
- **Platformspezifisch**: Komplett platformspezifische Implementierung (ESP32, ESP8266, etc.)
- **Service-Aufrufe**: Ruft Services (Odin, Loki) via gRPC auf

**Service-Unabhängigkeit:**
- **Services in Rust**: Alle Services (Odin, Loki, Thor, Freki, Geri, etc.) sind unabhängig von Platformen
- **gRPC-Kommunikation**: Jotunheim-Platform kommuniziert mit Services via gRPC
- **Wiederverwendbar**: Services können von verschiedenen Platformen genutzt werden

### Service Integration

**Hinweis**: Jotunheim-Devices haben KEINEN Odin-Prozess, da sie zu klein sind. Stattdessen nutzen sie Loki für Toolcalling-Funktionalität und kommunizieren über Loki mit anderen Devices.

**Jotunheim-Platform ruft Services auf:**
- **Odin**: Für Orchestration (wenn verfügbar)
- **Loki**: Für Script-Execution (siehe unten)

### Remote Control
- Empfängt Commands von anderen Devices
- Führt Commands aus
- Sendet Results zurück

### Capability Exposure (Platform Capability Protocol)
- **Einheitliches Protocol**: Jotunheim nutzt das gleiche Protocol wie andere Platformen (Midgard, Alfheim, Asgard, Ragnarok) für Capability-Exposure
- **Einherjar Protocol**: Platform ruft `EinherjarProtocol.GetCapabilities()` für alle Services auf der Platform auf
- **Capability-Aggregation**: Platform aggregiert Capabilities von allen Services (inkl. Loki) und propagiert sie an Odin
- **Device teilt verfügbare Tools/Functions mit**: Via Platform Capability Protocol
- **Capability-Negotiation mit Controller**: Über Platform Capability Protocol
- **Dynamic Capability Updates**: Werden über Platform Capability Protocol propagiert

## Toolcalling via Loki Service Protocol

**Wichtig**: Jotunheim nutzt **Loki Service Protocol** via gRPC für Toolcalling-Funktionalität. Es gibt kein separates "Loki Toolcalling Protocol" - Jotunheim nutzt das gleiche Loki Service Protocol wie andere Platformen.

### Protocol Features (Loki Service Protocol)
- **gRPC-based**: Type-safe, effizient, HTTP/2
- **Protobuf**: Binary, kompakt, automatische Code-Generierung
- **Token-efficient**: Deutlich weniger Overhead als MCP
- **Streaming**: Built-in Streaming für große Responses
- **Error Recovery**: Robustes Error-Handling mit Status-Codes
- **Type-Safe**: Protobuf garantiert korrekte Tool-Definitionen
- **Bereitgestellt von Loki**: Loki Service stellt das Protocol bereit
- **Dynamische Script-Funktionen**: Jedes User-Script wird zu einer gRPC-Funktion (`Script_<script_name>`)
- **Tool-Konfiguration**: Loki lädt Konfigurationsdatei mit User-Tools (KEIN Protocol)

### Integration mit Loki

Jotunheim-Devices nutzen Loki für Toolcalling:
- **Loki Service**: Jotunheim-Platform ruft Loki via gRPC auf
- **Toolcalling-Funktionen**: Werden über Loki bereitgestellt
- **Capability Exposure**: Device-Capabilities werden über Loki kommuniziert

### Vorteile von gRPC
- **Type-Safety**: Protobuf mit automatischer Code-Generierung
- **Effizienter**: HTTP/2, Binary-Format, weniger Overhead als MessagePack
- **Bessere Performance**: Schnellere Serialisierung, HTTP/2 Multiplexing
- **Streaming**: Built-in Streaming für große Responses
- **Error-Handling**: Besseres Error-Handling mit Status-Codes
- **Minimaler Footprint**: gRPC ist effizienter als MessagePack + Custom-Protocol

### Tool Definition

**WICHTIG**: Loki lädt eine Konfigurationsdatei mit den vom User zur Verfügung gestellten Tools (KEIN Protocol)

**Konfigurationsdatei-Format:**
- **Format**: JSON oder YAML (einfach zu parsen)
- **Speicherort**: Lokal auf Device oder auf Asgard (zentral verwaltet)
- **Tool-Definition**: Tools werden in Konfigurationsdatei definiert
- **Tool-Validierung**: Validierung von Tool-Definitionen bei Laden
- **Tool-Updates**: Dynamische Tool-Updates (Konfigurationsdatei wird neu geladen)

**Tool-Struktur:**
```typescript
interface Tool {
  name: string;
  description: string;
  parameters: {
    [key: string]: {
      type: string;
      required: boolean;
      description?: string;
    };
  };
  returns: {
    type: string;
    description?: string;
  };
}
```

**gRPC-Client-Implementierung:**
- **Lightweight gRPC-Client**: Leichtgewichtiger gRPC-Client für IoT-Devices
- **Protobuf-Lite-Support**: Protobuf-Lite für minimale Serialisierung
- **gRPC-Fehler**: Robustes Error-Handling bei gRPC-Fehlern (Retry, Fallback)

## Capability Negotiation

### Warum Capability Negotiation?

ESP32-Devices sind sehr variabel einsetzbar und können unterschiedlich konfiguriert sein:
- **Verschiedene Sensoren**: Temperatur, Feuchtigkeit, Bewegung, Licht, etc.
- **Verschiedene Aktoren**: LEDs, Motoren, Relais, Displays, etc.
- **Verschiedene Interfaces**: GPIO, I2C, SPI, UART, ADC, PWM, etc.
- **Verschiedene Funktionen**: Jedes Device kann andere Tools/Funktionen anbieten

**Das verbundene Device (Controller, z.B. Midgard/Asgard) muss wissen, was das Jotunheim-Device kann, um es steuern zu können.**

### Capability Structure

**Capability-Struktur für verschiedene IoT-Devices:**
- **Device-spezifische Capability-Formate**: Verschiedene Formate je nach Device-Typ
- **Capability-Kompatibilität**: Behandlung von Capability-Kompatibilitätsproblemen

```typescript
interface JotunheimCapabilities {
  // Device Information
  deviceId: string;
  deviceName: string;
  deviceType: string;          // z.B. "ESP32", "ESP8266", "Raspberry Pi Pico"
  firmwareVersion: string;
  protocolVersion: string;
  
  // Available Tools/Functions
  tools: Tool[];               // Liste aller verfügbaren Tools
  
  // Hardware Capabilities
  hardware: {
    gpio: {
      available: number[];     // Verfügbare GPIO-Pins
      digital: boolean;        // Digital I/O unterstützt
      analog: boolean;         // Analog I/O unterstützt
      pwm: boolean;            // PWM unterstützt
    };
    interfaces: string[];      // z.B. ["I2C", "SPI", "UART", "ADC", "PWM"]
    sensors: string[];         // z.B. ["DHT22", "DS18B20", "BMP280"]
    actuators: string[];       // z.B. ["LED", "Motor", "Relay", "Display"]
  };
  
  // Resource Limits
  resources: {
    maxMemory: number;         // Maximaler verfügbarer RAM
    maxCpu: number;            // CPU-Limit (optional)
    maxConcurrentTools: number; // Max. parallele Tool-Ausführungen
  };
  
  // Protocol Features
  features: {
    streaming: boolean;        // Streaming unterstützt
    compression: boolean;       // Compression unterstützt
    encryption: boolean;        // Encryption unterstützt (optional)
  };
}
```

### Negotiation Flow

1. **Connection Establishment**: Controller stellt TCP/UDP-Verbindung zum Jotunheim-Device her
2. **Capability Request**: Controller sendet `CAPABILITY_REQUEST` Message
3. **Capability Processing**: Controller analysiert Capabilities und erstellt Tool-Registry
4. **Tool Discovery & Registration**: Controller registriert alle verfügbaren Tools
5. **Dynamic Capability Updates**: Capabilities werden bei Kopplung/Verbindung propagiert

**Negotiation-Details:**
- **Negotiation-Timeouts**: Timeout-Mechanismen für Capability-Negotiation
- **Negotiation-Fehler**: Robustes Error-Handling bei Negotiation-Fehlern (Retry, Fallback)

## Implementation Constraints

### Memory Constraints
- **Minimal RAM**: Muss mit wenig RAM auskommen
- **Stack Management**: Effizientes Stack-Management
- **Memory Pooling**: Memory-Pooling für bessere Performance
- **Memory-Constraints**: Behandlung von Memory-Constraints (Eviction, Throttling)

### CPU Constraints
- **Low CPU Usage**: Minimaler CPU-Verbrauch
- **Async Processing**: Asynchrone Verarbeitung
- **Task Scheduling**: Effizientes Task-Scheduling
- **Task-Scheduling-Optimierungen**: Optimierungen für Task-Scheduling
- **CPU-Constraints**: Behandlung von CPU-Constraints (Throttling, Priorisierung)

### Network Constraints
- **Low Bandwidth**: Optimiert für niedrige Bandbreite
- **Connection Resilience**: Robustes Error-Handling
- **Reconnection**: Automatische Wiederverbindung
- **Network-Compression**: Network-Compression für minimale Datenübertragung
- **Network-Constraints**: Behandlung von Network-Constraints (Throttling, Priorisierung)

## Example: ESP32 Implementation

### Hardware Requirements
- ESP32 mit WiFi
- Optional: Bluetooth
- Optional: Sensors/Actuators

### Software Stack
- Rust (esp-rs für ESP32)
- WiFi Stack
- TCP/IP Stack
- gRPC Client (lightweight, Protobuf-Lite)
- Bifrost Client (WebSocket-Support, optional)

### Features
- WiFi Connection Management
- TCP/UDP Communication
- Toolcalling Protocol
- Remote Control

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Jotunheim sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)
- **WICHTIG**: Jotunheim hat stark begrenzte Resources (z.B. ESP32) und dort soll noch anderer Code ausgeführt werden
- **Minimale Abhängigkeiten**: Nur absolut notwendige DTOs und Protokolle verwenden, um Speicher zu sparen
- **Code-Größe**: Package-Größe muss minimal gehalten werden, da auf den Devices noch anderer Code ausgeführt werden soll
- **gRPC-Client**: Lightweight gRPC-Client mit Protobuf-Lite für ESP32

### Technische Abhängigkeiten

- Network Stack (WiFi, TCP/IP)
- gRPC Client Library (lightweight, Protobuf-Lite für ESP32)
- Bifrost Client Library (WebSocket-Support, optional)
- Rust Toolchain (esp-rs für ESP32)

## Loki Service

**Loki** ist ein unabhängiger Service (nicht Teil von Jotunheim-Platform), der user-generierte Scripte per gRPC zugänglich macht. Jedes Script wird zu einer aufrufbaren gRPC-Funktion, die das Script direkt auf dem Device ausführt.

### Loki-Funktionen

- **Script-Execution**: User-generierte Scripte per gRPC zugänglich machen
- **Dynamische gRPC-Funktionen**: Jedes Script wird zu einer aufrufbaren gRPC-Funktion (`Script_<script_name>`)
- **Direkte Ausführung**: Funktion führt Script direkt auf Device aus (nichts anderes)
- **Koordination der 3 Kinder**: Fenrir, Jörmungandr, Hel (siehe unten)

### Script-Execution-System

- **Nur leichtgewichtige, performante Sprachen**: Nur wirklich leichtgewichtige, performante Sprachen unterstützen
  - Lua (primär für ESP32 und kleinere Devices)
  - Ggf. leichtere Rust-Version wenn signifikante Verbesserungen
  - Device-abhängig: Script-Sprache abhängig vom Device (ESP32 = Lua, größere Devices = mehr Optionen)
- **Direkte Ausführung**: Keine Sandbox für Performance
- **Resource-Management**: Scripts müssen Resource-Limits beachten
- **gRPC-Funktion pro Script**: Jedes Script wird zu einer aufrufbaren gRPC-Funktion
- **Custom Scripts**: User sollen noch custom scripts auf solche Devices kopieren können

### Die 3 Kinder von Loki

#### Fenrir - Aggressive Tasks
- **Rolle**: Intensive Tasks, Hardware-Control
- **Funktionen**: Hardware-nahe Operationen, GPIO-Control, Sensor-Reading (intensiv), Actuator-Control, Low-level Hardware-Access

#### Jörmungandr - Network/Communication
- **Rolle**: Netzwerk-Tasks, Kommunikation
- **Funktionen**: Netzwerk-Operationen, HTTP/HTTPS Requests, WebSocket-Verbindungen, MQTT-Communication, Network-Protocol-Handling

#### Hel - Data/Storage
- **Rolle**: Datenverwaltung, Storage
- **Funktionen**: Daten-Speicherung, File-System-Operationen, Daten-Aggregation, Cache-Management, Data-Processing

### gRPC-Service-Definition (LokiService)

```protobuf
service LokiService {
  // Statische Methoden
  rpc GetCapabilities(CapabilityRequest) returns (CapabilityResponse);
  rpc GetChildrenStatus(StatusRequest) returns (StatusResponse);
  rpc ListScripts(ListScriptsRequest) returns (ListScriptsResponse);
  rpc RegisterScript(RegisterScriptRequest) returns (RegisterScriptResponse);
  
  // Dynamische Script-Funktionen (zur Laufzeit generiert)
  // Jedes User-Script wird zu einer gRPC-Funktion:
  // rpc Script_<script_name>(ScriptInput) returns (ScriptOutput);
  // rpc StreamScript_<script_name>(stream ScriptChunk) returns (stream ScriptResult);
}
```

### Bifrost-Verbindung (Optional)

- **Optional**: Bifrost-Verbindung nur bedingt nötig
- **gRPC-Streams**: Wenn gRPC-Streams möglich sind und verschlüsselt (TLS) oder in abgesichertem Netzwerk, brauchen wir keine Bifrost-Verbindung
- **Verschlüsselung**: Streams müssen verschlüsselt übertragen werden (TLS) oder in abgesichertem Netzwerk
- **Fallback**: Bifrost als Fallback wenn gRPC-Streams nicht möglich

### Flexibilität für verschiedene Devices

- **ESP32**: Minimaler Footprint, Lua als primäre Script-Sprache
- **ESP8266**: Ähnlich ESP32
- **Raspberry Pi Pico**: Mehr Optionen möglich
- **Größere Devices**: Mehr Script-Sprachen, mehr Features
- **Leichtere Rust-Version**: Wenn es eine noch leichtere Version von Rust gibt, soll auch Jotunheim und Loki darin geschrieben werden, wenn signifikante Verbesserungen dadurch erhält

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

### Jotunheim-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Capability-Konfiguration
- Network-Resilience-Einstellungen
- Resource-Limits
- OTA-Update-Einstellungen

## Integration

- **Jotunheim = Platform**: Wie Midgard, Alfheim, Asgard, Ragnarok
- **Platform-Funktionen**: Connections, Konvertierung zu Anfragen an Loki/Odin
- **Loki-Service**: Wird von Jotunheim-Platform via gRPC aufgerufen
  - **Loki-Integration**: Kommunikation zwischen Jotunheim-Platform und Loki-Service
  - **Device-spezifische Kommunikations-Optimierungen**: Optimierungen für verschiedene Device-Typen
  - **Loki-Ausfälle**: Robustes Error-Handling bei Loki-Ausfällen (Retry, Fallback)
- **Kein Odin**: Jotunheim-Devices haben keinen Odin-Prozess
- **Toolcalling Protocol**: gRPC-basierte Kommunikation (Type-safe, effizient)
- **gRPC-Server**: Controller (Midgard/Asgard) fungieren als gRPC-Server
- **gRPC-Client**: Jotunheim-Devices fungieren als gRPC-Clients
- **Bifrost**: Kann Bifrost-Verbindungen eingehen für erweiterte Kommunikation (optional, nur wenn gRPC-Streams nicht verschlüsselt)
- **Midgard/Asgard**: Werden als Controller verwendet (Odin erkennt Jotunheim-Devices und steuert sie über gRPC)
  - **Controller-Integration**: Kommunikation zwischen Jotunheim-Devices und Controllern
  - **Controller-spezifische Optimierungen**: Optimierungen für verschiedene Controller-Typen
  - **Controller-Ausfälle**: Robustes Error-Handling bei Controller-Ausfällen (Retry, Fallback)
- **Heimdall**: Für Security (optional, wenn Encryption unterstützt)
- **Programmiersprache**: Rust (esp-rs für ESP32, etc.)

## Einherjar Protocol Integration

Loki auf Jotunheim-Devices gibt alle verfügbaren Funktionen via Einherjar Protocol bekannt:

**Statische Funktionen:**
- `RegisterScript` - Neues Script registrieren
- `ListScripts` - Registrierte Scripts auflisten
- `GetCapabilities` - Device-Capabilities abfragen
- `GetChildrenStatus` - Status von Fenrir, Jörmungandr, Hel abfragen

**Dynamische User-Script-Funktionen:**
- `Script_<script_name>()` - Für jedes registrierte User-Script
- Automatisch erstellt nach `RegisterScript()`
- Via Einherjar Protocol für alle Services sichtbar

**Beispiel:**
1. User registriert Script "temperature_monitor" via `RegisterScript()`
2. Loki erstellt dynamische Funktion: `Script_temperature_monitor()`
3. Funktion wird via Einherjar Protocol bekannt gegeben
4. Jeder Service (Thor, Odin, etc.) kann Script aufrufen: `Script_temperature_monitor(params)`

**Integration-Flow:**
```
Service (Thor/Odin) → Fragt Device-Capabilities via Einherjar Protocol
    ↓
Loki antwortet: ["RegisterScript", "ListScripts", "Script_temperature_monitor", ...]
    ↓
Service ruft auf: RegisterScript(name="led_control", content="...", language="lua")
    ↓
Loki registriert Script → Erstellt Script_led_control() → Via Einherjar bekannt
    ↓
Service ruft auf: Script_led_control(params) → Loki führt aus → Result
```

**Vorteile:**
- **Generisch**: Jeder Service kann Loki-Funktionen nutzen (nicht nur Thor)
- **Type-Safe**: gRPC + Protobuf garantiert korrekte Typen
- **Dynamisch**: Scripts können zur Laufzeit hinzugefügt werden
- **Discovery**: Services entdecken Funktionen automatisch via Einherjar
- **Resource-Effizient**: Loki ist extrem lightweight für IoT-Devices

## Resource-Management

**Extrem Lightweight (Essentiell für Jotunheim):**
- **Extrem lightweight**: Jotunheim muss extrem lightweight sein
- **Custom Scripts**: User sollen noch custom scripts auf solche Devices kopieren können
- **Leichtgewichtige Sprachen**: Nur wirklich leichtgewichtige, performante Sprachen unterstützen
- **Minimaler Footprint**: Optimiert für geringen RAM- und Flash-Verbrauch
- **Resource-Reserve**: Genug Ressourcen für User-Scripts reservieren

**Connection-Resilience:**
- **Connection-Resilience-Implementierung**: Robustes Connection-Management für IoT-Devices
- **Automatische Reconnection**: Automatische Wiederverbindung bei Verbindungsausfall
- **Persistente Connection-Probleme**: Behandlung von persistenten Connection-Problemen (Exponential Backoff, Fallback)

**Error-Recovery:**
- **Error-Recovery-Mechanismen**: Umfassende Error-Recovery-Mechanismen für IoT-Devices
- **Retry-Mechanismen**: Intelligente Retry-Mechanismen mit Exponential Backoff
- **Error-Persistence**: Behandlung von Error-Persistence (Logging, Reporting)

**Resource-Limits:**
- CPU, Memory, Disk-Limits für Script-Execution
- Konfigurierbar pro Script-Typ
- **Strikte Limits**: Strikte Resource-Limits, damit User noch Scripts kopieren können

**Resource-Monitoring:**
- Überwachung von System-Ressourcen (CPU, RAM, Disk, Network)
- Resource-Usage wird überwacht während Script-Execution
- **Resource-Tracking**: Kontinuierliches Tracking, um Platz für User-Scripts zu gewährleisten

**Bei Resource-Exhaustion:**
- Verhindert Resource-Exhaustion durch Limits
- Scripts werden in Queue gelegt, wenn Ressourcen knapp
- **Priorität**: System-Scripts haben Priorität, aber User-Scripts müssen auch ausführbar bleiben

**Resource-Effizienz:**
- Minimale RAM-Nutzung, nur so viel wie nötig
- Optimiert für geringen Footprint auf IoT-Devices
- **Code-Effizienz**: Code sollte so kurz wie möglich sein, während Lesbarkeit erhalten bleibt

## Streaming-Features

**Drahtlose Stream-Übertragung:**
- **Von Haus aus**: Drahtlose Übertragung von Streams sollte möglich sein (von Haus aus bei Jotunheim)
- **Bidirektional**: Video und Audiostreams bidirektional senden können
- **Plugin-Vorbereitung**: Vermutlich mit extra Plugins, aber das muss vorbereitet werden
- **Streaming-Protokoll**: Vorbereitung für Video/Audio-Streaming-Protokolle

**Streaming-Architektur:**
- **Basis-Support**: Basis-Streaming-Support von Haus aus
- **Plugin-Erweiterung**: Erweiterte Streaming-Features via Plugins
- **Bidirektional**: Senden und Empfangen von Video/Audio-Streams
- **Effiziente Übertragung**: Optimiert für drahtlose Übertragung

## Performance

### Performance-Optimierungen
- **Minimaler Footprint**: Optimiert für geringen RAM- und Flash-Verbrauch
- **Effiziente Serialisierung**: Protobuf (Protobuf-Lite für ESP32) für minimale Datenübertragung
- **Low CPU Usage**: Minimale CPU-Nutzung für Battery-Life
- **Optimierte Network-Protocols**: gRPC mit HTTP/2 für minimale Overhead
- **Streaming Support**: Built-in gRPC-Streaming für große Datenmengen
- **Connection Pooling**: Wiederverwendung von gRPC-Verbindungen

### Performance-Metriken
- Minimaler RAM-Verbrauch (< 10KB für Basis-Implementation)
- Schnelle Tool-Execution (< 100ms für einfache Tools)
- Effiziente Netzwerk-Kommunikation (minimaler Overhead)

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden übertragen
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, wo möglich
  - **IoT-Datenverarbeitung**: IoT-optimierte lokale Datenverarbeitung
  - **Device-spezifische Datenschutz-Features**: Device-spezifische Datenschutz-Features
  - **Datenschutz-Constraints**: Behandlung von Datenschutz-Constraints (Hardware-Limits)
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **Opt-in Encryption**: Optional Verschlüsselung für sensible Daten

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden übertragen
- **User Control**: User hat Kontrolle über Datenübertragung

## Sicherheit

### Security-Features
- **IoT-Security-Implementierung**: Umfassende Security-Implementierung für IoT-Devices
- **Device-spezifische Security-Features**: Platform-spezifische Security-Features je nach Device-Typ
- **Security-Constraints**: Behandlung von Security-Constraints (Hardware-Limits)
- **Optional Encryption**: Verschlüsselung für sensible Daten (wenn unterstützt)
- **TLS Encryption**: TLS-Verschlüsselung für Netzwerk-Verbindungen (wenn unterstützt)
- **Authentication**: Device-Authentifizierung über Heimdall (optional)
- **Input Validation**: Validierung aller eingehenden Commands
- **Secure Key Storage**: Sichere Speicherung von Keys (wenn unterstützt)

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: OTA-Updates für Security-Patches
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Minimal Attack Surface**: Minimale Angriffsfläche durch Lightweight-Design

## Capability Synchronisation

### Wichtig: Capabilities sind hauptsächlich für Jotunheim-Devices

**Jotunheim-Devices benötigen Capability-Negotiation**
- **Variable Konfiguration**: Jotunheim-Devices (ESP32, etc.) haben variable Konfigurationen
- **Tools müssen bekannt sein**: Controller muss wissen, welche Tools/Funktionen verfügbar sind
- **Bei Kopplung**: Capabilities werden bei jeder Kopplung/Verbindung propagiert
- **Auf Anfrage**: Controller kann jederzeit Capabilities anfragen

**Andere Devices**
- **Midgard/Alfheim/Asgard**: Haben standardisierte Capabilities, weniger dynamisch
- **Capability-Updates**: Selten, hauptsächlich bei Updates

### Capability-Propagation

**Bei Kopplung**
- **Automatisch**: Bei jeder Kopplung/Verbindung werden Capabilities automatisch propagiert
- **Jotunheim-Devices**: Senden Capabilities bei Verbindungsaufbau
- **Controller**: Empfängt und registriert Capabilities

**Auf Anfrage**
- **Controller kann anfragen**: Controller kann jederzeit `CAPABILITY_REQUEST` senden
- **Jotunheim-Device antwortet**: Device sendet aktuelle Capabilities zurück
- **Use Case**: Wenn Controller unsicher ist, welche Capabilities verfügbar sind

**Automatische Updates**
- **Wichtige Änderungen**: Bei wichtigen Änderungen (z.B. nach OTA-Update) sendet Device automatisch neue Capabilities
- **Bei nächster Verbindung**: Sonst werden Capabilities bei nächster Verbindung aktualisiert
- **Event-basiert**: Capability-Updates werden als Events propagiert

### Synchronisation-Architektur

**Peer-to-Peer (Standard)**
- **Direkte Synchronisation**: Devices synchronisieren Capabilities direkt miteinander
- **Kein zentraler Server nötig**: Funktioniert auch ohne Asgard/Yggdrasil
- **Einfach und schnell**: Direkte Kommunikation zwischen Devices

**Asgard als Hub (wenn vorhanden)**
- **Asgard übernimmt**: Wenn Asgard im Netzwerk vorhanden ist, übernimmt Asgard die Capability-Synchronisation
- **Zentrale Registry**: Asgard fungiert als zentrale Registry für Capabilities
- **Alle Devices melden an Asgard**: Devices melden ihre Capabilities an Asgard
- **Asgard verteilt**: Asgard verteilt Capabilities an alle verbundenen Devices

**Mehrere Asgard-Server**
- **Leitender Server**: Bei mehreren Asgard-Servern im gleichen Netz ist der älteste der leitende Server
- **Alter bestimmt**: Server mit frühestem `created_at` wird zum leitenden Server
- **Leitender Server übernimmt**: Leitender Server übernimmt die Capability-Synchronisation
- **Andere Server**: Andere Server können als Backup fungieren

### Capability-Änderungen

**Automatische Re-Negotiation**
- **Wichtige Änderungen**: Bei wichtigen Änderungen (z.B. nach OTA-Update) sendet Device automatisch neue Capabilities
- **Event-basiert**: Device sendet Capability-Update-Event
- **Controller reagiert**: Controller empfängt Event und aktualisiert Tool-Registry

**Bei nächster Verbindung**
- **Sonst**: Bei weniger wichtigen Änderungen werden Capabilities bei nächster Verbindung aktualisiert
- **Standard-Flow**: Normale Capability-Request bei Verbindungsaufbau
- **Efficient**: Vermeidet unnötige Updates

### Capability-Cache-Invalidierung

**Event-basiert**
- **Bei Capability-Update**: Wenn Capability-Update-Event empfangen wird, wird Cache invalidiert
- **Sofortige Aktualisierung**: Cache wird sofort aktualisiert
- **Keine Verzögerung**: Keine Timeout-basierte Verzögerung

**Timeout als Fallback**
- **Falls Event fehlt**: Falls Event nicht empfangen wird, Timeout als Fallback
- **Sicherheit**: Verhindert veraltete Cache-Daten
- **Längere Timeout**: Timeout sollte länger sein (z.B. 1 Stunde)

## OTA Updates und Firmware Updates

### Kombination: Asgard lokal, Yggdrasil global

**Lokale Updates über Asgard**
- **Asgard als Update-Server**: Asgard fungiert als lokaler Update-Server für Jotunheim-Devices
- **Lokale Verteilung**: Updates werden lokal über Asgard verteilt
- **Schnell**: Keine Internet-Verbindung nötig
- **Kontrolle**: User hat volle Kontrolle über Updates

**Globale Updates über Yggdrasil**
- **Yggdrasil als Update-Server**: Yggdrasil fungiert als globaler Update-Server
- **Zentrale Verteilung**: Updates werden zentral von Yggdrasil verteilt
- **Automatisch**: Updates können automatisch verteilt werden
- **Für alle User**: Alle User erhalten Updates von Yggdrasil

### Workflow: Jotunheim-Device Update

1. **Update verfügbar**
   - Update wird auf Asgard oder Yggdrasil verfügbar
   - Update wird für Jotunheim-Device bereitgestellt

2. **Update-Verteilung**
   - **Lokal**: Asgard verteilt Update an Jotunheim-Devices im lokalen Netzwerk
   - **Global**: Yggdrasil verteilt Update an alle Jotunheim-Devices weltweit

3. **Update-Installation**
   - Jotunheim-Device empfängt Update
   - Update wird installiert
   - Device startet neu

4. **Capability-Update**
   - Nach Update sendet Device neue Capabilities
   - Controller aktualisiert Tool-Registry

### Update-Verifikation

**Kombination: Signaturen + Checksum**

**Digitale Signaturen**
- **Signierte Updates**: Alle Updates müssen digital signiert sein
- **Verifikation**: Device verifiziert Signatur vor Installation
- **Sicherheit**: Verhindert manipulierte Updates
- **Vertrauenswürdige Quellen**: Nur signierte Updates von vertrauenswürdigen Quellen

**Checksum-Verifikation**
- **MD5/SHA256**: Updates haben Checksum (MD5 oder SHA256)
- **Integritätsprüfung**: Device prüft Checksum nach Download
- **Korruption-Erkennung**: Erkennt korrupte Downloads
- **Zusätzliche Sicherheit**: Zusätzliche Sicherheitsebene

### Rollback bei fehlgeschlagenen Updates

**Kombination: Automatisch wenn möglich, sonst manuell**

**Automatischer Rollback**
- **Wenn möglich**: Falls Rollback möglich ist, wird automatisch zurückgerollt
- **Alte Version**: Alte Version wird wiederhergestellt
- **Sicherheit**: Verhindert, dass Device in fehlerhaftem Zustand bleibt
- **Schnell**: Rollback erfolgt automatisch ohne User-Intervention

**Manueller Rollback**
- **Falls nötig**: Falls automatischer Rollback nicht möglich ist, manueller Rollback
- **User-Intervention**: User muss Rollback manuell starten
- **Anleitung**: System gibt Anleitung für manuellen Rollback
- **Support**: Support kann bei manuellem Rollback helfen

## Implementierungs-Notizen

- **Extrem lightweight**: Muss extrem lightweight sein (minimaler Footprint)
- **Custom Scripts**: User sollen noch custom scripts auf solche Devices kopieren können
- **Leichtgewichtige Sprachen**: Nur wirklich leichtgewichtige, performante Sprachen unterstützen
- Sollte verschiedene Microcontroller unterstützen
- Muss robustes Error-Handling haben
- Sollte Low-Power-Modi unterstützen
- Muss Network-Resilience haben
- Sollte OTA-Updates unterstützen (optional)
- Muss gut dokumentiert sein mit Examples
- **Muss event-basierte Cache-Invalidierung haben**: Für Capability-Updates
- **Muss automatische Re-Negotiation für wichtige Änderungen unterstützen**: Nach OTA-Updates
- **Muss Asgard-Integration haben**: Für zentrale Capability-Synchronisation (wenn Asgard vorhanden)
- **Muss leitenden Server bei mehreren Asgard-Servern bestimmen können**: Basierend auf `created_at`
- **Muss Konflikt-Lösung haben**: Automatisch + User-Intervention für Capability-Konflikte
- **Muss OTA-Update-Mechanismus haben**: Für Jotunheim-Devices
- **Muss digitale Signaturen + Checksum-Verifikation haben**: Für Update-Sicherheit
- **Muss automatischen Rollback haben**: Wenn möglich, bei fehlgeschlagenen Updates
- **Muss manuellen Rollback haben**: Falls automatischer Rollback nicht möglich
- **Muss drahtlose Stream-Übertragung unterstützen**: Von Haus aus bei Jotunheim
- **Muss bidirektionale Streams vorbereiten**: Video und Audiostreams bidirektional senden können
- **Muss Plugin-Vorbereitung haben**: Vorbereitung für extra Plugins für erweiterte Streaming-Features
- **Muss Resource-Limits haben**: Strikte Resource-Limits, damit User noch Scripts kopieren können
- **Performance**: Muss optimiert sein für Resource-Constraints von Microcontrollern
- **Datenschutz**: Muss Privacy-by-Design implementieren, soweit möglich
- **Sicherheit**: Muss Security-Mechanismen haben, soweit Hardware es erlaubt

