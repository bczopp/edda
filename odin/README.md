# Odin - Main Process Service

## Übersicht

Odin ist der zentrale Orchestrator auf Midgard (Desktop), Alfheim (Mobile) und Asgard (Homeserver). Er koordiniert alle anderen Services und ist der Hauptprozess des Edda-Systems. Odin läuft NICHT auf Jötnar (IoT-Devices), da diese zu klein sind und ein spezielles Toolcalling-Protokoll verwenden.

## Verantwortlichkeiten

### 1. User-Command Processing
- **Text-Input**: Empfängt Text-Input direkt vom Frontend
- **Voice-Input**: Empfängt Voice-Input von Huginn (STT) - wird zu Text transkribiert
- **Einheitliche Verarbeitung**: Beide Input-Methoden werden als Text-Commands verarbeitet
- Analysiert und interpretiert User-Commands
- Entscheidet über erforderliche Actions

### 2. Action Planning & Decision Making
- Plant die Ausführung von Actions basierend auf User-Commands
- Entscheidet, welche Services benötigt werden
- Koordiniert den Workflow zwischen Services

### 3. Device State Management
- Verwaltet den aktuellen Zustand des Devices
- Trackt laufende Actions und Tasks
- Verwaltet Device-Konfiguration

### 4. Inter-Device Communication Coordination
- Koordiniert Kommunikation mit anderen Devices über Bifrost
- Verwaltet Device-Verbindungen
- Routet Messages zwischen Devices

## Service-Interfaces

### Inputs
- `RavenMessage` (von Frontend oder Huginn) - User Text/Voice Input
  - **Text-Input**: Direkt vom Frontend als Text
  - **Voice-Input**: Von Huginn (STT) transkribiert zu Text
- `ThorResult` (von Thor) - Action Execution Results
- `BifrostConnection` (von Bifrost) - Inter-Device Messages

### Outputs
- `RavenMessage` (an Muninn) - System Responses
- `ThorAction` (an Thor) - Actions zur Ausführung (inkl. Coding-Aufgaben)
- `WolfRequest` (an Freki/Geri) - AI Service Requests

**Wichtig**: Odin ruft Valkyries NICHT direkt auf. Odin arbeitet mit Thor, und Thor erkennt Coding-Aufgaben und leitet sie an Brünhild (Valkyries) weiter.

**Wichtig**: Odin ruft Frigg NICHT direkt auf. Odin arbeitet mit Thor, und Thor erkennt Healthcare-Aufgaben und leitet sie an Frigg weiter.

## Workflow

1. **User Input empfangen**
   - **Text-Input**: Frontend sendet `RavenMessage` direkt mit Text
   - **ODER Voice-Input**: Huginn sendet `RavenMessage` mit transkribiertem Text (aus Voice-Input)
   - Odin analysiert den Command (unabhängig von Input-Methode)

2. **Context Enrichment (optional)**
   - Falls nötig: Request an Freki (RAG) für Kontext-Anreicherung
   - Warten auf RAG-Response

3. **LLM Processing**
   - Request an Geri (LLM) mit angereichertem Prompt
   - Warten auf LLM-Response

4. **Action Planning**
   - Odin interpretiert LLM-Response
   - Erstellt `ThorAction` für erforderliche Actions

5. **Action Execution**
   - Sendet `ThorAction` an Thor
   - Wartet auf `ThorResult`

6. **Response Generation**
   - Erstellt Response basierend auf Action-Result
   - Sendet `RavenMessage` an Muninn für TTS

## Abhängigkeiten

- **Frontend**: Für Text-Input (optional, wenn Frontend vorhanden)
- **Huginn/Muninn**: STT/TTS Service (für Voice-Input und TTS-Output)
- **Freki**: RAG Service
- **Geri**: LLM Service
- **Thor**: Action Executor
- **Bifrost**: Communication Service
- **Heimdall**: Security Service
- **Edda Core Library**: DTOs, Protocols, Utils

## Technische Anforderungen

- Event-driven Architecture
- Asynchrone Message Processing
- State Management
- Error Handling & Recovery
- Logging & Monitoring

## Integration

- **Midgard**: Läuft als Hauptprozess auf Desktop/Laptop
- **Alfheim**: Läuft als Hauptprozess auf Mobile
- **Asgard**: Läuft als Hauptprozess auf Homeserver
- **Jötnar**: Kein Odin (zu klein, nutzt spezielles Toolcalling-Protocol)
- **Services**: Koordiniert alle Services (Huginn/Muninn, Freki, Geri, Thor, Bifrost, Heimdall)
- **Valkyries**: Indirekt über Thor (Thor erkennt Coding-Aufgaben)
- **Frigg**: Indirekt über Thor (Thor erkennt Healthcare-Aufgaben)

## Performance

### Performance-Optimierungen
- **Event-driven Architecture**: Asynchrone Event-Verarbeitung für bessere Performance
- **Parallel Processing**: Parallele Verarbeitung von mehreren Requests
- **Caching**: Intelligentes Caching für häufig verwendete Daten und Responses
- **Connection Pooling**: Effizientes Connection-Pooling für Service-Kommunikation
- **State Management**: Optimiertes State-Management für schnellen Zugriff
- **Resource Management**: Intelligentes Resource-Management für optimale Performance

### Performance-Metriken
- Niedrige Latenz für Command-Processing (< 100ms für einfache Commands)
- Hoher Durchsatz für parallele Requests
- Effiziente Service-Koordination (minimaler Overhead)

## Datenschutz

### Datenschutz-Features
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, wo möglich
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten ohne Zustimmung
- **Datenverschlüsselung**: Sensible Daten werden verschlüsselt gespeichert
- **User Control**: User hat volle Kontrolle über seine Daten

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Security-Features
- **Secure State Storage**: Verschlüsselte Speicherung von Device-State
- **Authentication Integration**: Integration mit Heimdall für sichere Authentication
- **Permission Checking**: Prüfung von Permissions für alle Actions
- **Input Validation**: Umfassende Validierung aller Inputs
- **Secure Communication**: Sichere Kommunikation mit allen Services
- **Audit Logging**: Logging aller Security-relevanten Events

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Fail-Safe**: Bei Fehlern wird sicherheitshalber verweigert (Deny statt Allow)

## Provider Selection

### Automatische Provider-Auswahl durch Odin

**User gibt Settings an**: Anspruch/Requirements in den Settings
**Odin wählt Provider**: Automatisch basierend auf Settings und aktuellen Provider-Angeboten
**Optimale Auswahl**: Odin berücksichtigt Preis, Qualität, Latency, Verfügbarkeit
**Transparenz**: User kann sehen, welcher Provider gewählt wurde (Marketplace als Übersicht)

### Zahlungsmethode erforderlich

**Als Consumer (Requester)**: Gültige Zahlungsmethode muss hinterlegt sein
- Für Bezahlung von Compute-Requests
- Automatische Abrechnung pro Request
- Pre-Authorization für schnelle Requests

**Als Provider**: Gültige Zahlungsmethode muss hinterlegt sein
- Für Auszahlung von Earnings
- Automatische Auszahlung (täglich/wöchentlich/monatlich)
- Steuerliche Dokumentation

**Beide Rollen**: User können gleichzeitig Consumer und Provider sein
- Separate Zahlungsmethoden möglich (aber nicht nötig)
- Netting möglich (Earnings gegen Costs)

### Marketplace als Übersicht

**Transparenz-Tool**: Zeigt alle verfügbaren Provider
**Vergleich**: Provider können verglichen werden
**Statistiken**: Provider-Statistiken (Qualität, Preis, Verfügbarkeit)
**Nicht für direkte Auswahl**: Direkte Provider-Auswahl ist möglich, aber nicht empfohlen

### User Settings

**Anspruch/Requirements (User konfiguriert)**
- **Quality Level**: Low (günstigste Option), Medium (Balance), High (beste Qualität), Custom (spezifische Anforderungen)
- **Max Cost**: Maximale Kosten pro Request
- **Max Latency**: Maximale Antwortzeit
- **Model Preferences**: Bevorzugte Modelle (optional)
- **Provider Preferences**: Bevorzugte Provider (optional, nicht empfohlen)

**Default Settings**
- **Empfohlene Einstellung**: "Medium" - Balance zwischen Preis und Qualität
- **Odin wählt optimal**: Basierend auf aktuellen Provider-Angeboten

### Odin's Provider Selection Algorithm

**1. Request Analysis**
- Anforderung analysieren: Was wird benötigt?
- Context verstehen: Komplexität, Token-Schätzung
- Requirements extrahieren: Aus User-Settings

**2. Provider Matching**
- Provider filtern: Basierend auf Verfügbarkeit und Capabilities
- Angebote vergleichen: Preis, Qualität, Latency
- Fair Distribution: Berücksichtigung von Fairness-Score

**3. Optimal Selection**
- Score berechnen: Für jeden passenden Provider
- Besten Provider wählen: Höchster Score
- Fallback vorbereiten: Alternative Provider falls nötig

**4. Execution**
- Request an Provider senden: Gewählter Provider
- Monitoring: Überwachung der Ausführung
- Fallback: Bei Fehler automatisch zu alternativem Provider

**5. Error Handling & Fallback**

**Kein passender Provider gefunden**
- **Fallback-Strategie**: Kombination aus mehreren Schritten
  1. **Lokales LLM**: Default ist lokales LLM, das jedes Device (außer Jötnar) mitliefern muss
  2. **Queue**: Request wird in Queue gelegt, später erneut versucht
  3. **Fehlermeldung**: Falls alles fehlschlägt, Fehlermeldung an User
- **Provider-Modelle**: Provider-Modelle können nur abgerufen werden, wenn User bei Yggdrasil registriert ist und mit Device/Netzwerk aktiv verbunden ist
- **Explizite Konfiguration**: Verbindung zu OpenAI etc. muss explizit in Konfiguration angegeben werden

**Provider-Ausfall während Ausführung**
- **Automatischer Retry**: Automatischer Retry mit anderem Provider (basierend auf Score)
- **Keine Entschädigung**: Ausgefallener Provider erhält keine Entschädigung für fehlgeschlagenen Request
- **User zahlt nicht**: User muss für fehlgeschlagenen Request nicht zahlen
- **Zahlung erst nach Erfolg**: Zahlung erfolgt erst nach erfolgreicher Antwort
- **Fallback zu lokalem LLM**: Wenn alle Provider ausfallen, Fallback zu lokalem LLM (kostenlos)

**Provider-Timeout**
- **Timeout-Handling**: Bei Timeout wird Request als fehlgeschlagen markiert
- **Automatischer Retry**: Retry mit anderem Provider
- **Fallback**: Falls alle Provider timeout, Fallback zu lokalem LLM

**Quality Metrics Update**
- **Kombination**: Sofort nach Request, aber auch periodische Aggregation
- **Sofort-Update**: Quality-Metriken werden sofort nach jedem Request aktualisiert
- **Periodische Aggregation**: Zusätzlich periodische Aggregation für langfristige Trends
- **Fair Distribution**: Quality-Metriken werden für Fair Distribution verwendet

### Scoring Algorithm

**Faktoren**
- **Preis** (30% Gewichtung): Niedriger Preis = höherer Score
- **Qualität** (25% Gewichtung): Höhere Qualität = höherer Score (basierend auf Quality-Metriken)
- **Latency** (20% Gewichtung): Niedrigere Latency = höherer Score
- **Verfügbarkeit** (15% Gewichtung): Höhere Verfügbarkeit = höherer Score
- **Fairness** (10% Gewichtung): Fair Distribution Score

**Quality-Metriken**: Quality-Metriken werden nach jedem Request gemessen (automatisch + optionales User-Feedback) und periodisch aggregiert (gewichteter Durchschnitt, neuere Requests haben höheres Gewicht). Quality wird als Faktor im Scoring-Algorithmus verwendet, und optional kann ein Minimum-Quality-Filter angewendet werden.

**User Settings Integration**
- **Quality Level "Low"**: Preis-Gewichtung erhöht (50%)
- **Quality Level "High"**: Qualität-Gewichtung erhöht (50%)
- **Quality Level "Medium"**: Ausgewogene Gewichtung
- **Max Cost**: Filtert Provider über Max Cost hinaus
- **Max Latency**: Filtert Provider über Max Latency hinaus

## Marketplace Quality Metrics

### Quality-Metric-Messung

**Kombination: Nach jedem Request + periodische Tests**

**Nach jedem Request**
- **Response-Qualität wird bewertet**: Nach jedem Request wird Response-Qualität automatisch bewertet
- **Sofortige Bewertung**: Bewertung erfolgt sofort nach Request-Abschluss
- **Kontinuierlich**: Quality-Metriken werden kontinuierlich aktualisiert

**Periodische Tests**
- **Regelmäßige Tests**: Provider werden regelmäßig getestet (z.B. täglich/wöchentlich)
- **Standardisierte Tests**: Standardisierte Test-Requests werden gesendet
- **Baseline**: Erstellt Baseline für Quality-Metriken
- **Ergänzend**: Ergänzt kontinuierliche Bewertung

### Quality-Metric-Aggregation

**Gewichteter Durchschnitt (neuere Requests haben höheres Gewicht)**

**Gewichtung basierend auf Zeit**
- **Neuere Requests**: Neuere Requests haben höheres Gewicht
- **Zeit-Decay**: Ältere Requests haben exponentiell abnehmendes Gewicht
- **Aktuell**: Aktuelle Performance wird stärker gewichtet

**Formel**
```
weighted_average = Σ(quality_score_i * weight_i) / Σ(weight_i)
weight_i = e^(-decay_rate * age_i)
```

### Quality-Bewertung nach jedem Request

**Kombination: Automatisch + optionales User-Feedback**

**Automatische Bewertung**
- **System bewertet**: System bewertet Response-Qualität automatisch
- **Metriken**: Verschiedene Metriken werden gemessen:
  - Antwortzeit (Latency)
  - Korrektheit (Accuracy)
  - Vollständigkeit (Completeness)
  - Konsistenz (Consistency)
- **Kontinuierlich**: Bewertung erfolgt bei jedem Request

**Optionales User-Feedback**
- **User kann Feedback abgeben**: User kann optional Feedback abgeben
- **Sofern abgegeben**: User-Feedback wird nur einbezogen, wenn User es abgibt
- **Nicht verpflichtend**: User muss kein Feedback abgeben
- **Zusätzliche Information**: User-Feedback ergänzt automatische Bewertung

### Quality-Metric-Updates

**Kombination: Sofort für wichtige Updates, Batch für Aggregation**

**Sofort für wichtige Updates**
- **Wichtige Änderungen**: Bei wichtigen Quality-Änderungen wird Metrik sofort aktualisiert
- **Signifikante Änderungen**: Bei signifikanten Änderungen (z.B. Quality-Drop) sofortige Aktualisierung
- **Alerts**: Bei wichtigen Änderungen werden Alerts gesendet

**Batch für Aggregation**
- **Periodische Aggregation**: Quality-Metriken werden periodisch aggregiert (z.B. stündlich/täglich)
- **Gewichteter Durchschnitt**: Gewichteter Durchschnitt wird berechnet
- **Effizient**: Reduziert Rechenaufwand

### Verwendung für Fair Distribution

**Kombination: Als Faktor + optionaler Filter**

**Als Faktor im Scoring-Algorithmus**
- **Quality-Gewichtung**: Quality-Metrik wird als Faktor im Scoring-Algorithmus verwendet
- **Höhere Quality = höherer Score**: Provider mit höherer Quality erhalten höheren Score
- **Standard**: Quality ist immer Teil des Scores

**Optionaler Filter**
- **Minimum-Quality**: User kann Minimum-Quality setzen
- **Filter**: Nur Provider über Minimum-Quality werden berücksichtigt
- **Optional**: Filter ist optional, nicht verpflichtend

## State Synchronisation

### State-Arten

**Alle State-Arten werden synchronisiert**
- **Device-State**: Status des Devices (online/offline, laufende Tasks, aktive Verbindungen)
- **Konfiguration-State**: User-Einstellungen, Präferenzen, Model-Auswahl
- **Action-State**: Status laufender Actions (welche Actions laufen, Fortschritt)
- **Network-State**: Verbundene Devices, Network-Topologie, Capabilities
- **Session-State**: Aktive Sessions, Token-Status, Authentifizierungs-Status

### State-Synchronisation

**Push-basiert (Sofortige Synchronisation)**

**Device sendet State-Updates sofort**
- **Bei Änderung**: Wenn State sich ändert, wird Update sofort gesendet
- **An alle verbundenen Devices**: State-Update wird an alle verbundenen Devices gesendet
- **Schnell**: Minimale Verzögerung
- **Konsistent**: Alle Devices haben aktuellen State

### Conflict Resolution

**Kombination: Timestamp + Priority**

**Timestamp-basiert**
- **Neuester State gewinnt**: State mit neuestem Timestamp gewinnt bei Konflikten
- **Chronologische Ordnung**: Garantiert chronologische Konsistenz
- **Standard**: Timestamp ist primäres Kriterium

**Priority-basiert**
- **Höhere Priorität gewinnt**: Bei gleichem Timestamp gewinnt höhere Priorität
- **Device-Priority**: Bestimmte Devices haben höhere Priorität (z.B. Asgard > Midgard > Alfheim)
- **User-Priority**: User kann Priorität für bestimmte State-Updates setzen

### State-Persistenz

**Lokal auf jedem Device**

**Jedes Device speichert eigenen State**
- **Lokale Speicherung**: Jedes Device speichert State lokal (SQLite, etc.)
- **Unabhängig**: Jedes Device ist unabhängig
- **Schnell**: Keine Netzwerk-Latenz für lokale Zugriffe
- **Robust**: Funktioniert auch ohne Netzwerk

**Synchronisation**
- **Bei Verbindung**: State wird bei Verbindung synchronisiert
- **Bei Änderung**: State wird bei Änderung synchronisiert
- **Konsistenz**: Alle Devices haben konsistenten State

### State-Update-Propagation

**Selective (nur an relevante Devices)**

**Nur relevante Devices erhalten Updates**
- **Selective Propagation**: State-Updates werden nur an relevante Devices gesendet
- **Relevanz-Prüfung**: System prüft, welche Devices den State benötigen
- **Effizient**: Reduziert unnötige Netzwerk-Traffic
- **Jötnar-Devices ausgeschlossen**: Jötnar-Devices erhalten State-Updates nicht (brauchen sie nicht)

## Implementierungs-Notizen

- Sollte als zentraler Event-Bus fungieren
- Muss thread-safe sein für parallele Requests
- Sollte Retry-Mechanismen für fehlgeschlagene Actions haben
- Muss Device-State persistent speichern können
- Muss alle Services koordinieren können
- Sollte robustes Error-Handling haben
- **Muss Provider Selection Service haben**: Teil von Odin
- **Muss Settings Management haben**: Verwaltung von User-Settings
- **Muss Scoring Engine haben**: Berechnung von Provider-Scores
- **Muss Selection Logic haben**: Auswahl-Logik für Provider
- **Muss Quality-Messung nach jedem Request haben**: Für Provider-Quality-Metriken
- **Muss periodische Tests haben**: Für Quality-Metriken
- **Muss gewichteten Durchschnitt haben**: Neuere Requests höheres Gewicht
- **Muss automatische Bewertung haben**: Für Quality-Metriken
- **Muss optionales User-Feedback haben**: Sofern abgegeben
- **Muss sofortige Updates für wichtige Änderungen haben**: Quality-Metriken
- **Muss Batch-Aggregation haben**: Für Quality-Metriken
- **Muss Quality als Faktor im Scoring-Algorithmus haben**: Für Provider-Auswahl
- **Muss optionalen Quality-Filter haben**: Für User-Präferenzen
- **Muss Push-basierte State-Synchronisation haben**: Für Device-State
- **Muss Timestamp + Priority-basierte Conflict Resolution haben**: Für State-Konflikte
- **Muss lokale State-Persistenz haben**: Für Device-State
- **Muss Selective Propagation haben**: Nur an relevante Devices, nicht an Jötnar-Devices
- **Performance**: Muss optimiert sein für schnelle Command-Processing und Service-Koordination
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für State-Management

