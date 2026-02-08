-- Jörmungandr: MQTT (Beispiel – Broker erforderlich)
-- Verwendung: jormungandr:mqtt_connect(host, port), jormungandr:mqtt_publish(topic, payload)
-- Beispiel: jormungandr:mqtt_connect("localhost", 1883)
--           jormungandr:mqtt_publish("test/topic", "hello")
jormungandr:mqtt_connect("localhost", 1883)
jormungandr:mqtt_publish("loki/example", "hello from Loki")
return "published"
