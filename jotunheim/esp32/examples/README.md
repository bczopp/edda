# Jotunheim ESP32 Examples

Minimal examples for the Jotunheim ESP32 crate (Phase 12.2.1).

## Voraussetzungen

- Aus dem Workspace-Root `jotunheim/`: Tests/Build mit `cargo build -p jotunheim-esp32` (Host).
- Für ESP32-Hardware: ESP32-Toolchain (siehe [../docs/PLATFORM_GUIDE.md](../docs/PLATFORM_GUIDE.md)).

## Beispiele ausführen

Alle Befehle von `jotunheim/` aus:

```bash
# LED-Control: Capabilities mit LED-Actuator und Tools
cargo run -p jotunheim-esp32 --example led_control

# Sensor-Reading: Capabilities für Sensoren + ResourceMonitor
cargo run -p jotunheim-esp32 --example sensor_reading

# Remote-Control: RemoteCommandHandler mit Mock-Executor
cargo run -p jotunheim-esp32 --example remote_control
```

## Inhalt

| Example         | Beschreibung |
|----------------|--------------|
| **led_control**    | DeviceCapabilityBuilder mit LED-Actuator, GPIO, Tools (led_on/led_off). |
| **sensor_reading** | DeviceCapabilityBuilder für Sensoren (DHT22, BMP280), ResourceMonitor (RAM/CPU). |
| **remote_control** | RemoteCommandHandler mit Mock-Executor; auf dem Gerät würde LokiClient als Executor genutzt. |

Auf dem ESP32 ersetzen Sie den Mock durch echte Hardware-Anbindung bzw. `LokiClient::connect(...)`.
