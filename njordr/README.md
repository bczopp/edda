# Njörðr - Marketplace Service

## Übersicht

**Tests ausführen:** Von `njordr/`: `docker compose -f docker-compose.test.yml run --rm njordr-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `njordr/**` läuft die Pipeline [.github/workflows/njordr.yml](../.github/workflows/njordr.yml) (Test im Container, Lint).

Njörðr ist ein Rust-Microservice für Yggdrasil, der den Distributed Computing Marketplace verwaltet. Er koordiniert Provider-Management, Request-Routing, Transaction-Management und Marketplace-Operationen.

**Mythologische Bedeutung**: Njörðr ist der Gott des Handels, der Seefahrt und des Wohlstands (Vanir). Er ist der Vater von Freyr und Freyja und ursprünglich aus Vanaheim.

**Programmiersprache**: Rust

## Verantwortlichkeiten

### 1. Provider Management
- **Provider Registration**: 
  - **Registration-Prozess**: Provider-Registration-Prozess (Request, Validation, Approval)
  - **Registration-Validierung**: Registration-Validierung (Capabilities, Requirements, Quality)
  - **Registration-Fehler**: Bei Registration-Fehlern wird Fehler zurückgegeben, Provider kann korrigieren
  - Verwaltung von Provider-Registrierungen
- **Provider Configuration**: 
  - **Configuration-Verwaltung**: Provider-Konfigurationen werden verwaltet (Pricing, Models, Capacity)
  - **Configuration-Validierung**: Configuration-Validierung (Schema-Validierung, Policy-Checks)
  - **Configuration-Updates**: Bei Configuration-Updates wird automatisch Migration durchgeführt
  - Verwaltung von Provider-Konfigurationen (Pricing, Models, Capacity)
- **Provider Discovery**: 
  - **Discovery-Algorithmen**: Provider-Discovery-Algorithmen (Capability-Matching, Quality-Filtering)
  - **Discovery-Fehler**: Bei Discovery-Fehlern wird Fallback zu bekannten Providern verwendet
  - Discovery und Matching von Providern
- **Provider Status**: 
  - **Status-Tracking**: Provider-Status wird getrackt (Available, Busy, Offline, Maintenance)
  - **Status-Übergänge**: Status-Übergänge werden verwaltet (State-Machine)
  - **Status-Änderungen**: Bei Status-Änderungen wird automatisch Routing angepasst
  - Tracking von Provider-Status (Available, Busy, Offline)

### 2. Request Routing
- **Request Reception**: Empfang von Compute-Requests
- **Provider Matching**: Matching von Requests mit passenden Providern
- **Route Selection**: Auswahl des optimalen Providers basierend auf Fair Distribution Algorithm
- **Request Routing**: Routing von Requests an gewählte Provider
- **Load Balancing**: Lastverteilung über Provider

### 3. Transaction Management
- **Transaction Tracking**: Tracking aller Marketplace-Transactions
- **Transaction Lifecycle**: Verwaltung des Transaction-Lifecycles (PENDING, PROCESSING, COMPLETED, FAILED, CANCELLED, REFUNDED)
- **Transaction Coordination**: Koordination mit Heidrun für Settlement
- **Refund Handling**: Verwaltung von Rückerstattungen
- **Dispute Resolution**: Unterstützung bei Streitbeilegung

### 4. Marketplace Operations
- **Marketplace Coordination**: Koordination aller Marketplace-Komponenten
- **Provider Analytics**: Sammlung von Provider-Analytics
- **Requester Analytics**: Sammlung von Requester-Analytics
- **Marketplace Statistics**: Aggregation von Marketplace-Statistiken

## Kommunikation

### gRPC
- **Yggdrasil (Elixir) ↔ Njörðr (Rust)**: gRPC
- **Asynchron**: Yggdrasil sendet Marketplace-Requests, Njörðr antwortet mit Ergebnissen
- **Type-safe**: Protobuf garantiert korrekte Request/Response-Strukturen

### Service-Integration
- **Heidrun**: Njörðr koordiniert mit Heidrun für Pricing und Settlement
- **Eikthyrnir**: Njörðr nutzt Eikthyrnir für Quality-Assessment bei Provider-Auswahl
- **Nornen**: Njörðr koordiniert mit Nornen für Provider-Registration-Approval und Request-Decisions

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

### Njörðr-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Marketplace-Konfiguration
- Fair-Distribution-Algorithmus-Einstellungen
- Transaction-Management-Einstellungen

## Integration

### Yggdrasil-Integration
- **Marketplace Management**: Njörðr ist der zentrale Service für Marketplace-Management
- **Provider Management**: Njörðr verwaltet alle Provider-Registrierungen und -Konfigurationen
- **Request Routing**: Njörðr routet alle Compute-Requests an Provider
- **Transaction Management**: Njörðr verwaltet alle Marketplace-Transactions

### Service-Integration
- **Heidrun**: Pricing, Settlement, Pre-Authorization
- **Eikthyrnir**: Quality-Assessment für Provider-Auswahl
- **Nornen**: Provider-Registration-Approval, Request-Decisions, User-Configuration

## Fair Distribution Algorithm

### Algorithm Details
- **Fairness Score**: Score basierend auf bisheriger Nutzung
- **Round-Robin**: Rotation bei gleichen Bedingungen
- **Quality Weighting**: Gewichtung nach Quality-Metriken (von Eikthyrnir)
- **Cost Optimization**: Optimierung nach Kosten (von Heidrun)

### Scoring
- **Preis** (30% Gewichtung): Niedriger Preis = höherer Score
- **Qualität** (25% Gewichtung): Höhere Qualität = höherer Score (von Eikthyrnir)
- **Latency** (20% Gewichtung): Niedrigere Latency = höherer Score
- **Verfügbarkeit** (15% Gewichtung): Höhere Verfügbarkeit = höherer Score
- **Fairness** (10% Gewichtung): Fair Distribution Score

## Performance

### Performance-Optimierungen
- **Effizientes Routing**: Optimierte Routing-Algorithmen für Provider-Auswahl
- **Caching**: Caching von Provider-Informationen und Quality-Metriken
- **Batch-Processing**: Batch-Processing für Analytics-Aggregation
- **Minimaler Footprint**: Minimaler RAM- und CPU-Verbrauch

## Sicherheit

### Security-Features
- **Input Validation**: Validierung aller eingehenden Marketplace-Requests
- **Provider Verification**: Verifizierung von Provider-Registrierungen
- **Transaction Security**: Sichere Transaction-Verwaltung
- **Audit Logging**: Audit-Logging für alle Marketplace-Operationen
- **No Hardcoded Secrets**: Keine Hardcoded Secrets oder Keys

### Security-Best-Practices
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Access Control**: Zugriffskontrolle für Marketplace-Operationen
- **Data Integrity**: Sicherstellung der Datenintegrität bei allen Operationen

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden verarbeitet
- **Privacy-by-Design**: Privacy-by-Design bei allen Marketplace-Operationen
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Data Minimization bei allen Operationen

## Workflows

### Provider Registration and Approval

1. **User registriert Device als Provider**
   - Device-Capabilities werden übermittelt
   - Pricing wird konfiguriert
   - Availability wird gesetzt
   - Zahlungsmethode wird hinterlegt und verifiziert

2. **Njörðr testet Connection**
   - Test-Request wird gesendet
   - Connection Quality wird gemessen
   - Latency wird gemessen

3. **Nornen genehmigt Provider-Registrierung**
   - Njörðr sendet Provider-Registrierung an Nornen
   - Nornen prüft Provider-Konfiguration
   - Nornen genehmigt/lehnt ab
   - Bei Genehmigung: Njörðr registriert Provider

4. **Provider wird verfügbar**
   - Njörðr nimmt Provider in Marketplace auf
   - Provider ist verfügbar für Requests

### Request Creation and Routing

1. **Requester sendet Compute-Request**
   - Request mit Model-Requirements und Prompt
   - Max Cost wird angegeben
   - Zahlungsmethode wird geprüft (gültige Zahlungsmethode muss vorhanden sein)
   - Pre-Authorization für geschätzte Kosten (via Heidrun)

2. **Njörðr findet passende Provider**
   - Provider-Matching basierend auf Requirements
   - Quality-Assessment (via Eikthyrnir)
   - Fair Distribution Algorithm
   - Provider-Zahlungsmethode wird geprüft (gültige Zahlungsmethode für Auszahlung muss vorhanden sein)

3. **Request wird geroutet**
   - Njörðr routet Request an gewählten Provider
   - Provider verarbeitet Request
   - Response wird zurückgesendet

4. **Transaction wird abgeschlossen**
   - Njörðr koordiniert Transaction-Abschluss
   - Tokens werden gezählt (via Heidrun)
   - Kosten werden berechnet (via Heidrun)
   - Payment wird verarbeitet (von Requester)
   - Earnings werden gutgeschrieben (an Provider, via Heidrun)
   - Quality wird gemessen (via Eikthyrnir)
   - Njörðr schließt Transaction ab

### Payment Processing

1. **Pre-Authorization**
   - Heidrun schätzt Kosten basierend auf Request-Parametern
   - Pre-Authorization wird durchgeführt
   - Authorization wird gespeichert

2. **Payment Processing**
   - Nach Request-Verarbeitung: Tokens werden gezählt
   - Heidrun berechnet finale Kosten
   - Payment wird von Requester abgebucht
   - Provider-Earnings werden gutgeschrieben
   - Company-Fee wird einbehalten

3. **Settlement**
   - Provider-Earnings werden berechnet (totalCost - companyFee)
   - Company-Fee wird berechnet (totalCost * commissionRate)
   - Settlement wird durchgeführt

### Quality Tracking

1. **Quality Measurement**
   - Nach jedem Request: Eikthyrnir misst Quality-Metriken
   - Response-Quality, Latency, Availability werden gemessen
   - Quality-Metriken werden gespeichert

2. **Quality Aggregation**
   - Eikthyrnir aggregiert Quality-Metriken (gewichteter Durchschnitt)
   - Neuere Requests haben höheres Gewicht
   - Quality-Metriken werden aktualisiert

3. **Quality Updates**
   - Sofortige Updates für wichtige Änderungen
   - Batch-Aggregation für normale Updates
   - Quality-Metriken werden für Provider-Ranking verwendet

### Settlement and Earnings

1. **Settlement Calculation**
   - Heidrun berechnet Provider-Earnings (totalCost - companyFee)
   - Heidrun berechnet Company-Fee (totalCost * commissionRate)
   - Settlement wird durchgeführt

2. **Earnings Distribution**
   - Provider-Earnings werden gutgeschrieben
   - Company-Fee wird einbehalten
   - Transaction wird abgeschlossen

## API Endpoints

### Provider Management
- `POST /api/v1/marketplace/providers/register` - Register device as provider
- `GET /api/v1/marketplace/providers` - List available providers
- `GET /api/v1/marketplace/providers/:id` - Get provider details
- `PUT /api/v1/marketplace/providers/:id` - Update provider configuration
- `DELETE /api/v1/marketplace/providers/:id` - Unregister as provider

### Request Management
- `POST /api/v1/marketplace/requests` - Create compute request
- `GET /api/v1/marketplace/requests/:id` - Get request status

### Transaction Management
- `GET /api/v1/marketplace/transactions` - List transactions
- `GET /api/v1/marketplace/transactions/:id` - Get transaction details

### Analytics
- `GET /api/v1/marketplace/analytics/provider` - Get provider analytics
- `GET /api/v1/marketplace/analytics/requester` - Get requester analytics

## Plugin Marketplace

Der Plugin Marketplace ermöglicht es Usern, ihre eigenen Plugins zu entwickeln und zu verkaufen.

**Plugin-Entwicklung**
- **Interface-basiert**: Alle Plugins müssen das `OdinPlugin`-Interface implementieren
- **Function Call Protocol**: Plugins müssen ihre Funktionen über das Function Call Protocol beschreiben
- **Schnelle Erkennung**: Odin kann Plugins sofort verstehen, ohne Code-Analyse

**Plugin-Publishing**
- **Plugin-Registrierung**: Developer können Plugins registrieren
- **Plugin-Beschreibung**: Titel, Beschreibung, Funktionen werden über Interface bereitgestellt
- **Pricing**: Developer können Preise für ihre Plugins festlegen
- **Versioning**: Plugin-Versionen werden verwaltet

**Plugin-Kategorien**
- **Action-Execution**: Plugins für Action-Execution (z.B. Thor)
- **Coding**: Plugins für Coding-Aufgaben (z.B. Valkyries)
- **Healthcare**: Plugins für Gesundheitsfragen (z.B. Frigg)
- **Custom**: Benutzerdefinierte Plugins

**Plugin-Installation**
- **Marketplace-Installation**: User können Plugins direkt aus dem Marketplace installieren
- **Lokale Installation**: User können Plugins lokal installieren
- **Plugin-Management**: User können installierte Plugins verwalten

## Skills, Rules, Commands & Snippets Marketplace

Der Marketplace unterstützt auch den Verkauf von:
- **Skills**: Vordefinierte Funktionssammlungen für spezifische Aufgaben
- **Rules**: Regel-basierte Konfigurationen
- **Commands**: Vordefinierte Command-Sets
- **Snippets für AGENTS.md**: Code-Snippets für Agent-Konfigurationen

## Analytics Dashboard

**Provider Analytics**
- **Request Statistics**: Anzahl von Requests
- **Earnings**: Verdienst-Statistiken
- **Quality Metrics**: Quality-Metriken
- **Usage Patterns**: Nutzungsmuster

**Requester Analytics**
- **Request History**: Request-Historie
- **Cost Analysis**: Kosten-Analyse
- **Quality Metrics**: Quality-Metriken der verwendeten Provider

## Implementierungs-Details

### Provider Registration System

**Data Structure**
- Provider ID
- Device ID
- Capabilities (Models, Hardware Specs)
- Pricing Configuration
- Availability Settings
- Payment Method
- Registration Timestamp
- Status (PENDING, APPROVED, REJECTED, ACTIVE, INACTIVE)

**Storage**
- Yggdrasil Database
- Encrypted Storage
- Backup & Restore

### Request Routing System

**Implementation**
- Provider Matching Algorithm
- Quality Assessment Integration (Eikthyrnir)
- Fair Distribution Algorithm
- Load Balancing
- Failover Mechanism

**Features**
- Request Reception
- Provider Matching
- Quality Assessment
- Route Selection
- Request Routing

### Transaction System

**Implementation**
- Transaction Tracking
- Settlement Calculation (Heidrun)
- Payment Processing
- Refund Handling
- Dispute Resolution

**Transaction States**
- PENDING
- PROCESSING
- COMPLETED
- FAILED
- CANCELLED
- REFUNDED

### Quality Assessment System

**Implementation**
- Quality Measurement (Eikthyrnir)
- Quality Aggregation
- Quality Updates
- Quality Propagation

**Quality Metrics**
- Response Quality
- Latency
- Availability
- Reliability

### Provider Configuration

**Provider Setup**
- **Device Registration**: User kann Device als Provider registrieren
- **Capability Declaration**: Device teilt Capabilities mit
  - Verfügbare Models
  - Hardware Specs (GPU, RAM, etc.)
  - Pricing Configuration
- **Availability Settings**: Verfügbarkeit konfigurieren
- **Sharing Settings**: Welche Devices zum Sharing freigegeben werden

**Provider Configuration**
- **Pricing**: Preis pro 1000 Tokens (in Cents, ganzzahlig)
- **Model Selection**: Welche Models angeboten werden
- **Capacity Limits**: Maximale Kapazität
- **Quality Settings**: Quality-Settings für Requests
- **Zahlungsmethode erforderlich**: Gültige Zahlungsmethode muss hinterlegt sein für Auszahlung von Earnings

**Payment Structure**
- **Requester**: Zahlt totalCost (muss gültige Zahlungsmethode haben)
- **Provider**: Erhält providerEarnings (totalCost - companyFee, muss gültige Zahlungsmethode haben)
- **Company**: Erhält companyFee (10-15% Commission)

**Pricing Model**
- **Token Pricing**: Cent-Berechnung pro 1000 Tokens (ganzzahlig, keine Kommastellen)
- **Berechnung**: (tokens / 1000) * pricePerToken (aufgerundet)
- **Commission**: 10-15% des Token-Preises

### Request Routing Engine

**Request Processing**
- **Request Reception**: Empfang von Compute-Requests
- **Provider Matching**: Matching von Requests mit Providern
- **Quality Assessment**: Bewertung der Provider-Quality
  - Nach jedem Request: Quality wird nach jedem Request gemessen (automatisch + optionales User-Feedback)
  - Periodische Tests: Regelmäßige Tests ergänzen kontinuierliche Bewertung
  - Gewichteter Durchschnitt: Quality-Metriken werden aggregiert (neuere Requests haben höheres Gewicht)
  - Sofort + Batch: Sofortige Updates für wichtige Änderungen, Batch-Aggregation für Effizienz
- **Route Selection**: Auswahl des optimalen Providers

**Routing Algorithm**
- **Fair Distribution**: Fair Distribution Algorithm
  - Round-Robin bei gleichen Bedingungen
  - Berücksichtigung von:
    - Preis pro Token
    - Verfügbare Kapazität
    - Connection Quality
    - Estimated Latency
    - Provider History (Fairness-Score)
- **Load Balancing**: Lastverteilung über Provider
- **Failover**: Automatisches Failover bei Provider-Ausfall

## Testing Requirements

### Provider Registration Tests
- Provider Registration Workflow
- Provider Approval Workflow
- Provider Configuration Tests
- Payment Method Verification Tests

### Request Routing Tests
- Request Creation Tests
- Provider Matching Tests
- Quality Assessment Tests
- Request Routing Tests
- Failover Tests

### Transaction Tests
- Transaction Creation Tests
- Payment Processing Tests
- Settlement Tests
- Refund Tests
- Dispute Resolution Tests

### Quality Assessment Tests
- Quality Measurement Tests
- Quality Aggregation Tests
- Quality Update Tests
- Quality Propagation Tests

### Integration Tests
- Njörðr Integration Tests
- Heidrun Integration Tests
- Eikthyrnir Integration Tests
- Nornen Integration Tests
- Yggdrasil Integration Tests
- End-to-End Workflow Tests

### Security Tests
- Authentication Tests
- Authorization Tests
- Payment Security Tests
- Data Protection Tests

## Implementierungs-Notizen

- Muss sehr performant sein für hohe Request-Volumes
- Muss robustes Error-Handling haben
- Muss gut dokumentiert sein
- **Muss Provider-Registrierung unterstützen**: Vollständiger Provider-Registration-Workflow
- **Muss Request-Routing haben**: Fair Distribution Algorithm mit Quality-Assessment
- **Muss Payment-Processing haben**: Integration mit Payment Provider
- **Muss Quality-Tracking haben**: Integration mit Eikthyrnir
- **Muss Settlement haben**: Integration mit Heidrun
- **Muss Analytics haben**: Provider- und Requester-Analytics
- **Performance**: Muss optimiert sein für hohe Request-Volumes
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss Security-Mechanismen haben für Payment-Processing
- **Skalierbarkeit**: Muss horizontal skalierbar sein

