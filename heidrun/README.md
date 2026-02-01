# Heidrun - Token & Pricing Service

## Übersicht

**Tests ausführen:** Von `heidrun/`: `docker compose -f docker-compose.test.yml run --rm heidrun-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `heidrun/**` läuft die Pipeline [.github/workflows/heidrun.yml](../.github/workflows/heidrun.yml) (Test im Container, Lint).

Heidrun ist ein Rust-Microservice für Yggdrasil, der Token-Berechnungen, Pricing, Settlement und Pre-Authorization für den Marketplace durchführt.

**Mythologische Bedeutung**: Heidrun ist die Ziege, die Met produziert (Wert/Flüssigkeit).

**Programmiersprache**: Rust

## Verantwortlichkeiten

### 1. Token-Berechnungen
- **Token-Counting**: 
  - **Counting nach Request**: Token-Counting wird nach Request-Verarbeitung durchgeführt
  - **Provider-spezifische Methoden**: Provider-spezifische Token-Counting-Methoden (verschiedene Tokenizer)
  - **Token-Counting-Diskrepanzen**: Bei Token-Counting-Diskrepanzen wird Provider-Angabe verwendet (Provider hat Priorität)
  - Token-Counting nach Request-Verarbeitung
- **Token-Tracking**: 
  - **Tracking-Mechanismen**: Token-Verbrauch wird pro Request verfolgt (Request-ID, Token-Count, Timestamp)
  - **Tracking-Fehler**: Bei Tracking-Fehlern wird Fehler geloggt, Tracking wird fortgesetzt
  - Verfolgung von Token-Verbrauch pro Request
- **Token-Aggregation**: 
  - **Aggregations-Algorithmen**: Token-Statistiken werden aggregiert (Summe, Durchschnitt, etc.)
  - **Aggregations-Fehler**: Bei Aggregations-Fehlern wird Fehler geloggt, Aggregation wird erneut durchgeführt
  - Aggregation von Token-Statistiken

### 2. Pricing
- **Kostenberechnung**: Berechnung von Kosten basierend auf Token-Verbrauch
- **Pricing-Model**: Cent-Berechnung pro 1000 Tokens (ganzzahlig, keine Kommastellen)
- **Berechnungsformel**: `(tokens / 1000) * pricePerToken` (aufgerundet)

### 3. Settlement
- **Provider-Earnings**: Berechnung von Provider-Earnings (`providerEarnings = totalCost - companyFee`)
- **Company-Fee**: Berechnung der Company-Fee (`companyFee = totalCost * commissionRate`)
- **Commission-Berechnung**: Berechnung der Company-Commission (10-15%)

### 4. Pre-Authorization
- **Geschätzte Kosten**: Pre-Authorization für geschätzte Kosten vor Request
- **Kostenabschätzung**: Abschätzung der Kosten basierend auf Request-Parametern
- **Authorization-Handling**: Verwaltung von Pre-Authorizations

## Kommunikation

### gRPC
- **Yggdrasil (Elixir) ↔ Heidrun (Rust)**: gRPC
- **Asynchron**: Yggdrasil sendet Token-Requests, Heidrun antwortet mit Berechnungen
- **Type-safe**: Protobuf garantiert korrekte Request/Response-Strukturen

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

### Heidrun-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Token-Berechnungs-Einstellungen
- Pricing-Model-Einstellungen
- Settlement-Einstellungen

## Integration

### Yggdrasil-Integration
- **Marketplace**: Heidrun wird für alle Marketplace-Transaktionen genutzt
- **Request-Processing**: Token-Counting und Pricing nach jedem Request
- **Settlement**: Settlement-Berechnungen für Provider und Company

### Service-Integration
- **Nornen**: Heidrun liefert Pricing-Informationen für Entscheidungen
- **Eikthyrnir**: Heidrun kann Quality-basierte Pricing-Anpassungen unterstützen (optional)

## Berechnungsformeln

### Token-Pricing
```
totalCost = (tokens / 1000) * pricePerToken (aufgerundet)
```

### Settlement
```
companyFee = totalCost * commissionRate (10-15%)
providerEarnings = totalCost - companyFee
```

### Pre-Authorization
```
estimatedCost = (estimatedTokens / 1000) * pricePerToken (aufgerundet)
```

## Performance

### Performance-Optimierungen
- **Effiziente Berechnungen**: Optimierte Berechnungen für Token-Counting und Pricing
- **Caching**: Caching von Pricing-Konfigurationen
- **Batch-Processing**: Batch-Processing für Settlement-Berechnungen
- **Minimaler Footprint**: Minimaler RAM- und CPU-Verbrauch

## Sicherheit

### Security-Features
- **Input Validation**: Validierung aller eingehenden Token- und Pricing-Requests
- **Secure Calculations**: Sichere Berechnungen ohne Rundungsfehler
- **Audit Logging**: Audit-Logging für alle Berechnungen
- **No Hardcoded Secrets**: Keine Hardcoded Secrets oder Keys

### Security-Best-Practices
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Access Control**: Zugriffskontrolle für Pricing-Konfigurationen
- **Data Integrity**: Sicherstellung der Datenintegrität bei Berechnungen

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden verarbeitet
- **Keine persönlichen Daten**: Keine Speicherung von persönlichen Daten
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen

## Implementierungs-Notizen

- Muss sehr performant sein für hohe Request-Volumes
- Muss präzise Berechnungen haben (keine Rundungsfehler)
- Muss robustes Error-Handling haben
- Muss gut dokumentiert sein
- **Muss Token-Counting nach Request-Verarbeitung durchführen**: Automatisches Token-Counting
- **Muss Pricing-Berechnung haben**: Cent-Berechnung pro 1000 Tokens (ganzzahlig)
- **Muss Settlement-Berechnung haben**: Provider-Earnings und Company-Fee
- **Muss Pre-Authorization unterstützen**: Geschätzte Kosten vor Request
- **Muss Commission-Berechnung haben**: 10-15% Company-Commission
- **Performance**: Muss optimiert sein für hohe Request-Volumes
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss Security-Mechanismen haben für sichere Berechnungen

