# Jotunheim Platform Guide

Guide für ESP32-Setup, Flashen und Capability-Konfiguration.

---

## ESP32-Setup

### Voraussetzungen

- Rust (stable)
- ESP32-Toolchain (esp-rs) für nativen ESP32-Build

### Toolchain installieren

1. **espup** installieren (einmalig):

   ```bash
   cargo install espup
   espup install
   ```

2. **Target** hinzufügen:

   ```bash
   rustup target add xtensa-esp32-espidf
   ```

3. **Umgebung** aktivieren (pro Shell oder in Profile):

   - Linux/macOS: `source $HOME/export-esp.sh` (von espup ausgegeben)
   - Windows: die von espup angegebene Umgebung setzen

### Projekt bauen (Host vs. ESP32)

- **Host** (z.B. Tests, Entwicklung ohne Hardware):

  ```bash
  cd jotunheim
  cargo build --release
  # oder: ./scripts/build.sh   (ohne --esp32)
  ```

- **ESP32**:

  ```bash
  cd jotunheim
  ./scripts/build.sh --esp32
  # bzw. unter Windows: .\scripts\build.ps1 --esp32
  ```

  Entspricht: `cargo build --release -p jotunheim-esp32 --target xtensa-esp32-espidf`

---

## Flash-Instructions

### Voraussetzungen

- ESP32 per USB verbunden
- `espflash` installiert: `cargo install espflash`

### Flashen

Aus dem Jotunheim-Verzeichnis:

```bash
./scripts/flash.sh
# Windows: .\scripts\flash.ps1
```

Das Skript führt aus:

`cargo espflash flash --release -p jotunheim-esp32 --target xtensa-esp32-espidf --monitor`

- **flash**: schreibt das Firmware-Image auf den ESP32
- **--monitor**: startet danach den Serial-Monitor (Logs/Debug)

Ohne Monitor: `cargo espflash flash --release -p jotunheim-esp32 --target xtensa-esp32-espidf`

### Serieller Port

Wenn der Port nicht automatisch erkannt wird, z.B.:

- Linux: `--monitor --serial-port /dev/ttyUSB0`
- Windows: `--monitor --serial-port COM3`
- macOS: oft `/dev/cu.usbserial-*`

---

## Capability-Configuration

### Konfigurationsdateien

- Konfiguration liegt unter **esp32/config/** (z.B. JSON/TOML).
- Schema siehe **Phase 1.3** im IMPLEMENTATION_PLAN (Settings-Schema, Validator, Loader).

### Capabilities programmatisch (DeviceCapabilityBuilder)

Device-Information, Hardware, Ressourcen und Features werden mit dem **DeviceCapabilityBuilder** beschrieben und als **JotunheimCapabilities** (Proto) an Controller/Loki gemeldet:

```rust
use jotunheim_esp32::capability::DeviceCapabilityBuilder;

let caps = DeviceCapabilityBuilder::new()
    .device_id("esp32-001")
    .device_name("Living Room Sensor")
    .device_type("ESP32")
    .firmware_version("0.1.0")
    .protocol_version("1.0")
    .gpio_pins(&[2, 4, 5])
    .gpio_digital(true)
    .interfaces(&["I2C", "SPI"])
    .sensors(&["DHT22"])
    .actuators(&["LED"])
    .max_memory_kb(256)
    .max_concurrent_tools(4)
    .streaming(false)
    .build();
```

### Capability-Negotiation und -Propagation

- **CapabilityNegotiator**: reagiert auf Capability-Requests und sendet eine **CapabilityResponse** (mit obigen Capabilities).
- **CapabilityPropagator**: sendet bei Kopplung/Verbindung **CapabilityUpdateEvent** (z. B. `event_type`: "connected", "updated").

Details zu Nachrichten und Protokoll siehe **README.md** (Capability Structure, Negotiation Flow) und **proto/jotunheim_capability.proto**.

---

## Performance & Code-Size (Phase 10)

- **Release-Profil:** `esp32/Cargo.toml` enthält `[profile.release]` mit `opt-level = "z"`, LTO und `codegen-units = 1` für geringe Binary-Größe.
- **Größe prüfen:** `./scripts/check-size.sh` bzw. `.\scripts\check-size.ps1` baut Release und gibt die Binary-Größe aus. Optional: `MAX_BYTES=5000000 ./scripts/check-size.sh` bricht mit Fehler ab, wenn die Größe den Wert überschreitet.
- **RAM-Ziel:** Auf dem ESP32 ist ein RAM-Footprint von unter 10 KB angestrebt; Validierung auf dem Gerät (MemoryMonitor, MemoryPool, Tests unter `tests/memory_profiling_test.rs`).

---

## Weitere Hinweise

- **Tests**: `docker compose -f docker-compose.test.yml run --rm jotunheim-test` oder `./scripts/run-tests.sh` (siehe README).
- **CI**: `.github/workflows/jotunheim.yml` (Tests im Container, Lint).
