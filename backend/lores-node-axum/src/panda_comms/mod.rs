mod config;
mod event_encoding;
pub mod lores_events;
mod panda_container;
pub use config::ThisP2PandaNodeRepo;
use lores_events::LoResEvent;
use lores_p2panda::{p2panda_core::PublicKey, TopicId};
pub use panda_container::{build_public_key_from_hex, PandaContainer};
use sqlx::SqlitePool;
use tokio::sync::mpsc;

use crate::{
    api::public_api::realtime::RealtimeState, config::config_state::LoresNodeConfigState,
    event_handlers::handle_event,
};

pub const NODE_ADMIN_TOPIC_ID: TopicId = [0u8; 32];

pub async fn start_panda(
    config_state: &LoresNodeConfigState,
    container: &PandaContainer,
    operations_pool: &SqlitePool,
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

    container.set_private_key(private_key).await;

    let bootstrap_details = repo.get_bootstrap_details(config_state).await;
    let bootstrap_node_id: Option<PublicKey> = match &bootstrap_details {
        Some(details) => build_public_key_from_hex(details.node_id.clone()),
        None => None,
    };
    container.set_bootstrap_node_id(bootstrap_node_id).await;

    if let Err(e) = container.start(operations_pool).await {
        println!("Failed to start P2PandaContainer on liftoff: {:?}", e);
    }

    match config.region_ids {
        Some(region_ids) => {
            for id in region_ids {
                if let Err(e) = container.join_region(id.clone()).await {
                    println!("Failed to join region {}: {:?}", id, e);
                } else {
                    println!("Successfully joined region {}", id);
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
