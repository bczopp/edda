//! MeshPacket/Data serialization tests (Phase 11.0.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::mesh::{packet::MeshPacket, transport, Data, BROADCAST_NODE_ID};
use serde_json;

#[test]
fn mesh_packet_roundtrip() {
    let pkt = MeshPacket {
        from: 1,
        to: 2,
        hop_limit: 3,
        hop_start: 3,
        want_ack: true,
        id: 100,
        channel: 0,
        payload: vec![1, 2, 3],
    };
    let json = serde_json::to_string(&pkt).unwrap();
    let back: MeshPacket = serde_json::from_str(&json).unwrap();
    assert_eq!(back.from, pkt.from);
    assert_eq!(back.to, pkt.to);
    assert_eq!(back.hop_limit, pkt.hop_limit);
    assert_eq!(back.payload, pkt.payload);
}

#[test]
fn mesh_packet_broadcast_to_is_max_u32() {
    assert_eq!(BROADCAST_NODE_ID, 0xFFFF_FFFF);
}

#[test]
fn data_roundtrip() {
    let data = Data {
        portnum: 1,
        payload: vec![10, 20],
        dest: Some(2),
        source: Some(1),
        request_id: None,
        reply_id: None,
        want_response: false,
    };
    let json = serde_json::to_string(&data).unwrap();
    let back: Data = serde_json::from_str(&json).unwrap();
    assert_eq!(back.portnum, data.portnum);
    assert_eq!(back.payload, data.payload);
    assert_eq!(back.dest, data.dest);
}

#[test]
fn transport_encode_decode_roundtrip() {
    let pkt = MeshPacket {
        from: 1,
        to: BROADCAST_NODE_ID,
        hop_limit: 2,
        hop_start: 2,
        want_ack: false,
        id: 42,
        channel: 0,
        payload: vec![1, 2, 3],
    };
    let encoded = transport::encode_mesh_packet(&pkt).unwrap();
    let decoded = transport::decode_mesh_packet(&encoded).unwrap();
    assert_eq!(decoded.from, pkt.from);
    assert_eq!(decoded.to, pkt.to);
    assert_eq!(decoded.hop_limit, pkt.hop_limit);
}
