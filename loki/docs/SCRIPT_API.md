# Loki Script API – Fenrir, Jörmungandr, Hel

Scripts werden in Lua ausgeführt. Loki registriert je nach Script-Inhalt die APIs **fenrir**, **jormungandr** und **hel** in der Lua-Umgebung. Erkennung erfolgt über Vorkommen von `fenrir:`, `jormungandr:` bzw. `hel:` im Quelltext.

---

## Fenrir (Hardware)

Global: `fenrir`

| Methode | Beschreibung |
|--------|--------------|
| `fenrir:gpio_read(pin)` | GPIO-Pin lesen (number → boolean) |
| `fenrir:gpio_write(pin, value)` | GPIO-Pin setzen (pin: number, value: boolean) |
| `fenrir:sensor_read(sensor_id)` | Sensor lesen (z. B. Temperatur; string → number) |
| `fenrir:actuator_set(actuator_id, value)` | Aktor setzen (z. B. LED 0.0–1.0; string, number) |

---

## Jörmungandr (Netzwerk)

Global: `jormungandr`

### HTTP
| Methode | Beschreibung |
|--------|--------------|
| `jormungandr:http_get(url)` | GET (string → string) |
| `jormungandr:http_post(url, body)` | POST (string, string → string) |
| `jormungandr:http_put(url, body)` | PUT (string, string → string) |
| `jormungandr:http_delete(url)` | DELETE (string → void) |

### WebSocket
| Methode | Beschreibung |
|--------|--------------|
| `jormungandr:ws_connect(url)` | Verbindung herstellen |
| `jormungandr:ws_send(text)` | Text senden |
| `jormungandr:ws_receive()` | Nächste Text-Nachricht (string oder nil) |
| `jormungandr:ws_reconnect()` | Verbindung neu aufbauen |

### MQTT
| Methode | Beschreibung |
|--------|--------------|
| `jormungandr:mqtt_connect(host, port)` | Broker verbinden |
| `jormungandr:mqtt_publish(topic, payload)` | Nachricht publizieren (payload: string) |
| `jormungandr:mqtt_subscribe(topic)` | Topic abonnieren |

---

## Hel (Daten/Speicher)

Global: `hel`

### Dateisystem
| Methode | Beschreibung |
|--------|--------------|
| `hel:fs_read(path)` | Datei lesen (string → string) |
| `hel:fs_write(path, contents)` | Datei schreiben |
| `hel:fs_delete(path)` | Datei/leeres Verzeichnis löschen |
| `hel:fs_list_dir(path)` | Verzeichnisinhalt (string → table of strings) |
| `hel:fs_create_dir(path)` | Verzeichnis (und Eltern) anlegen |

### Key-Value-Storage
| Methode | Beschreibung |
|--------|--------------|
| `hel:storage_get(key)` | Wert lesen (string → string oder nil) |
| `hel:storage_set(key, value)` | Wert setzen |
| `hel:storage_remove(key)` | Eintrag entfernen |
| `hel:storage_keys()` | Alle Keys (→ table of strings) |

### Cache
| Methode | Beschreibung |
|--------|--------------|
| `hel:cache_get(key)` | Wert aus Cache (string → string oder nil) |
| `hel:cache_set(key, value)` | Wert im Cache setzen (mit TTL) |
| `hel:cache_invalidate(key)` | Eintrag invalidieren |
| `hel:cache_invalidate_all()` | Gesamten Cache leeren |
