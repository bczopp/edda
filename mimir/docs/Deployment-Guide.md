# Mimir – Deployment-Guide

## Voraussetzungen

- **PostgreSQL** (mit Migrations aus `migrations/`)
- **Konfiguration**: `config/mimir.json` (Pfad relativ zum Arbeitsverzeichnis)

## Konfiguration

- **grpc_port**: gRPC-Server-Port (Standard: 50059).
- **database**: `url`, `max_connections`, `min_connections`.
- **security**: `encryption_algorithm`, `key_rotation_days`, `enable_access_control`, `enable_audit_logging`.
- **data_retention**: `default_retention_days`, `enable_auto_deletion`, `anonymize_on_deletion`.
- **encryption_key_path**: Pfad zur Datei mit dem 32-Byte-Verschlüsselungsschlüssel (wird bei Bedarf generiert).
- **logging**: `log_format` (text/json), `log_directory` (optional, tägliche Rotation). Log-Level über Umgebungsvariable `RUST_LOG`.

## Container / Docker

- **Tests**: `docker compose -f docker-compose.test.yml run --rm mimir-test` (PostgreSQL per Testcontainer).
- **Produktion**: Service bauen und mit erreichbarem PostgreSQL sowie gültiger `config/mimir.json` starten; Migrations vor dem ersten Start ausführen.

## Migrations

- `migrations/001_initial_schema.sql`, `002_add_purpose_retention.sql` – werden z. B. in der Test-Infrastruktur und bei Bedarf im Deployment ausgeführt (`sqlx migrate run`).

## Sicherheit

- Keine Secrets in der Konfiguration committen; `encryption_key_path` auf eine sichere, nur dem Prozess zugängliche Datei zeigen.
- Access Control und Audit Logging für produktive Nutzung aktivieren (`enable_access_control`, `enable_audit_logging`).

## Siehe auch

- [README](../README.md) – Übersicht und Settings.
- [GDPR-Compliance-Guide](GDPR-Compliance-Guide.md) – DSGVO-Features und Nutzung.
