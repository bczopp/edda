# Thor - Action Executor Plugin

## Übersicht

Thor ist ein Plugin für Action-Execution, das von Odin orchestriert wird. Wenn Funktionen ausgeführt werden müssen, delegiert Odin die Aufgabe an Thor (wenn verfügbar). Thor führt Actions aus, die von Odin geplant wurden. Er nutzt Mjölnir (Hammer), Tanngrisnir & Tanngnjóstr (Goats) und den Chariot (Streitwagen).

**Tests ausführen:** Von `thor/`: `docker compose -f docker-compose.test.yml run --rm thor-test` oder `./scripts/run-tests.sh` / `.\scripts\run-tests.ps1`. Von Repo-Root: `thor/scripts/run-tests.sh` bzw. `.\thor\scripts\run-tests.ps1`. **CI:** Bei Push/PR auf `thor/**` läuft die Pipeline [.github/workflows/thor.yml](../.github/workflows/thor.yml) (Test im Container, Lint).

**Plugin-Architektur**: Thor ist ein optionales Plugin, das modular zur Odin-Installation hinzugefügt werden kann. Odin entscheidet selbst, ob eine Aufgabe an Thor delegiert werden muss (wenn Funktionen ausgeführt werden müssen) oder ob Odin selbst antworten kann (bei einfachen Fragen).

**Plugin-Interface**: Thor implementiert das `OdinPlugin`-Interface mit:
- `get_title()`: Gibt "Thor - Action Executor" zurück
- `get_description()`: Beschreibt die Action-Execution-Funktionalität
- `get_functions()`: Gibt alle verfügbaren Action-Types zurück (Function Call Protocol)

**Einherjar Protocol**: Thor implementiert das Einherjar Protocol, um seine verfügbaren Funktionen und Zuständigkeits-Domains offenzulegen. Odin nutzt diese Informationen, um automatisch zu erkennen, wann Thor zuständig ist.

**Responsibility Service**: Thor implementiert das Responsibility Service, um Zuständigkeit zu übernehmen, zurückzugeben oder zurückzuweisen. Wenn eine Aufgabe nicht mehr Action-Execution ist, gibt Thor die Zuständigkeit an Odin zurück.

## Komponenten

### Mjölnir - Action Execution Engine
- Führt Actions tatsächlich aus
- Verwaltet Action-Lifecycle
- Handhabt verschiedene Action-Types

### Tanngrisnir & Tanngnjóstr - Resource Managers
- Verwaltet System-Ressourcen
- Überwacht Resource-Usage
- Allokiert Ressourcen für Actions

### Chariot - Task Queue & Scheduler
- Verwaltet Action-Queue
- Plant Action-Execution
- Handhabt Prioritäten

## Verantwortlichkeiten

### 1. Action Execution
- **Action-Types**: 
  - **FILE_OPERATION**: Datei-Operationen (Create, Read, Update, Delete, Move, Copy)
  - **SYSTEM_COMMAND**: System-Commands (Shell-Commands, Script-Execution)
  - **NETWORK_OPERATION**: Netzwerk-Operationen (HTTP-Requests, API-Calls)
  - **DEVICE_CONTROL**: Device-Control (Hardware-Steuerung, Sensor-Zugriff)
  - **APPLICATION_CONTROL**: Application-Control (App-Start, App-Stop, App-Interaktion)
- **Action-Parsing**: 
  - **LLM-Response-Parsing**: Actions werden aus LLM-Responses erkannt (strukturierte Outputs)
  - **Standard-Format**: Standard-Format für Action-Definitionen (JSON-basiert)
  - **Mehrdeutige Definitionen**: Bei mehrdeutigen Action-Definitionen wird User um Klärung gebeten
- **Neue Action-Types**: Neue Action-Types können über Plugin-Interface hinzugefügt werden

### 2. Resource Management
- Überwacht System-Ressourcen (CPU, RAM, Disk, Network)
- Allokiert Ressourcen für Actions
- Verhindert Resource-Exhaustion

### 3. Task Scheduling
- Verwaltet Action-Queue
- Handhabt parallele Execution
- **Conflict Resolution**: Löst Konflikte bei parallelen Actions (Locking)
- **Lock Management**: Verwaltet Locks für lokale Resources

### 4. Error Handling
- Fängt Action-Fehler ab
- Retry-Mechanismen
- Error-Reporting

### 5. Conflict Resolution
- **Locking**: File Locking für Konfliktlösung bei parallelen Actions
- **Lokales File Locking**: Für lokale Dateien/Resources auf demselben Device
- **Distributed File Locking**: File Locking über Asgard/Yggdrasil für geteilte Resources
- **Pessimistic Locking + Transactions**: Verhindert Race Conditions
- **Deadlock Detection**: Erkennt und löst Deadlocks auf (Timeout + Cycle-Detection)

### 6. Tool-Calling-Agent (Kernfunktion)
- **Thor ist der Tool-Calling-Agent**: Thor führt Actions aus, die von Odin geplant wurden
- **Action-Erkennung**: Thor erkennt automatisch, welche Actions nötig sind:
  - Datei-Änderungen → `FILE_OPERATION` Actions
  - System-Commands → `SYSTEM_COMMAND` Actions
  - Network-Requests → `NETWORK_OPERATION` Actions
  - Device-Control → `DEVICE_CONTROL` Actions
- **Action-Ausführung**: Thor führt erkannte Actions aus (via Mjölnir)
- **Response-Generierung**: Thor erstellt Text-Responses für Odin basierend auf Action-Results

### 7. Action-Execution für Plugin-Ergebnisse
- **Strukturierte Ergebnisse von Plugins**: Wenn Odin strukturierte Ergebnisse von Valkyries oder Frigg erhält, die Actions benötigen, leitet Odin diese an Thor weiter
- **Ergebnis-Analyse**: Thor analysiert strukturierte Ergebnisse (z.B. `ValkyrieResult`) und erkennt Actions (Datei-Änderungen, Commands, etc.)
- **Action-Ausführung**: Thor führt erkannte Actions aus
- **Ergebnis-Rückgabe**: Thor gibt `ThorResult` an Odin zurück (mit Text-Response und Action-Results)

## Service-Interfaces

### Inputs
- `ThorAction` von Odin (via gRPC)
- Resource Requests
- **Strukturierte Ergebnisse von Plugins**: Odin leitet strukturierte Ergebnisse von Valkyries oder Frigg an Thor weiter, wenn Actions benötigt werden
- **Cross-Device Actions**: ThorAction von anderen Devices (via gRPC über Bifrost)

### Outputs
- `ThorResult` mit Status, Result, Error, Execution Time (via gRPC)
- **Text-Response**: Text-Response für Odin (basierend auf Action-Results)
- **Action-Results**: Ergebnisse der ausgeführten Actions (File-Operations, System-Commands, etc.)
- **Cross-Device Results**: `ThorResult` an andere Devices (via gRPC über Bifrost)

### gRPC Service Definition

**Thor Service (gRPC):**
```protobuf
service ThorService {
  rpc ExecuteAction(ThorAction) returns (ThorResult);
  rpc StreamAction(stream ActionChunk) returns (stream ResultChunk);
}
```

## DTO-Definitionen

### ThorAction

`ThorAction` ist das DTO für Actions, die von Odin an Thor gesendet werden.

**Protobuf-Definition:**
```protobuf
message ThorAction {
  string action_id = 1;              // Eindeutige Action-ID
  ActionType action_type = 2;         // Typ der Action
  string target_resource = 3;         // Ziel-Ressource (Datei, URL, etc.)
  map<string, string> parameters = 4; // Action-spezifische Parameter
  optional ActionPriority priority = 5; // Priorität (optional)
  optional string session_id = 6;     // Session-ID für Kontext
  optional ActionMetadata metadata = 7; // Zusätzliche Metadaten
}

enum ActionType {
  FILE_OPERATION = 0;      // Datei-Operationen
  SYSTEM_COMMAND = 1;      // System-Commands
  NETWORK_OPERATION = 2;   // Netzwerk-Operationen
  DEVICE_CONTROL = 3;      // Device-Control
  APPLICATION_CONTROL = 4; // Application-Control
  TERMINAL_OPERATION = 5;  // Terminal-Emulation (PTY)
  UI_AUTOMATION = 6;       // UI-Automation (Klicks, Cursor)
  SCHEDULER_OPERATION = 7; // Scheduler (Cron, Task Scheduler, etc.)
  JOTUNHEIM_OPERATION = 8; // Jotunheim-Devices (IoT, Tool-Calling via Einherjar)
}

enum ActionPriority {
  LOW = 0;
  NORMAL = 1;
  HIGH = 2;
  URGENT = 3;
}

message ActionMetadata {
  optional string source = 1;         // Quelle (z.B. "odin", "valkyries")
  optional int64 timeout_ms = 2;     // Timeout in Millisekunden
  optional bool requires_lock = 3;   // Benötigt File-Lock
}
```

**Validierungsregeln:**
- `action_id`: Muss eindeutig sein, UUID-Format empfohlen
- `action_type`: Muss gültiger ActionType sein
- `target_resource`: Muss nicht leer sein
- `parameters`: Muss für alle erforderlichen Parameter gefüllt sein (abhängig von action_type)

### ThorResult

`ThorResult` ist das DTO für Results, die von Thor an Odin zurückgesendet werden.

**Protobuf-Definition:**
```protobuf
message ThorResult {
  string action_id = 1;              // Action-ID (zur Zuordnung)
  ActionStatus status = 2;            // Status der Action
  optional string result = 3;        // Ergebnis (Text-Response)
  optional bytes result_data = 4;    // Binäre Ergebnis-Daten (optional)
  optional string error = 5;          // Fehler-Message (falls vorhanden)
  int64 execution_time_ms = 6;       // Ausführungszeit in Millisekunden
  optional ResultMetadata metadata = 7; // Zusätzliche Metadaten
}

enum ActionStatus {
  SUCCESS = 0;
  FAILED = 1;
  TIMEOUT = 2;
  CANCELLED = 3;
}

message ResultMetadata {
  int64 timestamp = 1;                // Timestamp
  optional map<string, string> action_results = 2; // Action-spezifische Ergebnisse
  optional string resource_locked = 3; // Gesperrte Ressource (falls vorhanden)
}
```

**Validierungsregeln:**
- `action_id`: Muss mit ursprünglicher Action übereinstimmen
- `status`: Muss gültiger ActionStatus sein
- `result` oder `error`: Mindestens eines muss vorhanden sein
- `execution_time_ms`: Muss >= 0 sein

**Kommunikation:**
- **Lokal (gleiches Device)**: Odin ↔ Thor via gRPC
- **Cross-Device**: Device A (Odin) ↔ Device B (Thor) via gRPC über Bifrost
- **Bifrost**: Für Connection-Establishment, dann gRPC für Action-Execution

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
- **Fallback**: Bei Fehler Fallback zu alternativen Routen/Providern

## Action Types

### DEVICE_CONTROL
- Control device functions
- Hardware Control
- Settings Management

### FILE_OPERATION
- File System Operations
- Read, Write, Delete, Move, Copy
- Directory Operations

### NETWORK_OPERATION
- Network Requests
- HTTP/HTTPS Calls
- WebSocket Connections

### APPLICATION_CONTROL
- Application Management
- Start, Stop, Restart Applications
- Process Management

### SYSTEM_COMMAND
- System-level Commands
- Shell Commands
- System Configuration

### TERMINAL_OPERATION
- Terminal-Emulation mit PTY (Pseudo-Terminal)
- Interactive Programs (vim, htop, nano)
- PTY-Management
- Input/Output-Streaming
- Terminal-Size-Management (Rows, Columns)

### UI_AUTOMATION
- UI-Steuerung (Klicks, Cursor, Text-Input)
- Click, Double-Click, Right-Click
- Drag & Drop
- Cursor-Movement
- Text-Input via UI
- Platform-spezifisch (Windows UI Automation API, macOS Accessibility API, Linux AT-SPI)
- Element-Recognition (Buttons, Textfields, etc.)

### SCHEDULER_OPERATION
- Timer & Scheduled Tasks
- Cron (Linux/macOS)
- Task Scheduler (Windows)
- launchd (macOS)
- Google Calendar (optional, Cloud)
- Create/Delete/Update/List scheduled tasks

### JOTUNHEIM_OPERATION
- IoT-Device-Control via Jotunheim-Bridge
- Device-On/Off
- Device-Value-Setting (Helligkeit, Temperatur, etc.)
- Device-Status-Query
- Integration mit Jotunheim (IoT-Platform)
- Generisches Tool-Calling für alle Jotunheim-Device-Funktionen
  - Thor fragt Device-Capabilities via Einherjar Protocol ab
  - Device gibt verfügbare Funktionen bekannt (z.B. RegisterScript, ListScripts, Script_*)
  - Thor ruft Funktionen direkt via gRPC auf


## Workflow

### Standard-Workflow

1. **Action empfangen**
   - Odin sendet `ThorAction`
   - Thor validiert Action

2. **Action-Analyse**
   - **Normale Actions**: Thor führt Actions direkt aus (via Mjölnir)
   - **Strukturierte Ergebnisse von Plugins**: Falls Odin strukturierte Ergebnisse von Plugins (Valkyries, Frigg) weiterleitet, analysiert Thor diese und erkennt benötigte Actions

3. **Resource Check** (für normale Actions)
   - Prüft verfügbare Ressourcen
   - Allokiert Ressourcen falls verfügbar
   - Wartet falls Ressourcen knapp

4. **Action Scheduling** (für normale Actions)
   - Fügt Action zu Queue hinzu
   - Plant Execution basierend auf Priorität
   - Wartet auf Execution-Slot

5. **Action Execution**
   - Führt Action aus (via Mjölnir)
   - Überwacht Execution
   - Trackt Progress

6. **Result Processing**
   - Sammelt Result, verarbeitet Errors, berechnet Execution Time

7. **Response**
   - Sendet `ThorResult` zurück an Odin
   - Gibt Ressourcen frei

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

### Thor-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Action-Execution-Einstellungen
- Resource-Limits
- Locking-Konfiguration
- Error-Handling-Einstellungen

## Technische Anforderungen

### Security
- Sandboxing für unsichere Actions
- Permission Checking
- Input Validation
- Output Sanitization

### Performance
- **Parallel Execution**: 
  - **Parallele Actions**: Mehrere Actions können parallel ausgeführt werden (abhängig von Resources)
  - **Concurrency-Limits**: Concurrency-Limits basierend auf verfügbaren Resources
  - **Resource-Contention**: Intelligentes Handling von Resource-Contention (File Locking, Queuing)
- **Async Processing**: Asynchrone Verarbeitung für bessere Performance
- **Resource Pooling**: Effizientes Resource-Pooling
- **Caching**: Caching für häufig verwendete Actions

### Reliability
- **Retry-Mechanismen**: Retry mit Exponential Backoff
- **Timeout Handling**: Adaptive Timeouts mit Minimum/Maximum
- **Error Recovery**: Rollback wenn möglich, sonst Compensation
- **State Persistence**: State wird persistent gespeichert für Wiederherstellung
- **Replay-Mechanismus**: Actions können erneut ausgeführt werden
- **State-Synchronisation**: State kann von anderen Devices übernommen werden

### Monitoring & Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation
- Umfassendes Logging für Debugging und Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Resource-Usage (CPU, Memory, Disk, Network)
- Performance-Tracking für alle Actions
- Resource-Monitoring: Überwachung von System-Ressourcen
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Service-Ausfall-Behandlung

**Innerhalb einer Platform:**
- Fallback ist unnötig - Services müssen existieren, so bauen wir sie ja
- Services sind Teil der Platform-Installation

**Platformübergreifend:**
- Netzwerkplan verwenden für Service-Discovery
- Falls mit Yggdrasil verbunden: Netzwerkplan an Yggdrasil übertragen
- **WICHTIG**: Netzwerkplan darf unter keinen Umständen anderen Usern zugänglich gemacht werden
- Asgard fungiert wie eine weitere Platform (Server-optimiert), ähnlich wie Midgard (Desktop-optimiert) und Alfheim (Mobile-optimiert)

**Fallback-Strategien (nur platformübergreifend):**
- Alternative Route: Falls direkte Verbindung fehlschlägt, Fallback zu Relay (Asgard/Yggdrasil)
- Alternative Services bei Ausfall
- Relay-Fallback bei direkter Verbindung fehlschlägt

**Service-Ausfall-Behandlung:**
- Automatischer Retry mit Exponential Backoff
- Sofortiger Fallback zu alternativen Services (nur platformübergreifend)
- User-Benachrichtigung bei komplettem Service-Ausfall

**User-Kommunikation:**
- Fehlermeldung an User, wenn alle Versuche fehlschlagen
- Error-Logging für Debugging
- User kann später erneut versuchen
- Transparente Fehlerbehandlung

## Datenschutz

### GDPR-Compliance

**Right to Deletion:**
- User kann alle Daten löschen ("Right to be forgotten")
- Sichere Datenlöschung
- Automatische Löschung nach Retention-Policy

**User-Rechte:**
- Right to Access: User können ihre Daten abrufen
- Right to Rectification: User können ihre Daten korrigieren
- Right to Data Portability: User können ihre Daten exportieren
- Right to Object: User können der Datenverarbeitung widersprechen

**Data-Minimization:**
- Nur notwendige Daten werden gespeichert
- Nur notwendige Daten werden verarbeitet
- Purpose Limitation: Daten nur für spezifische Zwecke verwendet
- Storage Limitation: Daten nur so lange gespeichert wie nötig

### Datenschutz-Features
- **Lokale Verarbeitung**: Actions werden lokal verarbeitet, wo möglich
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten ohne Zustimmung
- **Datenverschlüsselung**: Sensible Daten werden verschlüsselt gespeichert
- **User Control**: User hat volle Kontrolle über seine Daten

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Thor sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Odin**: Für Actions (Thor ist ein Plugin für Odin)
- **Heimdall**: Für Permissions (optional)
- **System APIs**: Für Action Execution
- **Resource Management**

### Package-Management (für TypeScript/JavaScript)

**WICHTIG**: Für alle TypeScript/JavaScript-Dependencies muss **bun** verwendet werden (nicht npm oder pnpm).

- **Installation**: `bun install`
- **Script-Ausführung**: `bun run <script>`
- **Grund**: 10-100x schneller als npm, native TypeScript-Support, bessere Performance

### Plugin-Integration (optional, modular)

- **Keine direkte Plugin-Kommunikation**: Thor kommuniziert nicht direkt mit anderen Plugins (Valkyries, Frigg)
- **Action-Execution für Plugin-Ergebnisse**: Wenn Odin strukturierte Ergebnisse von Plugins erhält, die Actions benötigen, leitet Odin diese an Thor zur Action-Execution weiter

### Technische Abhängigkeiten (Erweitert)

**Core**:
- Rust (tokio, tonic, serde, tracing, anyhow)

**Terminal-Emulation**:
- `pty` oder `portable-pty`: PTY-Management für interactive Programs
- `tokio-pty`: Async PTY für tokio
- Use Cases: vim, htop, nano, interactive shell-sessions

**UI-Automation**:
- `windows-rs`: Windows UI Automation API
- `cocoa`: macOS Accessibility API
- `atspi`: Linux AT-SPI (Assistive Technology Service Provider Interface)
- Use Cases: Klicks, Cursor-Steuerung, Text-Input in UI

**Scheduler-Integration**:
- `cron_parser`: Crontab-Parsing für Linux/macOS
- `windows-service`: Windows Task Scheduler API
- `google-calendar3`: Google Calendar API (optional, Cloud)
- Use Cases: Timer, scheduled tasks, reminders

**Jotunheim-Integration**:
- Jotunheim-Client (gRPC, generiert aus Proto)
- Device-Registry-Integration (via Heimdall)
- Generisches Tool-Calling via Einherjar Protocol
  - Thor fragt Device-Capabilities ab
  - Device gibt verfügbare Funktionen bekannt
  - Thor ruft Funktionen direkt auf (z.B. RegisterScript, Script_*)
- Use Cases: Licht-Steuerung, Heizung, IoT-Geräte, Sensor-Reading, Custom-Scripts

**Package-Management (TypeScript/JavaScript)**:
- **bun**: Bevorzugt für alle TS/JS-Dependencies (nicht npm oder pnpm!)

## Integration

- **Odin**: 
  - Empfängt `ThorAction` von Odin, sendet `ThorResult` zurück
  - Empfängt `ResponsibilityRequest` von Odin, sendet `ResponsibilityResponse` zurück
  - Sendet `ResponsibilityReturn` an Odin, wenn Aufgabe nicht mehr Action-Execution ist
  - Sendet `ResponsibilityRejection` an Odin, wenn Request nicht in Thors Bereich ist
  - Implementiert Einherjar Protocol für Funktions-Offenlegung
- **Heimdall**: Für Permission-Checking
- **System APIs**: Für Action Execution
- **Asgard/Yggdrasil**: Für Distributed Locking (bei geteilten Resources)

## Parallel Action Conflict Resolution

### Problem: Race Conditions

**Gleichzeitiger Zugriff**
- Zwei oder mehr Devices versuchen gleichzeitig auf dieselbe Resource zuzugreifen
- Actions können sich gegenseitig beeinflussen oder überschreiben
- Dateninkonsistenz: Ohne Konfliktlösung können Daten inkonsistent werden

### Lösung: File Locking

**File Locking für Konfliktlösung**
- **First-come-first-served**: Erstes Device/Prozess bekommt Lock, zweites wartet
- **Locking**: Locking verhindert gleichzeitigen Zugriff auf Resources
- **Prozessübergreifend**: Da alle Götter in separaten Prozessen laufen, ist File Locking notwendig

### Locking-Mechanik

**File Locking als primäre Lösung**

Da Odin für Aufgaben, die er anderen Göttern übergibt, neue/parallele Prozesse startet (non-blocking), wird **File Locking** als einheitliche Lösung verwendet (prozessübergreifend). Dies ist notwendig, da alle Götter (Thor, Valkyries, etc.) in separaten Prozessen laufen.

**Lokale Resources (File Locking)**
- **File Locking für lokale Dateien/Resources**: File Locking für lokale Dateien/Resources auf demselben Device
- **Prozessübergreifend**: Funktioniert zwischen verschiedenen Prozessen auf demselben Device
- **Beispiele**: Lokale Dateien, lokale Anwendungen, lokale System-Settings
- **Lock-Types**: Read/Write Locks werden unterstützt (Read-Locks für mehrere gleichzeitige Lese-Operationen, Write-Locks für exklusive Schreib-Operationen)

**Geteilte Resources (Distributed File Locking)**
- **File Locking über Asgard/Yggdrasil**: File Locking über Asgard/Yggdrasil für geteilte Resources (Distributed File Locking)
- **Beispiele**: Geteilte Dateien, Netzwerk-Devices, Cloud-Ressourcen
- **Koordination**: Asgard/Yggdrasil koordiniert Locks zwischen Devices
- **Konsistent**: Garantiert Konsistenz über alle Devices

### Race Condition Prevention

**Kombination: Pessimistic Locking + Transactions**

**Pessimistic Locking**
- **Lock vor Zugriff**: Device bekommt Lock, bevor es auf Resource zugreift
- **Verhindert Konflikte**: Verhindert, dass andere Devices gleichzeitig zugreifen
- **Warten**: Andere Devices warten, bis Lock freigegeben wird

**Transaction-basiert**
- **Atomic Operations**: Actions werden als Transactions ausgeführt
- **Rollback**: Bei Fehler wird Transaction zurückgerollt
- **Isolation**: Transactions sind isoliert voneinander
- **Consistency**: Garantiert Datenkonsistenz

### Deadlock Detection & Resolution

**Kombination: Timeout + Deadlock-Detection**

**Timeout-basiert**
- **Lock-Expiration**: Locks haben automatische Expiration (Timeout)
- **Standard-Timeout**: Standard-Timeout (z.B. 30 Sekunden)
- **Konfigurierbar**: Timeout kann pro Action-Typ konfiguriert werden
- **Automatische Freigabe**: Lock wird automatisch freigegeben nach Timeout

**Deadlock-Detection**
- **System erkennt Deadlocks**: System erkennt Deadlock-Situationen
- **Cycle Detection**: System erkennt Zyklen in Lock-Abhängigkeiten
- **Automatische Auflösung**: System löst Deadlocks automatisch auf
- **Timeout-basiert**: Lock mit längster Wartezeit wird freigegeben (Timeout)

## Error Recovery und Resilience

### Netzwerk-Fehlerbehandlung

**Kombination: Retry → Fallback → Fehler**

**1. Automatischer Retry mit Exponential Backoff**

**Exponential Backoff-Formel:**
```
wait_time = base_delay * (2 ^ retry_count) + jitter
```

- **Base-Delay**: 1 Sekunde (konfigurierbar)
- **Retry-Count**: Anzahl der bisherigen Retry-Versuche
- **Jitter**: Zufälliger Wert (0-500ms) zur Vermeidung von Thundering-Herd-Problem
- **Max-Wait-Time**: 60 Sekunden (konfigurierbar)

**Retry-Limits:**
- **Max-Retries**: 5 Versuche (konfigurierbar)
- **Timeout pro Retry**: 30 Sekunden (konfigurierbar)
- **Gesamt-Timeout**: 5 Minuten (konfigurierbar)

**Retry-Strategie:**
1. **Sofortiger Retry**: Erster Retry-Versuch sofort (0ms Wartezeit)
2. **Exponential Backoff**: Nach jedem fehlgeschlagenen Versuch wird Wartezeit exponentiell erhöht
3. **Max-Retries**: Nach max. 5 Versuchen wird Retry abgebrochen
4. **Timeout**: Jeder Retry-Versuch hat Timeout von 30 Sekunden

**Retry-Beispiel:**
- Versuch 1: Sofort (0ms)
- Versuch 2: Nach 1s + jitter
- Versuch 3: Nach 2s + jitter
- Versuch 4: Nach 4s + jitter
- Versuch 5: Nach 8s + jitter
- Danach: Fallback zu alternativer Route

**2. Sofortiger Fallback**
- **Alternative Route**: Falls Retry fehlschlägt, sofortiger Fallback zu alternativer Route
- **Alternative Provider**: Falls Provider-Fehler, Fallback zu alternativem Provider
- **Relay-Fallback**: Falls direkte Verbindung fehlschlägt, Fallback zu Relay (Asgard/Yggdrasil)
- **Lokales LLM**: Falls Cloud-LLM-Fehler, Fallback zu lokalem LLM

**3. Fehlermeldung**
- **User-Benachrichtigung**: Falls alle Versuche fehlschlagen, Fehlermeldung an User
- **Error-Logging**: Alle Fehler werden geloggt für Debugging
- **Retry-Later**: User kann später erneut versuchen

### Partielle Failures

**Kombination: Rollback wenn möglich, sonst Compensation**

**Rollback (wenn möglich)**
- **Atomic Operations**: Falls Action als Transaction ausgeführt wurde, Rollback möglich
- **Alle Änderungen rückgängig**: Alle Änderungen werden rückgängig gemacht
- **Konsistenter Zustand**: System bleibt in konsistentem Zustand
- **Automatisch**: Rollback erfolgt automatisch bei Fehler

**Compensation (wenn Rollback nicht möglich)**
- **Fehlgeschlagene Teile kompensieren**: Fehlgeschlagene Teile werden kompensiert
- **Gegenteilige Actions**: Gegenteilige Actions werden ausgeführt
- **State-Korrektur**: State wird korrigiert
- **Manuelle Intervention**: Falls Compensation nicht möglich, manuelle Intervention nötig

### Daten-Wiederherstellung

**Kombination: Replay + State-Sync als Fallback**

**Replay (Hauptmethode)**
- **Actions erneut ausführen**: Fehlgeschlagene Actions werden erneut ausgeführt
- **Action-Log**: Alle Actions werden geloggt für Replay
- **Idempotenz**: Actions müssen idempotent sein (mehrfache Ausführung ohne Seiteneffekte)
- **Automatisch**: Replay erfolgt automatisch nach Fehler

**State-Synchronisation (Fallback)**
- **State von anderen Devices**: Falls Replay nicht möglich, State wird von anderen Devices übernommen
- **Asgard/Yggdrasil**: Asgard/Yggdrasil kann State bereitstellen
- **Synchronisation**: State wird synchronisiert
- **Konsistenz**: Garantiert konsistenten State

### Timeout-Handling

**Kombination: Adaptive Timeouts mit Minimum/Maximum**

**Adaptive Timeouts**
- **Basierend auf Historie**: Timeouts werden basierend auf historischen Daten angepasst
- **Durchschnittliche Laufzeit**: Timeout basiert auf durchschnittlicher Laufzeit ähnlicher Actions
- **Dynamisch**: Timeouts werden dynamisch angepasst
- **Lernfähig**: System lernt aus vergangenen Operations

**Minimum/Maximum**
- **Minimum-Timeout**: Mindest-Timeout (z.B. 5 Sekunden) - verhindert zu kurze Timeouts
- **Maximum-Timeout**: Maximal-Timeout (z.B. 300 Sekunden) - verhindert zu lange Timeouts
- **Sicherheit**: Garantiert, dass Timeouts in vernünftigem Bereich bleiben
- **Konfigurierbar**: Minimum/Maximum können konfiguriert werden

## Implementierungs-Notizen

**Programmiersprache:**
- **Rust**: Für maximale Performance, Memory-Safety ohne GC, moderne Tooling, Cross-compilation
- **TypeScript nur im Frontend**: Nur GUI-Frontends (Midgard/Alfheim) nutzen TypeScript

**Technische Anforderungen:**
- Sollte Plugin-Architektur für verschiedene Action-Types haben
- Muss Sandboxing für unsichere Actions unterstützen
- Sollte parallele Execution mit Resource-Limits haben
- Muss Timeout-Mechanismen haben
- Sollte Action-History für Debugging speichern
- Muss Integration mit System-Security haben
- Muss Coding-Aufgaben erkennen können (automatisch + explizit)
- Muss Healthcare-Aufgaben erkennen können (automatisch + explizit)
- Muss Queue-System für Brünnhilde und Frigg haben
- Muss prüfen, ob Valkyries/Frigg verfügbar sind
- Muss Task-Queue-Management haben (Enqueue, Dequeue, Status-Tracking)
- **Muss File Locking unterstützen**: File Locking als primäre Lösung für prozessübergreifendes Locking (da alle Götter in separaten Prozessen laufen)
- **Muss lokales File Locking haben**: Für lokale Dateien/Resources auf demselben Device
- **Muss Distributed File Locking haben**: File Locking über Asgard/Yggdrasil für geteilte Resources
- **Muss Read/Write Locks unterstützen**: Read-Locks für mehrere gleichzeitige Lese-Operationen, Write-Locks für exklusive Schreib-Operationen
- **Muss Pessimistic Locking + Transactions kombinieren**: Verhindert Race Conditions
- **Muss Deadlock-Detection haben**: Erkennt und löst Deadlocks auf (Timeout-basiert + Cycle-Detection)
- **Muss Timeout-basierte Lock-Expiration haben**: Automatische Freigabe nach Timeout
- **Muss Retry-Mechanismus mit Exponential Backoff haben**: Für Netzwerk-Fehler
- **Muss Fallback-Mechanismen haben**: Alternative Routen/Provider bei Fehlern
- **Muss Rollback-Mechanismus haben**: Wenn möglich, bei partiellen Failures
- **Muss Compensation-Mechanismus haben**: Wenn Rollback nicht möglich
- **Muss Replay-Mechanismus haben**: Für Daten-Wiederherstellung
- **Muss State-Synchronisation haben**: Als Fallback für Daten-Wiederherstellung
- **Muss adaptive Timeouts haben**: Mit Minimum/Maximum für Timeout-Handling
- **Muss Einherjar Protocol implementieren**: Für Funktions-Offenlegung und Zuständigkeits-Domains
- **Muss Responsibility Service implementieren**: Für Zuständigkeits-Management (TakeResponsibility, ReturnResponsibility, RejectResponsibility)
- **Muss Zuständigkeits-Rückgabe haben**: Wenn Aufgabe nicht mehr Action-Execution ist
- **Muss Rückweisungs-Mechanismus haben**: Kann Requests zurückweisen, wenn nicht in Thors Bereich

