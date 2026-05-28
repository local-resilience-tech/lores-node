mod config;
mod event_encoding;
pub mod lores_events;
mod panda_container;

pub use config::ThisP2PandaNodeRepo;
use lores_events::LoResEvent;
pub use lores_p2panda::RegionAdminTopic;
pub use lores_p2panda::RegionId;
pub use panda_container::{build_public_key_from_hex, PandaContainer};
use sqlx::SqlitePool;
use tokio::sync::mpsc;

use crate::{
    api::public_api::realtime::RealtimeState,
    config::config_state::LoresNodeConfigState,
    data::{projections_write::nodes::NodesWriteRepo, setup::OPERATION_DATABASE_URL},
    event_handlers::handle_event,
};

pub async fn start_panda(
    config_state: &LoresNodeConfigState,
    container: &PandaContainer,
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

    let public_key = private_key.verifying_key();

    NodesWriteRepo::init()
        .upsert_id(projections_pool, &public_key.to_hex())
        .await
        .unwrap_or_else(|e| {
            println!("Failed to upsert node id: {:?}", e);
        });

    container.set_private_key(private_key).await;

    let bootstrap_node_ids = repo.get_bootstrap_node_ids(config_state).await;
    container.set_bootstrap_node_ids(bootstrap_node_ids).await;

    if let Err(e) = container.start(OPERATION_DATABASE_URL.as_str()).await {
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
