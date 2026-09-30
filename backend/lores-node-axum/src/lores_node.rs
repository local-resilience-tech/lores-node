use lores_p2panda::{RegionAdminTopic, RegionId};
use sqlx::{Pool, Sqlite};

use crate::{
    config::LoresNodeConfig,
    data::{entities::LoresNode, node_data::lores_node_repo::LoresNodeRepo},
    panda_comms::{
        PandaContainer,
        lores_events::{LoResEventPayload::LoresNodeInstallChanged, LoresNodeInstallChangedDataV1},
    },
};

pub async fn handle_lores_installed_version_update(
    node_data_pool: &Pool<Sqlite>,
    node_config: &LoresNodeConfig,
    panda_container: &PandaContainer,
) {
    let _node_id = match node_config.public_key_hex.clone() {
        Some(id) => id,
        None => {
            tracing::info!("No node_id provided in config");
            return;
        }
    };

    let current_lores_version = env!("CARGO_PKG_VERSION");
    let lores_repo = LoresNodeRepo::init();
    let current_version = lores_repo.find(node_data_pool, &_node_id).await;

    let repo_result = match current_version {
        Ok(result) => result,
        Err(e) => {
            tracing::info!("Error fetching current node version from DB: {}", e);
            return;
        }
    };

    match repo_result {
        Some(node) => {
            if node.node_id == _node_id {
                // current version is already persisted
                return;
            }
        }
        _ => {}
    };

    let node = LoresNode {
        node_id: _node_id,
        lores_version: if current_lores_version.is_empty() {
            None
        } else {
            Some(current_lores_version.to_string())
        },
    };

    lores_repo.upsert(&node_data_pool, &node).await;

    if node.lores_version.is_some() && node_config.region_ids.is_some() {
        let event_payload = LoresNodeInstallChanged(LoresNodeInstallChangedDataV1 {
            node_id: node.node_id,
            lores_version: node.lores_version.unwrap(),
        });

        for region in node_config.region_ids.as_ref().unwrap() {
            let region_id = RegionId::from_hex(region.as_str());

            match region_id {
                Ok(id) => {
                    panda_container
                        .publish_persisted(&RegionAdminTopic::new(id), event_payload.clone(), None)
                        .await;
                }
                Err(e) => {
                    tracing::error!("Error converting region {} into region_id: {}", region, e);
                }
            }
        }
    }
}

// to do add a test
