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
) -> Result<(), String> {
    let node_id = match node_config.public_key_hex.clone() {
        Some(id) => id,
        None => return Err("No node_id in config".to_string()),
    };

    let current_lores_version = env!("CARGO_PKG_VERSION");
    let lores_repo = LoresNodeRepo::init();
    let current_version = lores_repo.find(node_data_pool, &node_id).await;

    let repo_result = match current_version {
        Ok(result) => result,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if let Some(node) = repo_result
        && node.node_id == node_id
    {
        // current version is already persisted
        // nothing to do
        return Ok(());
    };

    let node = LoresNode {
        node_id,
        lores_version: if current_lores_version.is_empty() {
            None
        } else {
            Some(current_lores_version.to_string())
        },
    };

    if let Err(e) = lores_repo.upsert(node_data_pool, &node).await {
        return Err(e.to_string());
    };

    let event_payload = match node.lores_version {
        Some(lores_version) => LoresNodeInstallChanged(LoresNodeInstallChangedDataV1 {
            node_id: node.node_id,
            lores_version,
        }),
        None => return Ok(()),
    };

    if let Some(region_ids) = &node_config.region_ids {
        for region in region_ids {
            let region_id = RegionId::from_hex(region.as_str());

            match region_id {
                Ok(id) => {
                    if let Err(e) = panda_container
                        .publish_persisted(&RegionAdminTopic::new(id), event_payload.clone(), None)
                        .await
                    {
                        tracing::error!("Issue publishing lores installed version: {}", e);
                    };
                }
                Err(e) => {
                    tracing::error!("Issue retrieving region_id {}: {}", region.as_str(), e);
                }
            }
        }
    }

    Ok(())
}
