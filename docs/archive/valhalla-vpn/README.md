# Archiv: Valhalla VPN

## Status

**Valhalla (VPN Service) wurde verworfen.** Die ursprüngliche Planung sah einen eigenen VPN-Service (Valhalla) für Layer-3-Connectivity zwischen Devices vor; Bifrost sollte darüber laufen.

## Ersetzung durch Bifrost Device-Mesh

Statt VPN erweitert **Bifrost** um ein Meshtastic-inspiriertes **Device-Mesh**:

- **Ein Dienst**: Bifrost = Mesh-Layer (Membership, Discovery, Multi-Hop, Transports IP + optional LoRa) + Bifrost-Protokoll (Connection, Messages, Events)
- **Mesh-Membership** ersetzt VPN-Membership; „im gleichen User-Mesh“ statt „im gleichen VPN“
- Valhalla wird nicht umbenannt, sondern entfällt; alle Device-Kommunikation läuft über Bifrost

## Referenz

- Plan: Umplanung VPN → Device-Mesh in Bifrost
- Bifrost: [bifrost/README.md](../../../bifrost/README.md), Mesh-Layer und Bifrost-Protokoll
- Dieses Archiv dient der Nachvollziehbarkeit der früheren VPN-Planung.
