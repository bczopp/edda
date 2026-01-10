# Jötnar - IoT-Devices Client

## Übersicht

Jötnar ist der Client für IoT-Devices (inkl. ESP32/Microcontroller). Er stellt eine minimale, lightweight Implementation für kleine Devices bereit.

## Zielplattformen

- ESP32 (ESP-IDF, Arduino)
- ESP8266
- Raspberry Pi Pico
- Andere Microcontroller mit WiFi/Network

## Projektstruktur

```
jotnar/
├── src/
│   ├── esp32/           # ESP32 Implementation
│   │   ├── main/
│   │   ├── services/
│   │   └── utils/
│   ├── generic/         # Generic Implementation
│   │   ├── protocol/
│   │   ├── services/
│   │   └── utils/
│   └── shared/         # Shared Code
│       ├── protocol/
│       └── utils/
├── config/
└── examples/
```

## Features

### Core Features

- **Remote Control**: Device kann von anderen Devices gesteuert werden
- **Toolcalling Protocol**: Token-effizientes Toolcalling-Protokoll
- **Capability Negotiation**: Device teilt Capabilities mit
- **Streaming Support**: Streaming für große Datenmengen

### IoT-Specific Features

- **Minimal Footprint**: Geringer Speicher- und CPU-Verbrauch
- **Low Power**: Energie-effiziente Operation
- **Network Resilience**: Robustes Error-Handling für Netzwerk-Fehler
- **OTA Updates**: Over-the-Air Updates (optional)

## Service Integration

**Hinweis**: Jötnar-Devices haben KEINEN Odin-Prozess, da sie zu klein sind. Stattdessen kommunizieren sie direkt über das spezielle Toolcalling-Protokoll mit anderen Devices.

### Remote Control
- Empfängt Commands von anderen Devices
- Führt Commands aus
- Sendet Results zurück

### Capability Exposure
- Device teilt verfügbare Tools/Functions mit
- Capability-Negotiation mit Controller
- Dynamic Capability Updates

## Toolcalling Protocol

### Protocol Features
- **MessagePack-based**: Binary, kompakt
- **Token-efficient**: Deutlich weniger Overhead als MCP
- **Streaming**: Unterstützung für Streaming
- **Error Recovery**: Robustes Error-Handling

### Message Types
- **CAPABILITY_REQUEST**: Request Device Capabilities
- **CAPABILITY_RESPONSE**: Device Capabilities
- **TOOL_CALL**: Tool/Function Call
- **TOOL_RESPONSE**: Tool Response
- **STREAM_START**: Start Streaming
- **STREAM_CHUNK**: Stream Chunk
- **STREAM_END**: End Streaming
- **ERROR**: Error Message
- **HEARTBEAT**: Keep-Alive

### Tool Definition

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

## Capability Negotiation

### Warum Capability Negotiation?

ESP32-Devices sind sehr variabel einsetzbar und können unterschiedlich konfiguriert sein:
- **Verschiedene Sensoren**: Temperatur, Feuchtigkeit, Bewegung, Licht, etc.
- **Verschiedene Aktoren**: LEDs, Motoren, Relais, Displays, etc.
- **Verschiedene Interfaces**: GPIO, I2C, SPI, UART, ADC, PWM, etc.
- **Verschiedene Funktionen**: Jedes Device kann andere Tools/Funktionen anbieten

**Das verbundene Device (Controller, z.B. Midgard/Asgard) muss wissen, was das Jötnar-Device kann, um es steuern zu können.**

### Capability Structure

```typescript
interface JotnarCapabilities {
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

1. **Connection Establishment**: Controller stellt TCP/UDP-Verbindung zum Jötnar-Device her
2. **Capability Request**: Controller sendet `CAPABILITY_REQUEST` Message
3. **Capability Processing**: Controller analysiert Capabilities und erstellt Tool-Registry
4. **Tool Discovery & Registration**: Controller registriert alle verfügbaren Tools
5. **Dynamic Capability Updates**: Capabilities werden bei Kopplung/Verbindung propagiert

## Implementation Constraints

### Memory Constraints
- **Minimal RAM**: Muss mit wenig RAM auskommen
- **Stack Management**: Effizientes Stack-Management
- **Memory Pooling**: Memory-Pooling für bessere Performance

### CPU Constraints
- **Low CPU Usage**: Minimaler CPU-Verbrauch
- **Async Processing**: Asynchrone Verarbeitung
- **Task Scheduling**: Effizientes Task-Scheduling

### Network Constraints
- **Low Bandwidth**: Optimiert für niedrige Bandbreite
- **Connection Resilience**: Robustes Error-Handling
- **Reconnection**: Automatische Wiederverbindung

## Example: ESP32 Implementation

### Hardware Requirements
- ESP32 mit WiFi
- Optional: Bluetooth
- Optional: Sensors/Actuators

### Software Stack
- ESP-IDF oder Arduino Framework
- WiFi Stack
- TCP/IP Stack
- MessagePack Library

### Features
- WiFi Connection Management
- TCP/UDP Communication
- Toolcalling Protocol
- Remote Control

## Abhängigkeiten

- **Edda Core Library**: Minimal (nur Jötnar Protocol)
- Network Stack (WiFi, TCP/IP)
- MessagePack Library
- Platform SDK (ESP-IDF, Arduino, etc.)

## Integration

- **Kein Odin**: Jötnar-Devices haben keinen Odin-Prozess
- **Toolcalling Protocol**: Direkte Kommunikation über spezielles Protocol
- **Midgard/Asgard**: Werden als Controller verwendet (Odin erkennt Jötnar-Devices und steuert sie)
- **Bifrost**: Für erweiterte Kommunikation (optional)
- **Heimdall**: Für Security (optional, wenn Encryption unterstützt)

## Performance

### Performance-Optimierungen
- **Minimaler Footprint**: Optimiert für geringen RAM- und Flash-Verbrauch
- **Effiziente Serialisierung**: MessagePack für minimale Datenübertragung
- **Low CPU Usage**: Minimale CPU-Nutzung für Battery-Life
- **Optimierte Network-Protocols**: Minimale Overhead für Netzwerk-Kommunikation
- **Streaming Support**: Effizientes Streaming für große Datenmengen

### Performance-Metriken
- Minimaler RAM-Verbrauch (< 10KB für Basis-Implementation)
- Schnelle Tool-Execution (< 100ms für einfache Tools)
- Effiziente Netzwerk-Kommunikation (minimaler Overhead)

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden übertragen
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, wo möglich
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **Opt-in Encryption**: Optional Verschlüsselung für sensible Daten

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden übertragen
- **User Control**: User hat Kontrolle über Datenübertragung

## Sicherheit

### Security-Features
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

### Wichtig: Capabilities sind hauptsächlich für Jötnar-Devices

**Jötnar-Devices benötigen Capability-Negotiation**
- **Variable Konfiguration**: Jötnar-Devices (ESP32, etc.) haben variable Konfigurationen
- **Tools müssen bekannt sein**: Controller muss wissen, welche Tools/Funktionen verfügbar sind
- **Bei Kopplung**: Capabilities werden bei jeder Kopplung/Verbindung propagiert
- **Auf Anfrage**: Controller kann jederzeit Capabilities anfragen

**Andere Devices**
- **Midgard/Alfheim/Asgard**: Haben standardisierte Capabilities, weniger dynamisch
- **Capability-Updates**: Selten, hauptsächlich bei Updates

### Capability-Propagation

**Bei Kopplung**
- **Automatisch**: Bei jeder Kopplung/Verbindung werden Capabilities automatisch propagiert
- **Jötnar-Devices**: Senden Capabilities bei Verbindungsaufbau
- **Controller**: Empfängt und registriert Capabilities

**Auf Anfrage**
- **Controller kann anfragen**: Controller kann jederzeit `CAPABILITY_REQUEST` senden
- **Jötnar-Device antwortet**: Device sendet aktuelle Capabilities zurück
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
- **Asgard als Update-Server**: Asgard fungiert als lokaler Update-Server für Jötnar-Devices
- **Lokale Verteilung**: Updates werden lokal über Asgard verteilt
- **Schnell**: Keine Internet-Verbindung nötig
- **Kontrolle**: User hat volle Kontrolle über Updates

**Globale Updates über Yggdrasil**
- **Yggdrasil als Update-Server**: Yggdrasil fungiert als globaler Update-Server
- **Zentrale Verteilung**: Updates werden zentral von Yggdrasil verteilt
- **Automatisch**: Updates können automatisch verteilt werden
- **Für alle User**: Alle User erhalten Updates von Yggdrasil

### Workflow: Jötnar-Device Update

1. **Update verfügbar**
   - Update wird auf Asgard oder Yggdrasil verfügbar
   - Update wird für Jötnar-Device bereitgestellt

2. **Update-Verteilung**
   - **Lokal**: Asgard verteilt Update an Jötnar-Devices im lokalen Netzwerk
   - **Global**: Yggdrasil verteilt Update an alle Jötnar-Devices weltweit

3. **Update-Installation**
   - Jötnar-Device empfängt Update
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

- Muss sehr lightweight sein (minimaler Footprint)
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
- **Muss OTA-Update-Mechanismus haben**: Für Jötnar-Devices
- **Muss digitale Signaturen + Checksum-Verifikation haben**: Für Update-Sicherheit
- **Muss automatischen Rollback haben**: Wenn möglich, bei fehlgeschlagenen Updates
- **Muss manuellen Rollback haben**: Falls automatischer Rollback nicht möglich
- **Performance**: Muss optimiert sein für Resource-Constraints von Microcontrollern
- **Datenschutz**: Muss Privacy-by-Design implementieren, soweit möglich
- **Sicherheit**: Muss Security-Mechanismen haben, soweit Hardware es erlaubt

