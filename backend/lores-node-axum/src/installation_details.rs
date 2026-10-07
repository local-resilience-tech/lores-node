use lores_p2panda::{RegionAdminTopic, RegionId};
use sqlx::{Pool, Sqlite};

use crate::{
    config::LoresNodeConfig,
    data::projections_write::lores_node_installations::LoresNodeInstallationsWriteRepo,
    panda_comms::{
        PandaContainer,
        lores_events::{LoResEventPayload::LoresNodeInstallChanged, LoresNodeInstallChangedDataV1},
    },
};

pub async fn handle_lores_installed_version_update(
    projections_pool: &Pool<Sqlite>,
    node_config: &LoresNodeConfig,
    panda_container: &PandaContainer,
) -> Result<(), String> {
    let node_id = match node_config.public_key_hex.clone() {
        Some(id) => id,
        None => return Err("No node_id in config".to_string()),
    };

    let current_lores_version = env!("CARGO_PKG_VERSION");

    if current_lores_version.is_empty() {
        return Ok(());
    }

    let lores_node_installations_write_repo = LoresNodeInstallationsWriteRepo::init();

    if let Err(e) = lores_node_installations_write_repo
        .upsert(projections_pool, &node_id, current_lores_version)
        .await
    {
        return Err(e.to_string());
    };

    let event_payload = LoresNodeInstallChanged(LoresNodeInstallChangedDataV1 {
        node_id,
        lores_version: current_lores_version.to_string(),
    });

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
