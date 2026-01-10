# Frigg - Healthcare Plugin

## Übersicht

Das Healthcare Plugin bietet zertifizierte Kurse, die von Krankenkassen genehmigt und bezahlt werden können. **Frigg** (Göttin der Fürsorge und Mutterschaft) führt das Healthcare Plugin an und koordiniert die Healthcare-Services. **Fulla** (Göttin der Fürsorge und Unterstützung) kann als Service für Daten und Behandlungspläne fungieren, falls benötigt.

**Wichtig**: Frigg ist ein separates Projekt, das als Extension zu Asgard hinzugefügt werden kann oder automatisch bei Yggdrasil vorhanden ist. Die Kommunikation mit Odin erfolgt über Thor (queue-basiert).

## Komponenten

### Frigg - Healthcare Plugin Lead
- **Verantwortlichkeiten**: 
  - Koordination aller Healthcare-Services
  - Course Management
  - Certification Management
  - Insurance Integration
  - Progress Tracking
- **Delegation**: Kann Aufgaben an Fulla delegieren (z.B. Datenbereitstellung, Behandlungspläne)

### Fulla - Healthcare Data Service (Optional)
- **Verantwortlichkeiten** (falls benötigt):
  - Datenbereitstellung für Healthcare-Kurse
  - Behandlungspläne bereitstellen
  - Unterstützung bei Course-Content
  - Datenverwaltung für Healthcare-Module
- **Delegation**: Wird von Frigg koordiniert

## Features

### 1. Course Management System (Frigg)
- Course Creation & Management
- Module Management
- Content Management
- Quiz/Assessment System
- **Delegation**: Frigg kann Fulla für Daten/Behandlungspläne nutzen

### 2. Certification Engine (Frigg)
- Certification Generation
- Certification Validation
- Certification Tracking
- Compliance Verification

### 3. Insurance Integration (Frigg)
- Insurance Provider Integration
- Claim Processing
- Payment Processing
- Approval Workflow

### 4. Progress Tracking (Frigg)
- User Progress Monitoring
- Completion Tracking
- Certification Tracking
- Analytics

### 5. Data & Treatment Plans (Fulla - Optional)
- **Datenbereitstellung**: Healthcare-Daten für Kurse
- **Behandlungspläne**: Bereitstellung von Behandlungsplänen
- **Content Support**: Unterstützung bei Course-Content mit Daten
- **Wird von Frigg koordiniert**: Fulla arbeitet unter Anleitung von Frigg

## Service-Architektur

### Frigg Service
- Haupt-Service für Healthcare Plugin
- Koordiniert alle Healthcare-Funktionen
- Kann Fulla-Service nutzen, falls benötigt

### Fulla Service (Optional)
- Wird von Frigg koordiniert
- Stellt Daten und Behandlungspläne bereit
- Unterstützt Frigg bei Content-Management

## Datenbank-Architektur

### Eigenständige Datenbank für Datenschutz

Frigg verwendet eine **eigenständige, isolierte Datenbank** für alle Gesundheitsdaten, um:
- **Datenschutz zu gewährleisten**: Gesundheitsdaten sind strikt von anderen Systemdaten getrennt
- **Schnellen und einfachen Zugriff**: Gesundheitsdaten eines Users sind schnell und einfach abrufbar
- **Compliance**: Erfüllung von Datenschutz-Anforderungen (GDPR, etc.)
- **Isolation**: Keine Vermischung mit anderen Daten

### Datenbank-Features

#### Datenschutz
- **Isolierte Datenbank**: Komplett getrennt von Yggdrasil und anderen Systemen
- **Verschlüsselung**: Alle Gesundheitsdaten werden verschlüsselt gespeichert
- **Access Control**: Strikte Zugriffskontrolle basierend auf User-Identität
- **Audit Logging**: Vollständiges Audit-Logging aller Datenzugriffe
- **Data Minimization**: Nur notwendige Daten werden gespeichert

#### Performance
- **Optimierte Indizes**: Schnelle Abfragen für User-spezifische Daten
- **Caching**: Intelligentes Caching für häufig abgerufene Daten
- **Query Optimization**: Optimierte Datenbankabfragen für schnellen Zugriff

### Datenbank-Schema

#### Users Table
- user_id (PK)
- encrypted_health_data
- created_at
- updated_at
- last_access

#### Course Progress Table
- progress_id (PK)
- user_id (FK)
- course_id
- module_id
- progress_data (JSON, verschlüsselt)
- completed_at
- started_at

#### Treatment Plans Table
- plan_id (PK)
- user_id (FK)
- plan_name
- plan_data (JSON, verschlüsselt)
- insurance_code
- activated_at
- expires_at

#### Health Records Table
- record_id (PK)
- user_id (FK)
- record_type
- record_data (JSON, verschlüsselt)
- created_at
- updated_at

### Datenbank-Technologie

- **Empfohlene Datenbank**: PostgreSQL (mit Verschlüsselung) oder spezialisierte Healthcare-Datenbank
- **Backup & Recovery**: Regelmäßige Backups mit Verschlüsselung
- **Replication**: Optional für Hochverfügbarkeit
- **Migration**: Sichere Datenmigration zwischen Umgebungen

### Datenzugriff

#### Schneller User-Datenzugriff
- **User-spezifische Indizes**: Optimierte Indizes für user_id
- **Partitionierung**: Optional nach User oder Zeitraum
- **Connection Pooling**: Effiziente Datenbankverbindungen
- **Query Optimization**: Spezielle Abfragen für häufige Zugriffe

#### API für Datenzugriff
- `GET /api/health/user/:userId` - Alle Gesundheitsdaten eines Users
- `GET /api/health/user/:userId/courses` - Kurs-Fortschritt eines Users
- `GET /api/health/user/:userId/plans` - Behandlungspläne eines Users
- `GET /api/health/user/:userId/records` - Gesundheitsaufzeichnungen eines Users

## Integration mit Thor

### Thor als Vermittler
- **Odin ruft Frigg NICHT direkt auf**: Odin arbeitet mit Thor und übergibt ihm Aufgaben
- **Thor erkennt Healthcare-Aufgaben**: Thor erkennt automatisch, ob es sich um eine Healthcare-Aufgabe handelt
- **User kann explizit anfordern**: User kann auch explizit Frigg anfordern oder eine Behandlung starten wollen
- **Queue-basierte Kommunikation**: Thor legt Task in Queue, Frigg holt Task ab und verarbeitet ihn
- **Ergebnis-Rückgabe**: Frigg legt Ergebnis in Ergebnis-Queue, Thor holt es ab und gibt es an Odin zurück

### Workflow: Odin → Thor → Frigg

1. **Odin erkennt Anforderung**
   - Odin erkennt, dass etwas verlangt wird (z.B. "Starte Gesundheitskurs")
   - Odin erstellt `ThorAction` und sendet es an Thor

2. **Thor erkennt Healthcare-Aufgabe**
   - Thor prüft, ob es sich um eine Healthcare-Aufgabe handelt
   - Falls ja: Thor legt Task in Queue für Frigg
   - Falls nein: Thor führt normale Action aus

3. **Frigg verarbeitet Task**
   - Frigg holt Task aus Queue
   - Frigg analysiert Task und verarbeitet Healthcare-Request
   - Frigg kann Fulla für Daten/Behandlungspläne nutzen

4. **Ergebnis-Rückgabe**
   - Frigg legt Ergebnis in Ergebnis-Queue
   - Thor holt Ergebnis aus Queue und erstellt `ThorResult`
   - Thor gibt `ThorResult` an Odin zurück
   - Odin verarbeitet Ergebnis und gibt Response an User

## Insurance Integration

### Freischaltcode-System
- **User benötigt Freischaltcode**: User muss einen Freischaltcode von der Versicherung erhalten
- **Yggdrasil als Proxy**: Integration mit Insurance Providers erfolgt über Yggdrasil (Yggdrasil als Proxy)
- **Freischaltung**: User gibt Freischaltcode ein, Yggdrasil validiert mit Insurance Provider
- **Nur freigeschaltete Pläne**: User darf nur für die Pläne freigeschaltet werden, die die Krankenkasse finanziert
- **Pläne werden freigeschaltet**: Nach Validierung werden entsprechende Behandlungspläne freigeschaltet

### Workflow: Freischaltung

1. **User erhält Freischaltcode**
   - User erhält Freischaltcode von seiner Krankenkasse
   - Code ist spezifisch für bestimmte Behandlungspläne

2. **User gibt Code ein**
   - User gibt Freischaltcode in Frigg ein
   - Frigg sendet Code an Yggdrasil zur Validierung

3. **Yggdrasil validiert**
   - Yggdrasil kontaktiert Insurance Provider API
   - Insurance Provider validiert Code
   - Insurance Provider teilt mit, welche Pläne finanziert werden

4. **Freischaltung**
   - Yggdrasil schaltet entsprechende Pläne frei
   - Frigg erhält Liste der freigeschalteten Pläne
   - User kann nur freigeschaltete Pläne starten

## Certification Management

### Zentrale Speicherung auf Yggdrasil
- **Yggdrasil speichert**: Certifications werden auf Yggdrasil gespeichert (zentrale Speicherung)
- **Validierung**: Certifications werden von Yggdrasil validiert
- **Zugriff**: Frigg kann Certifications von Yggdrasil abrufen
- **Synchronisation**: Certifications werden zwischen Devices synchronisiert (über Yggdrasil)

## Erkennung von Healthcare-Aufgaben

### Automatische Erkennung durch Thor
- **Keyword-basiert**: Thor erkennt Keywords wie "Gesundheit", "Kurs", "Behandlung", "Therapie", etc.
- **Context-basiert**: Thor analysiert Context der Anforderung
- **Heuristik**: Thor verwendet Heuristik zur Erkennung

### Explizite Anforderung durch User
- **User kann anfordern**: User kann explizit Frigg anfordern oder eine Behandlung starten wollen
- **Einfachere Erkennung**: Macht Erkennung einfacher und zuverlässiger
- **Beispiele**: "Starte Gesundheitskurs", "Zeige verfügbare Behandlungen", "Frigg: Starte Therapie"

## Deployment

### Als Extension zu Asgard
- **Optional**: Frigg kann als Extension zu Asgard hinzugefügt werden
- **Installation**: User kann Frigg separat installieren
- **Integration**: Nach Installation integriert sich Frigg mit Asgard
- **Thor-Integration**: Thor erkennt, ob Frigg verfügbar ist

### Automatisch bei Yggdrasil
- **Standard**: Frigg ist automatisch bei Yggdrasil vorhanden
- **Keine Installation nötig**: User muss nichts installieren
- **Immer verfügbar**: Healthcare-Aufgaben können immer über Yggdrasil verarbeitet werden
- **Thor-Integration**: Thor erkennt automatisch, dass Frigg verfügbar ist

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- **Thor**: Für Integration mit Odin (Queue-basierte Kommunikation)
- **Yggdrasil**: Für Insurance Provider Integration und Certification Storage
- **Eigenständige Datenbank**: PostgreSQL oder spezialisierte Healthcare-Datenbank für Gesundheitsdaten
- Insurance Provider APIs (über Yggdrasil)
- Certification Authorities
- Compliance Frameworks
- Fulla Service (optional, falls Daten/Behandlungspläne benötigt werden)

## Integration

- **Odin**: Erstellt `ThorAction` und erhält `ThorResult`
- **Thor**: Erkennt Healthcare-Aufgaben und vermittelt zwischen Odin und Frigg
- **Frigg**: Verarbeitet Healthcare-Tasks
- **Fulla**: Optional, für Daten/Behandlungspläne
- **Yggdrasil**: Für Insurance Provider Integration und Certification Storage
- **Queue-System**: Für Kommunikation zwischen Thor und Frigg

## Performance

### Performance-Optimierungen
- **Optimierte Datenbankabfragen**: Schnelle Abfragen für User-spezifische Daten
- **Caching**: Intelligentes Caching für häufig abgerufene Gesundheitsdaten
- **Connection Pooling**: Effizientes Connection-Pooling für Datenbankverbindungen
- **Index Optimization**: Optimierte Indizes für schnellen Datenzugriff
- **Query Optimization**: Spezielle Abfragen für häufige Zugriffe
- **Partitionierung**: Optional nach User oder Zeitraum für bessere Performance

### Performance-Metriken
- Schneller User-Datenzugriff (< 50ms für Standard-Queries)
- Effiziente Datenbankabfragen (optimierte Indizes)
- Hoher Durchsatz für parallele Requests

## Datenschutz

### Datenschutz-Features
- **Isolierte Datenbank**: Komplett getrennt von Yggdrasil und anderen Systemen
- **Verschlüsselung**: Alle Gesundheitsdaten werden verschlüsselt gespeichert
- **Access Control**: Strikte Zugriffskontrolle basierend auf User-Identität
- **Audit Logging**: Vollständiges Audit-Logging aller Datenzugriffe
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **User Control**: User hat volle Kontrolle über seine Gesundheitsdaten

### Compliance
- **GDPR-konform**: Vollständige Einhaltung der GDPR-Anforderungen
- **Healthcare Compliance**: Einhaltung von Healthcare-spezifischen Datenschutz-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Gesundheitsdaten löschen
- **Transparency**: User wird über Datenverarbeitung informiert
- **Data Portability**: User kann seine Daten exportieren

## Sicherheit

### Security-Features
- **Verschlüsselung**: Alle Gesundheitsdaten werden verschlüsselt gespeichert (at rest und in transit)
- **Access Control**: Strikte Zugriffskontrolle für Gesundheitsdaten
- **Authentication**: Sichere Authentifizierung über Heimdall
- **Authorization**: Granulares Permission-System für Gesundheitsdaten
- **Audit Logging**: Vollständiges Logging aller Datenzugriffe für Security-Audits
- **Threat Detection**: Erkennung von verdächtigen Zugriffen
- **Secure Backup**: Verschlüsselte Backups für Disaster Recovery

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Kontinuierliches Scanning für bekannte Vulnerabilities
- **Penetration Testing**: Regelmäßige Penetration Tests
- **Incident Response**: Automatische Response auf Security-Incidents
- **Compliance Audits**: Regelmäßige Compliance-Audits

## Implementierungs-Notizen

- Frigg muss als separates Projekt implementiert werden
- **Eigenständige Datenbank**: Muss eine isolierte Datenbank für Gesundheitsdaten haben
- **Datenschutz**: Alle Gesundheitsdaten müssen verschlüsselt gespeichert werden
- **Performance**: Datenbank muss für schnellen User-Datenzugriff optimiert sein
- Thor muss Healthcare-Aufgaben erkennen können (automatisch + explizit)
- Queue-System muss zwischen Thor und Frigg funktionieren
- Frigg kann als Extension zu Asgard hinzugefügt werden
- Frigg ist automatisch bei Yggdrasil vorhanden
- Thor muss prüfen, ob Frigg verfügbar ist
- Insurance-Integration erfolgt über Yggdrasil (nicht direkt)
- Freischaltcode-System muss implementiert werden
- Certifications werden auf Yggdrasil gespeichert (nicht in Frigg-Datenbank)
- Fulla ist optional und wird von Frigg koordiniert
- **Datenbank-Backup**: Regelmäßige, verschlüsselte Backups müssen implementiert werden
- **Access Control**: Strikte Zugriffskontrolle für Gesundheitsdaten
- **Audit Logging**: Vollständiges Logging aller Datenzugriffe
- **Performance**: Muss optimiert sein für schnellen Zugriff auf Gesundheitsdaten
- **Datenschutz**: Muss höchste Datenschutz-Standards erfüllen (GDPR, Healthcare Compliance)
- **Sicherheit**: Muss Enterprise-Grade Security haben mit kontinuierlichem Monitoring

