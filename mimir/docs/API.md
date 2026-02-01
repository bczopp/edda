# Mimir – gRPC API-Dokumentation

Service: **MimirService** (Port konfigurierbar, Standard: 50059).

## RPCs

| RPC | Request | Response | Beschreibung |
|-----|---------|----------|--------------|
| **StoreData** | StoreDataRequest | StoreDataResponse | Daten speichern (verschlüsselt, optional mit Purpose). |
| **RetrieveData** | RetrieveDataRequest | RetrieveDataResponse | Einzelnen Datensatz abrufen (mit Access Control). |
| **DeleteData** | DeleteDataRequest | DeleteDataResponse | Einzelnen Datensatz löschen. |
| **ExportUserData** | ExportUserDataRequest | ExportUserDataResponse | Alle Daten eines Nutzers exportieren (GDPR Art. 15/20). |
| **DeleteUserData** | DeleteUserDataRequest | DeleteUserDataResponse | Alle Daten eines Nutzers löschen (GDPR Art. 17). |
| **RectifyUserData** | RectifyUserDataRequest | RectifyUserDataResponse | Nutzerdaten korrigieren (GDPR Art. 16). |

## Nachrichten

- **StoreDataRequest**: `user_id`, `data` (bytes).  
- **StoreDataResponse**: `data_id`.
- **RetrieveDataRequest**: `data_id`, `user_id`.  
- **RetrieveDataResponse**: `data` (bytes).
- **DeleteDataRequest**: `data_id`, `user_id`.  
- **DeleteDataResponse**: `success`.
- **ExportUserDataRequest**: `user_id`.  
- **ExportUserDataResponse**: `data` (JSON-Export, bytes).
- **DeleteUserDataRequest**: `user_id`.  
- **DeleteUserDataResponse**: `success`.
- **RectifyUserDataRequest**: `user_id`, `data_id`, `new_data`.  
- **RectifyUserDataResponse**: `success`, `data_id`.

Authentifizierung/User-Context erfolgt über gRPC-Metadata (z. B. für Access Control). Proto-Definition: `proto/mimir.proto`.
