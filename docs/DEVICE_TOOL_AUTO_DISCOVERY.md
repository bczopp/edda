# Device Tool Auto-Discovery und Tool-Generierung

## Ziel

Eingebaute Sensoren und Aktoren (und optional GPIO) sollen **automatisch identifiziert** werden; daraus werden **Tool-Calling-Definitionen automatisch erzeugt**. Der User muss diese Definitionen nicht mehr manuell anlegen. Soweit möglich sollen auch **kryptische Gerätebezeichnungen** (z. B. von einer anderen Platform, die mit einem Jotunheim-Device verbunden ist) aufgelöst werden – idealerweise für alle verbundenen Geräte/Bauteile.

## Komponenten

### 1. Jotunheim (IoT-Platform)

- **Capabilities:** Device meldet bereits `sensors[]`, `actuators[]`, `hardware.gpio` (siehe `JotunheimCapabilities`).
- **Device Tool Registry:** Mapping von bekannten Sensor-/Aktor-Typen (z. B. `DHT22`, `LED`, `BMP280`) auf Standard-Tool-Schemas (Name, Beschreibung, Parameter, Rückgabetyp).
- **Tool-Generierung:** `generate_tools_from_capabilities(capabilities)` erzeugt eine Liste von Tool-Definitionen aus:
  - jedem Eintrag in `sensors` (z. B. DHT22 → `read_dht22_temperature`, `read_dht22_humidity`),
  - jedem Eintrag in `actuators` (z. B. LED → `set_led`),
  - optional: generische GPIO-Tools wenn `gpio.digital` true (`gpio_read`, `gpio_write`).
- **Unbekannte Typen:** Generische Tools wie `read_sensor(sensor_type, pin)` / `set_actuator(actuator_type, pin, value)` als Fallback, damit immer nutzbare Tools entstehen.
- **GPIO:** User soll korrekte Definition vornehmen; wo möglich kann automatisch erkannt werden (z. B. I2C-Scan), um zusätzliche Möglichkeiten vorzuschlagen.

### 2. Loki (Script/Tool-Service)

- **RegisterScript / Tool-Config:** Kann Tool-Definitionen von außen erhalten (z. B. von Jotunheim oder Odin). Für auto-generierte Tools: Stub-Skripte, die den Aufruf an das Gerät weiterleiten (z. B. gRPC/Call an Jotunheim).
- **Integration:** Platform (oder Odin) ruft Loki `RegisterScript` mit den von Jotunheim gelieferten Tool-Definitionen (+ Stub-Script) auf, sodass die Tools für die LLM sichtbar sind.

### 3. Odin (Orchestrierung) / verbundene Platform

- **Device-Auflösung:** Wenn nur kryptische IDs (z. B. `ESP-7F2A`, `0x3f`) verfügbar sind: Odin (oder die Platform, die die Verbindung zum Jotunheim-Device hält) kann eine **Device-Registry** bzw. einen **Resolver** bereitstellen: `device_id` / Hardware-ID → Anzeigename + `JotunheimCapabilities` (oder Referenz darauf).
- **Ablauf:** Platform verbindet sich mit Jotunheim-Device, erhält Capabilities (inkl. device_id, device_name), speichert Zuordnung. Später: Nutzeranfrage oder LLM referenziert „Gerät in der Küche“ oder eine ID → Odin/Platform löst auf und stellt die passenden (auto-generierten) Tools bereit.

### 4. Ablauf (Zusammenfassung)

1. **Jotunheim-Device** meldet Capabilities (sensors, actuators, gpio) – wie heute.
2. **Tool-Generierung** (in Jotunheim oder von der aufrufenden Platform): `generate_tools_from_capabilities(capabilities)` → Liste von Tool-Schemas (ohne Script).
3. **Platform/Odin** hat ggf. Device-Resolver: kryptische ID → device_name + capabilities.
4. **Loki** (oder die Platform) registriert die generierten Tools inkl. Stub-Script, das den Call an das jeweilige Jotunheim-Device weiterleitet.
5. **LLM** sieht die Tools wie gewohnt (z. B. über Einherjar/ListScripts) und kann sie nutzen, ohne dass der User Tool-Definitionen manuell anlegen muss.

## Erweiterungen (optional)

- **GPIO-Auto-Erkennung:** Wenn auf dem Device I2C/SPI-Scan oder Pin-Profile möglich sind, können vorgeschlagene Sensor-/Aktor-Typen oder Pin-Belegungen abgeleitet werden; der User bestätigt oder passt an.
- **Einheitliche Geräte-IDs:** Über alle verbundenen Geräte/Bauteile hinweg ein konsistentes Schema für IDs und Anzeigenamen (z. B. in Odin/Device-Registry), damit überall dieselbe Auflösung genutzt werden kann.

## Implementierungsstand

- **Jotunheim:** 
  - `generate_tools_from_capabilities(capabilities, prefix)` + eingebaute Registry für DHT22, DS18B20, BMP280, LED, Relay, Motor; generische Fallbacks; GPIO-Tools (siehe `device_tool_registry.rs`, Tests `device_tool_registry_test.rs`).
  - **Device-Resolver:** Trait `DeviceResolver` (`resolve(device_id) -> Option<ResolvedDevice>`, `list_device_ids()`) und `InMemoryDeviceResolver` (register/unregister) für Auflösung kryptischer IDs zu Anzeigename + Capabilities (siehe `device_resolver.rs`, Tests `device_resolver_test.rs`). Odin/Platform können das Trait implementieren oder `InMemoryDeviceResolver` mit persistenter Befüllung nutzen.
- **Loki:** Nutzung über bestehendes `RegisterScript` / Tool-Config; Stub-Skripte für Device-Weiterleitung können dort oder in der Platform ergänzt werden.
- **Odin/Platform:** Device-Resolver-Interface in Jotunheim umgesetzt; Odin kann eigenes Registry-Backend implementieren oder den In-Memory-Resolver mit Daten aus der eigenen Device-Registry befüllen.
