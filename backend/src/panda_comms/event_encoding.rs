use anyhow::Result;

use super::lores_events::{LoResEvent, LoResEventHeader, LoResEventPayload};

pub fn encode_lores_event_payload(event_payload: LoResEventPayload) -> Result<Vec<u8>> {
    let encoded_payload: Vec<u8> = serde_json::to_vec(&event_payload)?;

    Ok(encoded_payload)
}

pub fn decode_lores_event_payload(encoded_payload: &[u8]) -> Result<LoResEventPayload> {
    let result: Result<LoResEventPayload, _> = serde_json::from_slice(encoded_payload);
    match result {
        Ok(decoded_payload) => {
            // Successfully decoded
            return Ok(decoded_payload);
        }
        Err(e) => {
            // Handle the error
            log::error!("Failed to decode payload: {}", e);
            return Err(anyhow::anyhow!("Failed to decode payload: {}", e));
        }
    }
}

pub fn decode_lores_event(author_node_id: String, encoded_payload: &[u8]) -> Result<LoResEvent> {
    let decoded_payload: LoResEventPayload = decode_lores_event_payload(encoded_payload)?;

    let header = LoResEventHeader { author_node_id };

    let lores_event = LoResEvent {
        header,
        payload: decoded_payload,
    };

    println!("  Parsed LoResEvent: {:?}", lores_event);

    Ok(lores_event)
}
