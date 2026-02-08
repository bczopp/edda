# Mimir – GDPR-Compliance-Guide

Dieses Dokument beschreibt, wie Mimir die Anforderungen der DSGVO (GDPR) umsetzt und wie Dienste die entsprechenden APIs nutzen.

## Data Subject Rights (Betroffenenrechte)

| Recht | gRPC-Methode | Beschreibung |
|-------|--------------|--------------|
| **Right to Access (Art. 15)** | `ExportUserData` | Export aller personenbezogenen Daten eines Nutzers (JSON). |
| **Right to Rectification (Art. 16)** | `RectifyUserData` | Korrektur von Daten: `user_id`, `data_id`, `new_data`. |
| **Right to Erasure (Art. 17)** | `DeleteUserData` | Löschung aller Daten eines Nutzers („Recht auf Vergessenwerden“). |
| **Right to Data Portability (Art. 20)** | `ExportUserData` | Daten in einem strukturierten, maschinenlesbaren Format (JSON). |

Zusätzlich:
- **RetrieveData** – Abruf einzelner Datensätze (mit Access Control).
- **StoreData** / **DeleteData** – Speichern/Löschen einzelner Einträge (mit Audit).

## Data Protection (Datenschutzgrundsätze)

- **Data Minimization**: Policy-System (`DataMinimizationPolicy`: max_data_size, max_entries_per_user, forbidden_fields) prüft bei der Speicherung.
- **Purpose Limitation**: `purpose` wird bei Speicherung mitgeführt und bei Zugriff validiert (`store_data_with_purpose`, `retrieve_data_with_access_control_and_purpose`).
- **Storage Limitation**: Retention-Policy (`StorageLimitationPolicy`, `expires_at`), automatische Löschung abgelaufener Daten; Konfiguration über `data_retention` in den Settings.

## Audit & Nachweispflicht

- Alle datenbezogenen Operationen (Store, Retrieve, Delete, Export, Rectify, AccessDenied) werden im Audit-Log erfasst.
- Audit-Logs sind unveränderbar und für Compliance-Prüfungen abfragbar (`get_user_audit_logs`, `get_data_audit_logs`).

## Konfiguration (Auszug)

- **Security**: `enable_access_control`, `enable_audit_logging` – RBAC und Audit aktivieren.
- **Data Retention**: `default_retention_days`, `enable_auto_deletion`, `anonymize_on_deletion` in den Settings.

## Siehe auch

- [API-Dokumentation](API.md) – gRPC-Service und Nachrichten.
- [README](../README.md) – Übersicht und Architektur.
