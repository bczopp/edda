# Loki - Script Execution Service

## Übersicht

Loki ist ein unabhängiger Service für Script-Execution, der user-generierte Scripte per gRPC zugänglich macht. Jedes Script wird zu einer aufrufbaren gRPC-Funktion, die das Script direkt auf dem Device ausführt.

**Wichtig**: Loki ist ein unabhängiger Service und nicht Teil der Jotunheim-Platform. Jotunheim-Platform ruft Loki via gRPC auf, aber Loki kann auch von anderen Platformen genutzt werden.

**Tests ausführen:** Von `loki/`: `docker compose -f docker-compose.test.yml run --rm loki-test` oder `./scripts/run-tests.sh` / `.\scripts\run-tests.ps1`. Von Repo-Root: `loki/scripts/run-tests.sh` bzw. `.\loki\scripts\run-tests.ps1`. **CI:** Bei Push/PR auf `loki/**` läuft die Pipeline [.github/workflows/loki.yml](../.github/workflows/loki.yml) (Test im Container, Lint).

## Verantwortlichkeiten

### 1. Script-Execution
- **User-generierte Scripte**: Macht user-generierte Scripte per gRPC zugänglich
- **Tool-Konfigurationsdatei**: Loki lädt eine Konfigurationsdatei mit den vom User zur Verfügung gestellten Tools (KEIN Protocol)
- **Dynamische gRPC-Funktionen**: Jedes Script wird zu einer aufrufbaren gRPC-Funktion (`Script_<script_name>`)
- **Direkte Ausführung**: Funktion führt Script direkt auf Device aus (nichts anderes)
- **Resource-Management**: Scripts müssen Resource-Limits beachten

### 2. Koordination der 3 Kinder
Loki koordiniert drei spezialisierte Sub-Services:

#### Fenrir - Aggressive Tasks
- **Rolle**: Intensive Tasks, Hardware-Control
- **Funktionen**: 
  - Hardware-nahe Operationen
  - GPIO-Control
  - Sensor-Reading (intensiv)
  - Actuator-Control
  - Low-level Hardware-Access

#### Jörmungandr - Network/Communication
- **Rolle**: Netzwerk-Tasks, Kommunikation
- **Funktionen**:
  - Netzwerk-Operationen
  - HTTP/HTTPS Requests
  - WebSocket-Verbindungen
  - MQTT-Communication
  - Network-Protocol-Handling

#### Hel - Data/Storage
- **Rolle**: Datenverwaltung, Storage
- **Funktionen**:
  - Daten-Speicherung
  - File-System-Operationen
  - Daten-Aggregation
  - Cache-Management
  - Data-Processing

## Script-Execution-System

### Script-Sprachen
- **Unterstützte Sprachen**: 
  - **Lua** (Standard für ESP32 und kleinere Devices)
  - **Größere Devices**: Mehr Optionen möglich (Python, JavaScript, etc., abhängig vom Device)
- **Language-spezifische Features**: 
  - **Lua**: Minimaler Footprint, optimiert für Embedded Systems
  - **Größere Devices**: Mehr Features möglich (abhängig vom Device)
- **Sprach-Auswahl**: Script-Sprachen werden basierend auf Device-Capabilities ausgewählt
- **Nur leichtgewichtige, performante Sprachen**: Nur wirklich leichtgewichtige, performante Sprachen unterstützen
- **Leichtere Rust-Version**: Wenn es eine noch leichtere Version von Rust gibt, soll auch Loki darin geschrieben werden, wenn signifikante Verbesserungen dadurch erhält
- **Custom Scripts**: User sollen noch custom scripts auf solche Devices kopieren können

### Ausführungsmodell
- **Direkte Ausführung**: Keine Sandbox für Performance
- **Device-abhängig**: Script-Sprache abhängig vom Device (ESP32 = Lua, größere Devices = mehr Optionen)
- **Resource-Limits**: Scripts müssen Resource-Limits beachten
- **gRPC-Funktion pro Script**: Jedes Script wird zu einer aufrufbaren gRPC-Funktion

### Flexibilität für verschiedene Devices
- **ESP32**: Minimaler Footprint, Lua als primäre Script-Sprache
- **ESP8266**: Ähnlich ESP32
- **Raspberry Pi Pico**: Mehr Optionen möglich
- **Größere Devices**: Mehr Script-Sprachen, mehr Features

## Loki Service Protocol

Das **Loki Service Protocol** ist ein gRPC-basiertes Protokoll für Script-Execution. Es ermöglicht:
- **Statische Methoden**: GetCapabilities, ListScripts, RegisterScript, etc.
- **Dynamische Script-Funktionen**: Jedes User-Script wird zu einer aufrufbaren gRPC-Funktion (`Script_<script_name>`)
- **Tool-Konfiguration**: Loki lädt Konfigurationsdatei mit User-Tools (KEIN Protocol)
- **Script-Execution**: Direkte Ausführung von Scripts auf Device

## gRPC-Service-Definition

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

## Kommunikation

### gRPC
- **Platform ↔ Loki**: Platformen (Jotunheim, Midgard, Asgard, etc.) kommunizieren mit Loki via gRPC
- **Type-safe**: Protobuf garantiert korrekte Script-Definitionen
- **Streaming**: Built-in Streaming für große Script-Responses

**gRPC Connection-Management:**
- **Connection-Pooling**: Wiederverwendung von Verbindungen für bessere Performance
- **Connection Reuse**: Connections werden effizient wiederverwendet
- **Automatische Reconnection**: Kombination aus sofortigem Versuch + Exponential Backoff
  - Sofortiger Reconnect-Versuch bei Verbindungsabbruch
  - Nach erstem Fehler beginnt Exponential Backoff
  - Maximale Wartezeit (z.B. 60 Sekunden)
  - Kontinuierliche Versuche zur Wiederherstellung
- **Connection Monitoring**: Verbindungsstatus wird überwacht

**gRPC Error-Handling:**
- **gRPC Status-Codes**: gRPC-Fehler werden über Status-Codes behandelt
- **Retry-Mechanismen**: Automatischer Retry mit Exponential Backoff (siehe gemeinsame Klärungspunkte)
- **Timeout-Konfiguration**: Adaptive Timeouts mit Minimum/Maximum
- **Fallback**: Bei Fehler Fallback zu alternativen Routen

### Bifrost-Verbindung (Optional)
- **Optional**: Bifrost-Verbindung nur bedingt nötig
- **gRPC-Streams**: Wenn gRPC-Streams möglich sind und verschlüsselt (TLS) oder in abgesichertem Netzwerk, brauchen wir keine Bifrost-Verbindung
- **Verschlüsselung**: Streams müssen verschlüsselt übertragen werden (TLS) oder in abgesichertem Netzwerk
- **Fallback**: Bifrost als Fallback wenn gRPC-Streams nicht möglich

## Tool-Konfigurationsdatei

**WICHTIG**: Loki lädt eine Konfigurationsdatei mit den vom User zur Verfügung gestellten Tools. Es gibt kein Protocol für Tool-Discovery.

### Konfigurationsdatei
- **Format**: 
  - **Konfigurationsdatei-Format**: JSON oder YAML (konfigurierbar)
  - **Tool-Definitionen**: Tools werden in der Konfigurationsdatei definiert (Name, Beschreibung, Parameter, Return-Type, etc.)
- **Speicherort**: 
  - **User-bereitgestellt**: Konfigurationsdatei wird vom User bereitgestellt
  - **Standard-Pfad**: Standard-Pfad für Konfigurationsdatei (z.B. `~/.loki/tools.json`)
- **Tool-Validierung**: 
  - **Validierung beim Laden**: Tool-Definitionen werden beim Laden validiert (Schema-Validierung)
  - **Fehlerbehandlung**: Bei Validierungs-Fehlern wird Fehler geloggt, ungültige Tools werden ignoriert
- **Tool-Updates**: 
  - **Hot-Reload**: Konfigurationsdatei kann zur Laufzeit neu geladen werden (Hot-Reload)
  - **Update-Handling**: Bei Tool-Updates werden gRPC-Funktionen aktualisiert
- **Laden**: Loki lädt die Konfigurationsdatei beim Start oder bei Änderungen

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

### Loki-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Tool-Konfigurationsdatei (siehe Abschnitt "Tool-Konfigurationsdatei")
- Resource-Limits
- Sandboxing-Einstellungen

## Integration

### Platform-Integration
- **Jotunheim**: Jotunheim-Platform ruft Loki via gRPC auf für Script-Execution
- **Andere Platformen**: Midgard, Asgard, etc. können Loki ebenfalls nutzen
- **Service-Unabhängigkeit**: Loki ist unabhängig von Platformen

### Service-Integration
- **Odin**: Kann Loki für Script-Execution nutzen (optional)
- **Thor**: Nutzt Loki via generisches Tool-Calling (Einherjar Protocol)
- **Alle Services**: Können Loki-Funktionen via Einherjar Protocol entdecken und aufrufen
- **Heimdall**: Für Security (optional, wenn Encryption unterstützt)

## Function Discovery & Capability Exposure

### Einherjar Protocol Integration

Loki implementiert das **Einherjar Protocol** für Function Discovery:

**Verfügbare Funktionen:**
- `GetCapabilities()` - Device-Capabilities abfragen
- `ListScripts()` - Registrierte Scripts auflisten  
- `RegisterScript()` - Neues Script registrieren
- `Script_<script_name>()` - Dynamische Script-Funktionen (zur Laufzeit generiert)

**Integration:**
- Jeder Service (Thor, Odin, etc.) kann via Einherjar Protocol Loki's Funktionen abfragen
- Loki gibt alle verfügbaren Funktionen bekannt
- Services rufen Funktionen direkt via gRPC auf

**Beispiel-Workflow:**
1. Service fragt Device: `GetCapabilities()`
2. Device antwortet: `["RegisterScript", "ListScripts", "Script_temperature_monitor", ...]`
3. Service ruft auf: `RegisterScript(name="led_control", content="...", language="lua")`
4. Loki registriert Script und erstellt dynamische Funktion: `Script_led_control()`
5. Funktion wird via Einherjar Protocol bekannt gegeben
6. Service kann nun aufrufen: `Script_led_control(params)`

**Vorteile:**
- **Generisch**: Nicht auf Thor beschränkt - alle Services können Loki nutzen
- **Discovery**: Services entdecken Funktionen automatisch
- **Type-Safe**: gRPC + Protobuf garantiert korrekte Typen
- **Dynamisch**: User-Scripts werden zur Laufzeit zu aufrufbaren Funktionen

## Resource-Management

**Extrem Lightweight (Essentiell für Jotunheim/Loki):**
- **Extrem lightweight**: Jotunheim/Loki müssen extrem lightweight sein
- **Custom Scripts**: User sollen noch custom scripts auf solche Devices kopieren können
- **Leichtgewichtige Sprachen**: Nur wirklich leichtgewichtige, performante Sprachen unterstützen
- **Minimaler Footprint**: Optimiert für geringen RAM- und Flash-Verbrauch
- **Resource-Reserve**: Genug Ressourcen für User-Scripts reservieren

**Resource-Limits:**
- CPU, Memory, Disk-Limits für Script-Execution
- Konfigurierbar pro Script-Typ
- Resource-Limits werden pro Script durchgesetzt
- **Strikte Limits**: Strikte Resource-Limits, damit User noch Scripts kopieren können

**Resource-Monitoring:**
- Überwachung von System-Ressourcen (CPU, RAM, Disk, Network)
- Resource-Usage wird überwacht während Script-Execution
- Allokation von Ressourcen für Scripts
- **Resource-Tracking**: Kontinuierliches Tracking, um Platz für User-Scripts zu gewährleisten

**Bei Resource-Exhaustion:**
- Verhindert Resource-Exhaustion durch Limits
- Scripts werden in Queue gelegt, wenn Ressourcen knapp
- Warten auf verfügbare Ressourcen
- **Priorität**: System-Scripts haben Priorität, aber User-Scripts müssen auch ausführbar bleiben

**Resource-Effizienz:**
- Minimale RAM-Nutzung, nur so viel wie nötig
- Optimiert für geringen Footprint auf IoT-Devices
- **Code-Effizienz**: Code sollte so kurz wie möglich sein, während Lesbarkeit erhalten bleibt

## Performance

### Performance-Optimierungen
- **Minimaler Footprint**: Optimiert für geringen RAM- und Flash-Verbrauch
- **Effiziente Serialisierung**: Protobuf für minimale Datenübertragung
- **Low CPU Usage**: Minimale CPU-Nutzung
- **Optimierte Network-Protocols**: gRPC mit HTTP/2 für minimale Overhead
- **Streaming Support**: Built-in gRPC-Streaming für große Datenmengen
- **Connection Pooling**: Wiederverwendung von gRPC-Verbindungen

### Performance-Metriken
- Minimaler RAM-Verbrauch
- Schnelle Script-Execution
- Effiziente Netzwerk-Kommunikation (minimaler Overhead)

### Performance-Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Resource-Usage (CPU, Memory, Disk)
- Performance-Tracking für alle Script-Execution-Requests
- Resource-Monitoring: Überwachung von Script-Ressourcen
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Monitoring & Logging

### Strukturiertes Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation
- Umfassendes Logging für Debugging und Monitoring

## Sicherheit

### Security-Features
- **Optional Encryption**: Verschlüsselung für sensible Daten (wenn unterstützt)
- **TLS Encryption**: TLS-Verschlüsselung für Netzwerk-Verbindungen (wenn unterstützt)
- **Authentication**: Device-Authentifizierung über Heimdall (optional)
- **Input Validation**: Validierung aller eingehenden Script-Requests
- **Resource Limits**: Scripts müssen Resource-Limits beachten

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Minimal Attack Surface**: Minimale Angriffsfläche durch Lightweight-Design

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden übertragen
- **Lokale Verarbeitung**: Scripts werden lokal verarbeitet
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden übertragen
- **User Control**: User hat Kontrolle über Script-Execution

## Implementierungs-Notizen

- **Extrem lightweight**: Muss extrem lightweight sein (minimaler Footprint)
- **Custom Scripts**: User sollen noch custom scripts auf solche Devices kopieren können
- **Leichtgewichtige Sprachen**: Nur wirklich leichtgewichtige, performante Sprachen unterstützen
- Sollte verschiedene Devices unterstützen (ESP32, ESP8266, Raspberry Pi Pico, etc.)
- Muss robustes Error-Handling haben
- Sollte Low-Power-Modi unterstützen (für IoT-Devices)
- Muss Network-Resilience haben
- Muss gut dokumentiert sein mit Examples
- **Muss Resource-Limits für Scripts haben**: Verhindert Resource-Exhaustion, damit User noch Scripts kopieren können
- **Muss dynamische gRPC-Funktionen unterstützen**: Jedes Script wird zu einer gRPC-Funktion
- **Muss Koordination der 3 Kinder haben**: Fenrir, Jörmungandr, Hel
- **Performance**: Muss optimiert sein für Resource-Constraints von Microcontrollern
- **Datenschutz**: Muss Privacy-by-Design implementieren, soweit möglich
- **Sicherheit**: Muss Security-Mechanismen haben, soweit Hardware es erlaubt

