use anyhow::Result;
use p2panda_core::cbor::{decode_cbor, encode_cbor, DecodeError, EncodeError};

use super::lores_events::{
    LoResEvent, LoResEventHeader, LoResEventMetadataV1, LoResEventPayload,
    LoResPossibleEventPayload, LoResWirePayload,
};

pub fn encode_lores_event_payload(
    event_payload: LoResEventPayload,
    metadata: LoResEventMetadataV1,
) -> Result<Vec<u8>, EncodeError> {
    let wire_payload = LoResWirePayload {
        metadata,
        event_payload: LoResPossibleEventPayload::LoResEventPayload(event_payload),
    };

    encode_lores_wire_event(wire_payload)
}

fn encode_lores_wire_event(wire_event: LoResWirePayload) -> Result<Vec<u8>, EncodeError> {
    encode_cbor(&wire_event)
}

#[allow(dead_code)]
fn decode_lores_wire_event(encoded_payload: &[u8]) -> Result<LoResWirePayload, DecodeError> {
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

#[allow(dead_code)]
pub fn decode_lores_event_payload(
    encoded_payload: &[u8],
) -> Result<LoResEventPayload, anyhow::Error> {
    let wire_event: LoResWirePayload = decode_lores_wire_event(encoded_payload)?;

    match wire_event.event_payload {
        LoResPossibleEventPayload::LoResEventPayload(payload) => Ok(payload),
        LoResPossibleEventPayload::DeprecatedLoResEventPayload(_) => {
            println!("Received deprecated LoResEventPayload, which is no longer supported.");
            Err(anyhow::anyhow!(
                "Received deprecated LoResEventPayload, which is no longer supported."
            ))
        }
    }
}

#[allow(dead_code)]
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

        let metadata = LoResEventMetadataV1 {
            node_steward_id: None,
        };

        let encoded = encode_lores_event_payload(payload.clone(), metadata).unwrap();
        let decoded = decode_lores_event_payload(&encoded).unwrap();

        assert_eq!(payload, decoded);
    }
}
