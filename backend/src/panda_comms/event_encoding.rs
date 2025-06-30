use anyhow::Result;
use p2panda_core::cbor::{decode_cbor, encode_cbor, DecodeError, EncodeError};

use super::lores_events::{LoResEvent, LoResEventHeader, LoResEventPayload, LoResWireEvent};

pub fn encode_lores_event_payload(
    event_payload: LoResEventPayload,
) -> Result<Vec<u8>, EncodeError> {
    encode_lores_wire_event(LoResWireEvent::LoResEventPayload(event_payload))
}

fn encode_lores_wire_event(wire_event: LoResWireEvent) -> Result<Vec<u8>, EncodeError> {
    encode_cbor(&wire_event)
}

fn decode_lores_wire_event(encoded_payload: &[u8]) -> Result<LoResWireEvent, DecodeError> {
    let result = decode_cbor(encoded_payload);

    match result {
        Ok(decoded_payload) => {
            // Successfully decoded
            return Ok(decoded_payload);
        }
        Err(e) => {
            // Handle the error
            eprintln!("Failed to decode payload: {}", e);
            return Err(e);
        }
    }
}

pub fn decode_lores_event_payload(
    encoded_payload: &[u8],
) -> Result<LoResEventPayload, anyhow::Error> {
    let wire_event: LoResWireEvent = decode_lores_wire_event(encoded_payload)?;

    match wire_event {
        LoResWireEvent::LoResEventPayload(payload) => Ok(payload),
        LoResWireEvent::DeprecatedLoResEventPayload(_) => {
            println!("Received deprecated LoResEventPayload, which is no longer supported.");
            Err(anyhow::anyhow!(
                "Received deprecated LoResEventPayload, which is no longer supported."
            ))
        }
    }
}

pub fn decode_lores_event(
    header: LoResEventHeader,
    encoded_payload: &[u8],
) -> Result<LoResEvent, anyhow::Error> {
    let decoded_payload: LoResEventPayload = decode_lores_event_payload(encoded_payload)?;

    let lores_event = LoResEvent {
        header,
        payload: decoded_payload,
    };

    println!("  Parsed LoResEvent: {:?}", lores_event);

    Ok(lores_event)
}

// Test encoding and decoding
#[cfg(test)]
mod tests {
    use super::*;
    use crate::panda_comms::lores_events::{LoResEventPayload, NodeAnnouncedDataV1};

    #[test]
    fn test_encode_decode_lores_event_payload() {
        let payload = LoResEventPayload::NodeAnnounced(NodeAnnouncedDataV1 {
            name: "Test Node".to_string(),
        });

        let encoded = encode_lores_event_payload(payload.clone()).unwrap();
        let decoded = decode_lores_event_payload(&encoded).unwrap();

        assert_eq!(payload, decoded);
    }
}
