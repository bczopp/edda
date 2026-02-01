# Bifrost Mesh-Layer Design

## Übersicht

Bifrost erweitert sich um einen **Mesh-Layer** (Meshtastic-inspiriert). Device-Connectivity beruht auf Mesh-Membership statt VPN. Alle Devices eines Users bilden ein User-Mesh; Nachrichten können über mehrere Hop zum Ziel gelangen.

## Architektur

- **Mesh-Layer** (neu): Membership, Discovery, Multi-Hop-Routing (Managed Flood), Transports (IP, optional LoRa).
- **Bifrost-Protokoll** (bestehend): Connection-Establishment, MESSAGE, HEARTBEAT, Events – läuft *über* den Mesh-Layer.

## Paketformat (Meshtastic-inspiriert)

### MeshPacket-Envelope

- **from**: Sender-Node-ID (fixed32)
- **to**: Ziel-Node-ID oder Broadcast (fixed32, 0xFFFFFFFF = Broadcast)
- **hop_limit**: Verbleibende Hops (uint32); 0 = nicht weiterleiten
- **hop_start**: Ursprünglicher Hop-Limit beim Senden (für Hop-Count)
- **want_ack**: Zuverlässige Zustellung gewünscht (bool)
- **id**: Eindeutige Paket-ID pro Sender (fixed32, für Deduplizierung/ACK)
- **channel**: Kanal-Index / Channel-Hash (uint32)
- **payload**: decoded (Data) oder encrypted (bytes)
- **next_hop**, **relay_node**: Routing-Information (intern)

### Data (SubPacket)

- **portnum**: Port/Service-Typ (z.B. Bifrost-Protokoll, Routing, Discovery)
- **payload**: Nutzdaten (bytes)
- **dest**, **source**: End-Ziel und -Quelle für Multi-Hop
- **request_id**, **reply_id**: Für Request/Response-Zuordnung
- **want_response**: Antwort anfordern (bool)

### NodeInfo / User / MyNodeInfo

- Identity und Discovery; Anbindung an Heimdall/User-Identität.
- **NodeInfo**: Num, User, Position (optional), snr, last_heard, hops_away, etc.
- **User**: id, long_name, short_name, public_key, role, etc.
- **MyNodeInfo**: my_node_num, reboot_count, device_id, etc.

## Managed Flood Routing

- Kurz warten vor Rebroadcast, um Duplikate zu reduzieren (jitter/delay).
- Bei Empfang eines MeshPackets: wenn **to** != eigene Node-ID und **to** != Broadcast → optional weiterleiten, wenn **hop_limit** > 0.
- **hop_limit** bei jedem Hop dekrementieren; bei 0 nicht weiterleiten.
- **want_ack**: Bei Unicast ACK zurück; bei Broadcast implizites ACK, wenn Rebroadcast beobachtet wird (Meshtastic-Strategie).

## Discovery

- Neighbor-Erkennung über empfangene MeshPackets (NodeInfo/User in Payload oder gesonderte Discovery-Pakete).
- Lokale Discovery: mDNS/Bonjour weiterhin für IP-Nachbarn; Mesh-Layer hält NodeDB (NodeInfo pro Node).
- Yggdrasil/Asgard: Optional als Mesh-Relay-Knoten oder externer Relay für Bifrost-Streams (noch im Design zu klären).

## Hop-Limit

- Konfigurierbarer Max-Hop (z.B. 3–5); **hop_limit** und **hop_start** im MeshPacket.
- Verhindert endloses Flooding; typische User-Mesh-Topologie: wenige Hops.

## IP-Transport-Entscheid

- **Primär (Bifrost-Protokoll, persistente Sessions)**: **WebSocket über TLS** – bereits verwendet, bleibt Transport für Connection/MESSAGE/HEARTBEAT.
- **Mesh-Routing (Paketverteilung im Mesh)**: **WebSocket** pro Nachbar-Verbindung; jede bestehende Bifrost-WebSocket-Verbindung kann MeshPackets tragen. Kein separates UDP-Protokoll in Phase 1.
- **Optional später**: UDP oder Multicast-UDP für reines Mesh-Routing (ähnlich Meshtastic Multicast-UDP), wenn Performance oder Broadcast-Storm-Management es erfordern.

**Festgehaltene Wahl**: WebSocket (TLS) als einheitlicher Transport für Mesh + Bifrost-Protokoll in der ersten Phase. LoRa als weiterer Transport in einer späteren Phase (gleiche MeshPacket/Data-Semantik).

## Security (gegenüber Standard-Meshtastic verschärft)

- Verschlüsselung/Integrität konsequent (TLS für WebSocket; Payload optional E2E mit ChaCha20-Poly1305).
- Mesh-Membership und Device-Identität über Heimdall (Tokens, Permissions); kein offenes Mesh.
- Channel/Port-Konzepte nur für User-Mesh; Bifrost-Port für Application-Protokoll.

## LoRa (optional, spätere Phase)

- Gleiche MeshPacket/Data-Semantik; anderer physikalischer Kanal (z.B. Jotunheim).
- Mesh-Layer erhält Transport-Abstraktion; LoRa-Transport implementiert gleiche Send/Receive-Schnittstelle wie IP-Transport.

## Guest Mesh (ohne VPN)

- **Eigene Devices** eines Users nutzen das **Main Mesh** – kein Guest Mesh nötig.
- **Guest Mesh** nur für **fremde Devices** (Gäste/Besucher), die an derselben Bifrost-Instanz teilnehmen, ohne dort einen User-Account zu haben (z. B. Besucher-Phone). Isoliertes Segment: eigene Mesh-ID, kein Flood in den Haupt-User-Mesh, explizite Erlaubnis für Datentransfer (Heimdall-User-Bestätigung).
- Kein VPN (Valhalla); Connectivity über Mesh-Membership. Siehe Phase 12 (Guest Mesh) im IMPLEMENTATION_PLAN.

## Offene Punkte (kurzfristig)

- Yggdrasil/Asgard: Als Mesh-Knoten (Relay) vs. nur externer Relay für Bifrost-Streams.
- Meshtastic-Protobuf-Kompatibilität: Schlanke Edda-Variante vs. bewusst kompatibel zu Meshtastic-Protobufs für spätere LoRa-Knoten von Drittanbietern.
