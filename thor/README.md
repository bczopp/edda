# Thor - Action Executor Service

## Übersicht

Thor ist der Action Executor und führt Actions aus, die von Odin geplant wurden. Er nutzt Mjölnir (Hammer), Tanngrisnir & Tanngnjóstr (Goats) und den Chariot (Streitwagen).

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
- Führt verschiedene Action-Types aus:
  - Device Control
  - File Operations
  - Network Operations
  - Application Control
  - System Commands

### 2. Resource Management
- Überwacht System-Ressourcen (CPU, RAM, Disk, Network)
- Allokiert Ressourcen für Actions
- Verhindert Resource-Exhaustion

### 3. Task Scheduling
- Verwaltet Action-Queue
- Plant Actions basierend auf Priorität
- Handhabt parallele Execution
- **Conflict Resolution**: Löst Konflikte bei parallelen Actions (Priority + Locking)
- **Lock Management**: Verwaltet Locks für lokale Resources

### 4. Error Handling
- Fängt Action-Fehler ab
- Retry-Mechanismen
- Error-Reporting

### 5. Conflict Resolution
- **Priority + Locking**: Kombination aus Priorität und Locking für Konfliktlösung
- **Hybrid Locking**: Lokal für lokale Resources, Distributed für geteilte Resources
- **Pessimistic Locking + Transactions**: Verhindert Race Conditions
- **Deadlock Detection**: Erkennt und löst Deadlocks auf (Timeout + Detection)
- **System-Priority mit User-Override**: System bestimmt Priorität, User kann überschreiben

### 6. Coding Task Recognition & Delegation
- **Erkennung**: Thor erkennt automatisch, ob es sich um eine Coding-Aufgabe handelt
- **User kann explizit angeben**: User kann auch explizit angeben, dass es eine Coding-Aufgabe ist
- **Weiterleitung**: Thor legt Task in Queue für Brünhild (Valkyries)
- **Ergebnis**: Thor erhält Ergebnis von Brünhild aus Queue und gibt es an Odin zurück

### 7. Healthcare Task Recognition & Delegation
- **Erkennung**: Thor erkennt automatisch, ob es sich um eine Healthcare-Aufgabe handelt
- **User kann explizit anfordern**: User kann auch explizit Frigg anfordern oder eine Behandlung starten wollen
- **Weiterleitung**: Thor legt Task in Queue für Frigg
- **Ergebnis**: Thor erhält Ergebnis von Frigg aus Queue und gibt es an Odin zurück

## Service-Interfaces

### Inputs
- `ThorAction` von Odin
- Resource Requests
- **Coding Tasks**: Thor erkennt Coding-Aufgaben und leitet sie an Brünhild (Valkyries) weiter
- **Healthcare Tasks**: Thor erkennt Healthcare-Aufgaben und leitet sie an Frigg weiter

### Outputs
- `ThorResult` mit Status, Result, Error, Execution Time
- **Coding Results**: Thor erhält Ergebnisse von Brünhild und gibt sie an Odin zurück
- **Healthcare Results**: Thor erhält Ergebnisse von Frigg und gibt sie an Odin zurück

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

### CODING_TASK
- **Coding-Aufgaben**: Thor erkennt Coding-Aufgaben und leitet sie an Brünhild (Valkyries) weiter
- **Erkennung**: Thor erkennt automatisch, ob es sich um eine Coding-Aufgabe handelt
- **User kann explizit angeben**: User kann auch explizit angeben, dass es eine Coding-Aufgabe ist (macht Erkennung einfacher)
- **Weiterleitung**: Thor legt Task in Queue für Brünhild
- **Ergebnis**: Thor erhält Ergebnis von Brünhild aus Queue und gibt es an Odin zurück

### HEALTHCARE_TASK
- **Healthcare-Aufgaben**: Thor erkennt Healthcare-Aufgaben und leitet sie an Frigg weiter
- **Erkennung**: Thor erkennt automatisch, ob es sich um eine Healthcare-Aufgabe handelt
- **User kann explizit anfordern**: User kann auch explizit Frigg anfordern oder eine Behandlung starten wollen
- **Weiterleitung**: Thor legt Task in Queue für Frigg
- **Ergebnis**: Thor erhält Ergebnis von Frigg aus Queue und gibt es an Odin zurück

## Workflow

1. **Action empfangen**
   - Odin sendet `ThorAction`
   - Thor validiert Action

2. **Task-Typ Erkennung**
   - **Thor erkennt Coding-Aufgaben**: Thor prüft, ob es sich um eine Coding-Aufgabe handelt
   - **User kann explizit angeben**: User kann explizit angeben, dass es eine Coding-Aufgabe ist
   - **Weiterleitung an Brünhild**: Falls Coding-Aufgabe, legt Thor Task in Queue für Brünhild (Valkyries)
   - **Thor erkennt Healthcare-Aufgaben**: Thor prüft, ob es sich um eine Healthcare-Aufgabe handelt
   - **User kann explizit anfordern**: User kann explizit Frigg anfordern oder eine Behandlung starten wollen
   - **Weiterleitung an Frigg**: Falls Healthcare-Aufgabe, legt Thor Task in Queue für Frigg
   - **Normale Actions**: Falls keine spezielle Aufgabe, normaler Workflow

3. **Resource Check** (für normale Actions)
   - Prüft verfügbare Ressourcen
   - Allokiert Ressourcen falls verfügbar
   - Wartet falls Ressourcen knapp

4. **Action Scheduling** (für normale Actions)
   - Fügt Action zu Queue hinzu
   - Plant Execution basierend auf Priorität
   - Wartet auf Execution-Slot

5. **Action Execution**
   - **Normale Actions**: Führt Action aus (via Mjölnir)
   - **Coding Tasks**: Wartet auf Ergebnis von Brünhild
   - **Healthcare Tasks**: Wartet auf Ergebnis von Frigg
   - Überwacht Execution
   - Trackt Progress

6. **Result Processing**
   - **Normale Actions**: Sammelt Result, verarbeitet Errors, berechnet Execution Time
   - **Coding Tasks**: Erhält Ergebnis von Brünhild (nach Prüfung und Bestätigung, dass alle Aufgaben erledigt wurden)
   - **Healthcare Tasks**: Erhält Ergebnis von Frigg

7. **Response**
   - Sendet `ThorResult` zurück an Odin
   - Gibt Ressourcen frei

## Technische Anforderungen

### Security
- Sandboxing für unsichere Actions
- Permission Checking
- Input Validation
- Output Sanitization

### Performance
- Parallel Execution
- Async Processing
- Resource Pooling
- Caching

### Reliability
- **Retry-Mechanismen**: Retry mit Exponential Backoff
- **Timeout Handling**: Adaptive Timeouts mit Minimum/Maximum
- **Error Recovery**: Rollback wenn möglich, sonst Compensation
- **State Persistence**: State wird persistent gespeichert für Wiederherstellung
- **Replay-Mechanismus**: Actions können erneut ausgeführt werden
- **State-Synchronisation**: State kann von anderen Devices übernommen werden

### Monitoring
- Action Tracking
- Performance Metrics
- Error Logging
- Resource Usage Monitoring

## Datenschutz

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

- **Odin**: Für Actions
- **Heimdall**: Für Permissions
- **System APIs**: Für Action Execution
- **Resource Management**
- **Brünhild (Valkyries)**: Für Coding-Aufgaben (optional, wenn Valkyries installiert)
- **Frigg (Healthcare)**: Für Healthcare-Aufgaben (optional, wenn Frigg installiert)
- **Edda Core Library**: DTOs, Protocols, Utils

## Integration

- **Odin**: Empfängt `ThorAction` von Odin, sendet `ThorResult` zurück
- **Heimdall**: Für Permission-Checking
- **System APIs**: Für Action Execution
- **Brünhild (Valkyries)**: Queue-basierte Kommunikation für Coding-Aufgaben
- **Frigg**: Queue-basierte Kommunikation für Healthcare-Aufgaben
- **Asgard/Yggdrasil**: Für Distributed Locking (bei geteilten Resources)

## Parallel Action Conflict Resolution

### Problem: Race Conditions

**Gleichzeitiger Zugriff**
- Zwei oder mehr Devices versuchen gleichzeitig auf dieselbe Resource zuzugreifen
- Actions können sich gegenseitig beeinflussen oder überschreiben
- Dateninkonsistenz: Ohne Konfliktlösung können Daten inkonsistent werden

### Lösung: Priority + Locking

**Kombination aus Priorität und Locking**
- **Priority-basiert**: Höhere Priorität gewinnt bei Konflikten
- **Locking**: Erstes Device bekommt Lock, zweites wartet
- **Kombination**: Priority bestimmt, wer Lock bekommt, Locking verhindert gleichzeitigen Zugriff

### Locking-Mechanik

**Hybrid-Ansatz: Lokal + Distributed**

**Lokal für lokale Resources**
- **Local Locking**: Jedes Device verwaltet eigene Locks für lokale Resources
- **Beispiele**: Lokale Dateien, lokale Anwendungen, lokale System-Settings
- **Schnell**: Keine Netzwerk-Latenz
- **Einfach**: Keine Koordination nötig

**Distributed für geteilte Resources**
- **Distributed Locking**: Locking über Asgard/Yggdrasil für geteilte Resources
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

### Priorisierung von Actions

**Kombination: System-Priority mit User-Override**

**System-Priority (Standard)**
- **System bestimmt**: System bestimmt automatisch Priorität basierend auf:
  - Action-Typ (kritische Actions haben höhere Priorität)
  - Resource-Typ (wichtige Resources haben höhere Priorität)
  - Device-Typ (Server hat höhere Priorität als Mobile)
  - Zeitpunkt (ältere Requests haben höhere Priorität)
- **Fairness**: System sorgt für faire Verteilung

**User-Override**
- **User kann Priorität setzen**: User kann explizit Priorität für Actions setzen
- **Höhere Priorität**: User kann Actions höhere Priorität geben
- **Niedrigere Priorität**: User kann Actions niedrigere Priorität geben
- **Use Cases**: Wichtige Tasks, Eilige Tasks, Hintergrund-Tasks

**Priority-Levels**
- **CRITICAL**: Kritische Actions (z.B. System-Updates, Sicherheits-Operations)
- **HIGH**: Wichtige Actions (z.B. User-Commands, wichtige Datei-Operations)
- **NORMAL**: Normale Actions (Standard-Priorität)
- **LOW**: Niedrige Priorität (z.B. Hintergrund-Tasks, Wartungs-Operations)
- **BACKGROUND**: Hintergrund-Priorität (niedrigste Priorität)

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
- **Priorität-basiert**: Device mit niedrigster Priorität gibt Lock frei

## Error Recovery und Resilience

### Netzwerk-Fehlerbehandlung

**Kombination: Retry → Fallback → Fehler**

**1. Automatischer Retry mit Exponential Backoff**
- **Erster Versuch**: Sofortiger Retry bei Netzwerk-Fehler
- **Exponential Backoff**: Bei wiederholten Fehlern wird Wartezeit exponentiell erhöht
- **Maximale Retries**: Maximale Anzahl von Retries (z.B. 3-5 Versuche)
- **Timeout**: Retry-Versuche haben Timeout

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

- Sollte Plugin-Architektur für verschiedene Action-Types haben
- Muss Sandboxing für unsichere Actions unterstützen
- Sollte parallele Execution mit Resource-Limits haben
- Muss Timeout-Mechanismen haben
- Sollte Action-History für Debugging speichern
- Muss Integration mit System-Security haben
- Muss Coding-Aufgaben erkennen können (automatisch + explizit)
- Muss Healthcare-Aufgaben erkennen können (automatisch + explizit)
- Muss Queue-System für Brünhild und Frigg haben
- Muss prüfen, ob Valkyries/Frigg verfügbar sind
- Muss Task-Queue-Management haben (Enqueue, Dequeue, Status-Tracking)
- **Muss Hybrid-Locking unterstützen**: Lokal für lokale Resources, Distributed für geteilte Resources
- **Muss Priority-basiertes Locking haben**: System-Priority mit User-Override
- **Muss Pessimistic Locking + Transactions kombinieren**: Verhindert Race Conditions
- **Muss Deadlock-Detection haben**: Erkennt und löst Deadlocks auf
- **Muss Timeout-basierte Lock-Expiration haben**: Automatische Freigabe nach Timeout
- **Muss Retry-Mechanismus mit Exponential Backoff haben**: Für Netzwerk-Fehler
- **Muss Fallback-Mechanismen haben**: Alternative Routen/Provider bei Fehlern
- **Muss Rollback-Mechanismus haben**: Wenn möglich, bei partiellen Failures
- **Muss Compensation-Mechanismus haben**: Wenn Rollback nicht möglich
- **Muss Replay-Mechanismus haben**: Für Daten-Wiederherstellung
- **Muss State-Synchronisation haben**: Als Fallback für Daten-Wiederherstellung
- **Muss adaptive Timeouts haben**: Mit Minimum/Maximum für Timeout-Handling

