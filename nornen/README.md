# Nornen (Urd, Verdandi) - Decision Service

## Übersicht

Nornen ist der Decision Service bei Yggdrasil, der Entscheidungen über eingehende/ausgehende Requests trifft, Provider-Registrierungen verwaltet und die Admin API bereitstellt.

**Mythologische Bedeutung**: Die Nornen sind die Schicksalsgöttinnen. Urd (Vergangenheit), Verdandi (Gegenwart). **Hinweis**: Skuld (Zukunft) ist ein separater Service, der auf allen Devices installiert werden muss.

**Programmiersprache**: Rust

**Wichtig**: Nornen ist nur bei Yggdrasil verfügbar. Skuld ist ein separater Service, der auf allen Devices installiert werden muss.

## Verantwortlichkeiten

### 1. Entscheidungen über Requests
- **Request-Entscheidungen**: Entscheidet über eingehende/ausgehende Requests
- **Request-Validation**: Validiert Requests basierend auf Business-Logik
- **Request-Routing**: Entscheidet über Request-Routing
- **Request-Priorisierung**: Priorisiert Requests basierend auf verschiedenen Faktoren

### 2. Provider-Registrierung
- **Provider-Genehmigung**: Genehmigt oder lehnt Provider-Registrierungen ab
- **Provider-Verwaltung**: Verwaltet Provider-Registrierungen
- **Provider-Validation**: Validiert Provider-Capabilities und Requirements
- **Provider-Monitoring**: Überwacht Provider-Performance

### 3. User-Konfiguration
- **Konfiguration-Speicherung**: Speichert User-Konfiguration für Marketplace
- **Konfiguration-Verwaltung**: Verwaltet User-Konfigurationen
- **Konfiguration-Validation**: Validiert User-Konfigurationen
- **Konfiguration-Synchronisation**: Synchronisiert Konfigurationen zwischen Devices

### 4. Admin API
- **Health Check**: Health-Check-Endpoints für Monitoring
- **Dashboard-Informationen**: Bereitstellung von Dashboard-Daten
- **Monitoring-Daten**: Bereitstellung von Monitoring-Daten
- **Admin-Informationen**: Alle Informationen, die Admins benötigen

### 5. Analytics (Urd & Verdandi)
- **Urd (Vergangenheit)**: Historie, Request-History, historische Statistiken
- **Verdandi (Gegenwart)**: Aktuelle Statistiken, Real-time Analytics, Live-Metriken

## Service-Interfaces

### Inputs
- `DecisionRequest` (von Nidhöggr) - Requests für Entscheidungen
  - Provider-Registration-Requests
  - Request-Validation-Requests
  - Configuration-Requests
  - Admin-Requests

- `AnalyticsRequest` (von Yggdrasil) - Requests für Analytics
  - Provider-Analytics-Requests
  - Requester-Analytics-Requests
  - Historical-Data-Requests

### Outputs
- `DecisionResponse` (an Nidhöggr) - Entscheidungen und Responses
- `AnalyticsResponse` (an Yggdrasil) - Analytics-Daten und Statistiken

## Workflow

### Provider-Registrierung

1. **Provider-Registration-Request**
   - User sendet Provider-Registration-Request über Vedrfolnir
   - Nidhöggr leitet Request an Nornen weiter

2. **Nornen validiert Request**
   - Urd analysiert Provider-Historie (falls vorhanden)
   - Verdandi prüft aktuelle Provider-Requirements
   - Provider-Capabilities werden validiert
   - Provider-Requirements werden geprüft

3. **Entscheidung**
   - Nornen entscheidet über Genehmigung/Ablehnung
   - Bei Genehmigung: Provider wird registriert
   - Bei Ablehnung: Begründung wird zurückgegeben

4. **Response**
   - Nornen sendet Decision-Response an Nidhöggr
   - Nidhöggr sendet Response über Ratatoskr-Protocol zurück

### Request-Entscheidungen

1. **Request kommt an**
   - Request kommt über Nidhöggr an
   - Nidhöggr leitet Request an Nornen weiter

2. **Nornen analysiert Request**
   - Urd analysiert Request-Historie
   - Verdandi prüft aktuelle Request-Requirements
   - Request wird validiert

3. **Entscheidung**
   - Nornen entscheidet über Request-Behandlung
   - Request-Routing wird bestimmt
   - Request-Priorisierung wird festgelegt

4. **Response**
   - Nornen sendet Decision-Response
   - Request wird entsprechend behandelt

### Admin API

1. **Admin-Request**
   - Admin sendet Request an Admin-API
   - Request kommt über Nidhöggr an

2. **Nornen verarbeitet Request**
   - Urd liefert historische Daten
   - Verdandi liefert aktuelle Daten
   - Dashboard-Informationen werden zusammengestellt

3. **Response**
   - Nornen sendet Response mit Admin-Informationen
   - Dashboard-Daten werden zurückgegeben

## Der Brunnen (Mímisbrunnr)

**Mythologische Bedeutung**: Der Brunnen Mímisbrunnr ist die Quelle der Weisheit. In diesem Kontext ist der Brunnen die Datenbank, die von Mimir verwaltet wird.

**Datenbank-Verwaltung:**
- **Mimir verwaltet Datenbank**: Mimir (Privacy Database Service) verwaltet die Datenbank
- **Nornen nutzen Datenbank**: Nornen nutzen die Datenbank für Entscheidungen und Analytics
- **Datenbank-Zugriff**: Nornen greifen über Mimir auf die Datenbank zu

## Analytics-Features

### Provider Analytics
- **Request Statistics**: Anzahl von Requests pro Provider
- **Earnings**: Verdienst-Statistiken pro Provider
- **Quality Metrics**: Quality-Metriken pro Provider
- **Usage Patterns**: Nutzungsmuster pro Provider

### Requester Analytics
- **Request History**: Request-Historie pro Requester
- **Cost Analysis**: Kosten-Analyse pro Requester
- **Quality Metrics**: Quality-Metriken der verwendeten Provider
- **Usage Patterns**: Nutzungsmuster pro Requester

### Aggregation
- **Zeitbasierte Aggregation**: Aggregation von Daten über Zeiträume
- **Trend-Analyse**: Erkennung von Trends und Mustern
- **Predictive Analytics**: Vorhersage von Trends (durch Skuld, wenn verfügbar)

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- **Mimir**: Privacy Database Service (für Datenbank-Zugriff)
- **Yggdrasil (Elixir)**: Hauptprozess, koordiniert alle Services

## Integration

- **Nidhöggr**: Empfängt Requests von Nidhöggr
- **Mimir**: Nutzt Mimir für Datenbank-Zugriff
- **Yggdrasil (Elixir)**: Hauptprozess, koordiniert alle Services
- **Heidrun**: Für Payment-Informationen (falls nötig)
- **Eikthyrnir**: Für Quality-Metriken (falls nötig)

## Performance

### Performance-Optimierungen
- **Caching**: Intelligentes Caching für häufig abgerufene Daten
- **Async Processing**: Asynchrone Verarbeitung von Requests
- **Database-Optimization**: Optimierte Datenbankabfragen über Mimir
- **Batch-Processing**: Batch-Verarbeitung für Analytics-Requests

### Performance-Metriken
- Schnelle Entscheidungen (< 100ms für Standard-Entscheidungen)
- Effiziente Analytics-Abfragen (< 200ms für Standard-Analytics)
- Hoher Durchsatz (100+ Requests/Sekunde pro Instanz)

## Implementierungs-Notizen

- **Programmiersprache**: Rust
- **Nur bei Yggdrasil**: Nornen ist nur bei Yggdrasil verfügbar
- **Skuld ist separater Service**: Skuld ist nicht Teil von Nornen
- **Mimir-Integration**: Nutzt Mimir für Datenbank-Zugriff
- **gRPC-Kommunikation**: Kommuniziert mit anderen Services über gRPC
- **Admin API**: RESTful API für Admin-Zugriff
- **Performance**: Optimiert für schnelle Entscheidungen und Analytics

