mod config;
mod event_encoding;
pub mod lores_events;
mod panda_container;
use std::fmt::Display;

pub use config::ThisP2PandaNodeRepo;
use hex::FromHexError;
use lores_events::LoResEvent;
use lores_p2panda::p2panda_core::PublicKey;
pub use panda_container::{build_public_key_from_hex, PandaContainer};
use sqlx::SqlitePool;
use tokio::sync::mpsc;

use crate::{
    api::public_api::realtime::RealtimeState, config::config_state::LoresNodeConfigState,
    data::projections_write::nodes::NodesWriteRepo, event_handlers::handle_event,
};

pub async fn start_panda(
    config_state: &LoresNodeConfigState,
    container: &PandaContainer,
    operations_pool: &SqlitePool,
    projections_pool: &SqlitePool,
) {
    let repo = ThisP2PandaNodeRepo::init();
    let config = config_state.get().await;

    match config.network_name.clone() {
        Some(network_name) => {
            println!("Using network name: {:?}", network_name);
            container.set_network_name(network_name.clone()).await;
        }
        None => {
            println!("No network name set");
        }
    }

    let private_key = match repo.get_or_create_private_key(config_state).await {
        Ok(key) => key,
        Err(e) => {
            println!("Failed to get or create private key: {:?}", e);
            return;
        }
    };

    let public_key = private_key.public_key();

    NodesWriteRepo::init()
        .upsert_id(projections_pool, &public_key.to_hex())
        .await
        .unwrap_or_else(|e| {
            println!("Failed to upsert node id: {:?}", e);
        });

    container.set_private_key(private_key).await;

    let bootstrap_node_ids = repo.get_bootstrap_node_ids(config_state).await;
    container.set_bootstrap_node_ids(bootstrap_node_ids).await;

    if let Err(e) = container.start(operations_pool).await {
        println!("Failed to start P2PandaContainer on liftoff: {:?}", e);
    }

    match config.region_ids {
        Some(region_ids) => {
            for id_string in region_ids {
                match RegionId::from_hex(&id_string) {
                    Ok(region_id) => {
                        if let Err(e) = container.join_region(region_id.clone()).await {
                            println!("Failed to join region {}: {:?}", region_id, e);
                        } else {
                            println!("Successfully joined region {}", region_id);
                        }
                    }
                    Err(e) => {
                        println!(
                            "Invalid region id in config: {:?}, error: {:?}",
                            id_string, e
                        );
                    }
                }
            }
        }
        None => {
            println!("No region ids set");
        }
    }
}

pub fn start_panda_event_handler(
    channel_rx: mpsc::Receiver<LoResEvent>,
    pool: SqlitePool,
    realtime_state: RealtimeState,
) {
    tokio::spawn(async move {
        let mut events_rx = channel_rx;

        // Start the event loop to handle events
        while let Some(event) = events_rx.recv().await {
            handle_event(event, &pool, &realtime_state).await;
        }
    });
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegionId {
    bytes: [u8; 32],
}

impl From<RegionId> for [u8; 32] {
    fn from(id: RegionId) -> Self {
        id.bytes
    }
}

impl From<[u8; 32]> for RegionId {
    fn from(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }
}

impl Display for RegionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl RegionId {
    pub fn from_hex(value: &str) -> Result<RegionId, FromHexError> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(value, &mut bytes as &mut [u8])?;

        Ok(RegionId { bytes })
    }

    pub fn generate() -> Self {
        let mut arr = [0u8; 32];
        rand::fill(&mut arr[..]);
        RegionId { bytes: arr }
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.bytes)
    }
}
