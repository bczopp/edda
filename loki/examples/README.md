# Loki Example Scripts

Beispiel-Lua-Skripte für Fenrir, Jörmungandr und Hel. Ausführung z. B. über gRPC `ExecuteScript(script_content)` oder nach Registrierung als Tool.

## Fenrir (Hardware)

- **led_control.lua** – GPIO-LED an/aus
- **sensor_read.lua** – Sensor (z. B. Temperatur) auslesen

## Jörmungandr (Netzwerk)

- **http_request.lua** – HTTP GET
- **mqtt_publish.lua** – MQTT Publish (Broker erforderlich)

## Hel (Daten/Speicher)

- **file_write.lua** – Datei schreiben und lesen
- **cache_example.lua** – Cache setzen/get/invalidieren
