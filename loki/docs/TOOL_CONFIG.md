# Loki Tool-Config-Format

Die Tool-Konfiguration wird als **TOML** geladen (Standard: `tools.toml`). Hot-Reload wird unterstützt.

## Schema

```toml
[[tools]]
name = "script_name"
description = "Kurzbeschreibung"
return_type = "string"   # string | number | boolean | object | array | void
script = { inline = "return 42" }
# ODER: script = { path = "/path/to/script.lua" }

[[tools]]
name = "with_params"
description = "Script mit Parametern"
parameters = [
  { name = "input", type = "string", required = true, description = "Eingabe" },
  { name = "count", type = "number", required = false }
]
return_type = "string"
script = { path = "scripts/with_params.lua" }
```

## Felder

| Feld | Typ | Pflicht | Beschreibung |
|------|-----|---------|--------------|
| `tools` | Array | ja | Liste der Tool-Definitionen |
| `name` | string | ja | Eindeutiger Script-Name (wird zu gRPC-Funktion) |
| `description` | string | ja | Beschreibung des Tools |
| `parameters` | Array | nein | Parameter (name, type, required, description) |
| `return_type` | string | ja | Rückgabetyp: string, number, boolean, object, array, void |
| `script` | object | ja | `inline` (Lua-Code) oder `path` (Dateipfad) |

## Parameter-Typen

`type`: string, number, boolean, object, array

## Integration

- **Config-Loader**: `ToolConfigLoader` lädt TOML von konfiguriertem Pfad.
- **Registry**: Registrierte Tools erscheinen in `ListScripts` und sind über `ExecuteScript` / `StreamScriptExecution` ausführbar.
- **RegisterScript** (gRPC): Registriert/aktualisiert ein Tool zur Laufzeit; Cache für dieses Script wird invalidiert.
