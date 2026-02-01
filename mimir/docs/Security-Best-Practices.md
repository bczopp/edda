# Mimir – Security-Best-Practices

## Konfiguration

- **Verschlüsselungsschlüssel**: `encryption_key_path` außerhalb des Repos, dateiberechtigungen restriktiv (nur Service-User).
- **Datenbank**: Sichere Verbindung (TLS) und starke Zugangsdaten; keine Credentials in Config committen (Umgebungsvariablen/Secret-Manager).
- **Access Control**: `enable_access_control` in Produktion aktivieren; Nutzer-Context stets über gRPC-Metadata übergeben.

## Audit & Compliance

- **Audit Logging**: `enable_audit_logging` aktivieren; Audit-Logs nicht nachträglich verändern; Retention an Compliance-Anforderungen anpassen.

## Zugriff

- gRPC nur über vertrauenswürdige Netzwerke/TLS exponieren; Nutzer-Identität für alle datenbezogenen RPCs prüfen (RBAC).

## Siehe auch

- [GDPR-Compliance-Guide](GDPR-Compliance-Guide.md) – Betroffenenrechte und Datenschutz.
- [Deployment-Guide](Deployment-Guide.md) – Konfiguration und Deployment.
