# Eikthyrnir - Quality Assessment Service

## Übersicht

Eikthyrnir ist ein Rust-Microservice für Yggdrasil, der Quality Assessment, Quality-Aggregation und Quality-Metriken für Marketplace-Provider verwaltet.

**Mythologische Bedeutung**: Eikthyrnir ist der Hirsch, der aus dem Brunnen trinkt. Die Tropfen werden zu Flüssen (Qualität fließt weiter).

**Programmiersprache**: Rust

## Verantwortlichkeiten

### 1. Quality Assessment
- **Post-Request Assessment**: 
  - **Assessment nach Request**: Provider-Quality wird nach jedem Request bewertet
  - **Assessment-Algorithmen**: Assessment-Algorithmen (Multi-Faktor-Bewertung, Quality-Scoring)
  - **Assessment-Fehler**: Bei Assessment-Fehlern wird Fehler geloggt, Assessment wird erneut durchgeführt
  - Bewertung der Provider-Quality nach jedem Request
- **Automatische Messung**: 
  - **Messungs-Algorithmen**: Automatische Quality-Messung nach jedem Request (Response-Analyse, Latency-Messung)
  - **Messungs-Fehler**: Bei Messungs-Fehlern wird Fehler geloggt, Messung wird erneut durchgeführt
  - Automatische Quality-Messung nach jedem Request
- **Periodische Tests**: 
  - **Test-Strategien**: Periodische Tests (Health-Checks, Quality-Validation)
  - **Test-Fehler**: Bei Test-Fehlern wird Fehler geloggt, Tests werden fortgesetzt
  - Regelmäßige Tests ergänzen kontinuierliche Bewertung
- **Quality-Metriken**: 
  - **Metrik-Definitionen**: Metrik-Definitionen (Response-Quality, Latency, Availability, Reliability)
  - **Metrik-Updates**: Metrik-Updates werden gehandhabt (automatisch nach jedem Request)
  - Messung von Response-Quality, Latency, Availability

### 2. Quality-Aggregation
- **Gewichteter Durchschnitt**: Gewichteter Durchschnitt von Quality-Metriken
- **Quality-Weighting**: Neuere Requests haben höheres Gewicht
- **Batch-Aggregation**: Batch-Aggregation für Effizienz
- **Sofortige Updates**: Sofortige Updates für wichtige Änderungen

### 3. Quality-Metriken
- **Response-Quality**: Messung der Response-Quality
- **Latency**: Messung der Response-Latency
- **Availability**: Messung der Provider-Availability
- **Reliability**: Messung der Provider-Reliability

### 4. Quality-Updates
- **Sofortige Updates**: Sofortige Updates für wichtige Änderungen
- **Batch-Aggregation**: Batch-Aggregation für Effizienz
- **Quality-Propagation**: Quality-Updates werden an relevante Services propagiert

## Kommunikation

### gRPC
- **Yggdrasil (Elixir) ↔ Eikthyrnir (Rust)**: gRPC
- **Asynchron**: Yggdrasil sendet Quality-Assessment-Requests, Eikthyrnir antwortet mit Quality-Metriken
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

### Eikthyrnir-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Quality-Assessment-Einstellungen
- Quality-Metriken-Einstellungen
- Aggregation-Einstellungen

## Integration

### Yggdrasil-Integration
- **Marketplace**: Eikthyrnir wird für alle Marketplace-Provider-Quality-Assessments genutzt
- **Request-Processing**: Quality-Assessment nach jedem Request
- **Provider-Ranking**: Quality-Metriken für Provider-Ranking

### Service-Integration
- **Nornen**: Eikthyrnir liefert Quality-Metriken für Entscheidungen
- **Heidrun**: Eikthyrnir kann Quality-basierte Pricing-Anpassungen unterstützen (optional)

## Quality-Messung

### Automatische Messung
- **Nach jedem Request**: Automatische Quality-Messung nach jedem Request
- **Response-Quality**: Messung der Response-Quality (Korrektheit, Vollständigkeit)
- **Latency**: Messung der Response-Latency
- **Availability**: Messung der Provider-Availability

### Periodische Tests
- **Regelmäßige Tests**: Regelmäßige Tests ergänzen kontinuierliche Bewertung
- **Health Checks**: Health Checks für Provider
- **Quality Validation**: Validation der Quality-Metriken

### Gewichteter Durchschnitt
- **Quality-Aggregation**: Quality-Metriken werden aggregiert
- **Weighting**: Neuere Requests haben höheres Gewicht
- **Time-Decay**: Ältere Requests haben geringeres Gewicht

### Update-Strategie
- **Sofort + Batch**: Sofortige Updates für wichtige Änderungen, Batch-Aggregation für Effizienz
- **Important Changes**: Sofortige Updates bei wichtigen Quality-Änderungen
- **Efficiency**: Batch-Aggregation für normale Updates

## Performance

### Performance-Optimierungen
- **Effiziente Aggregation**: Optimierte Aggregation-Algorithmen
- **Caching**: Caching von Quality-Metriken
- **Batch-Processing**: Batch-Processing für Quality-Updates
- **Minimaler Footprint**: Minimaler RAM- und CPU-Verbrauch

## Sicherheit

### Security-Features
- **Input Validation**: Validierung aller eingehenden Quality-Assessment-Requests
- **Secure Calculations**: Sichere Berechnungen für Quality-Metriken
- **Audit Logging**: Audit-Logging für alle Quality-Assessments
- **No Hardcoded Secrets**: Keine Hardcoded Secrets oder Keys

### Security-Best-Practices
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Access Control**: Zugriffskontrolle für Quality-Konfigurationen
- **Data Integrity**: Sicherstellung der Datenintegrität bei Quality-Berechnungen

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden verarbeitet
- **Keine persönlichen Daten**: Keine Speicherung von persönlichen Daten in Quality-Metriken
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen

## Implementierungs-Notizen

- Muss sehr performant sein für hohe Request-Volumes
- Muss präzise Quality-Berechnungen haben
- Muss robustes Error-Handling haben
- Muss gut dokumentiert sein
- **Muss Quality-Assessment nach jedem Request durchführen**: Automatische Quality-Messung
- **Muss Quality-Aggregation haben**: Gewichteter Durchschnitt von Quality-Metriken
- **Muss Quality-Weighting haben**: Neuere Requests haben höheres Gewicht
- **Muss sofortige Updates für wichtige Änderungen haben**: Sofortige Quality-Updates
- **Muss Batch-Aggregation für Effizienz haben**: Batch-Processing für normale Updates
- **Muss periodische Tests haben**: Regelmäßige Tests ergänzen kontinuierliche Bewertung
- **Performance**: Muss optimiert sein für hohe Request-Volumes
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss Security-Mechanismen haben für sichere Quality-Berechnungen

