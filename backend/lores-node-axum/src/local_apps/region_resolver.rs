use lores_p2panda_server::{AppInstanceIds, NodeInfo, ResolveNodeInfo, ResolveRegionId, ResolveRegionIdError, ResolvedRegion};
use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::warn;

use crate::data::node_data::local_apps_repo::LocalAppsRepo;
use crate::data::projections_read::region_nodes::RegionNodesReadRepo;
use crate::data::projections_read::regions::RegionsReadRepo;

async fn resolve_region_id_hex(pool: &SqlitePool, ids: &AppInstanceIds) -> Result<String, ResolveRegionIdError> {
    let row = LocalAppsRepo::init()
        .find(pool, &ids.app_id, &Some(ids.instance_id.clone()))
        .await
        .map_err(|e| {
            warn!("[region_resolver] database error: {e}");
            ResolveRegionIdError::Internal
        })?;

    row.and_then(|app| app.bound_to_region_id).ok_or(ResolveRegionIdError::NotFound)
}

/// Returns a [`lores_p2panda_server::ResolveRegionId`] callback that looks up
/// the `bound_to_region_id` for a given app/instance pair from the node_data
/// database, then enriches with slug/name from the projections database.
pub fn make_region_resolver(node_data_pool: SqlitePool, projections_pool: SqlitePool) -> ResolveRegionId {
    Arc::new(move |ids: AppInstanceIds| {
        let node_data_pool = node_data_pool.clone();
        let projections_pool = projections_pool.clone();
        Box::pin(async move {
            let region_id_hex = resolve_region_id_hex(&node_data_pool, &ids).await?;

            let region_id = lores_p2panda::RegionId::from_hex(&region_id_hex).map_err(|_| {
                warn!(
                    "[region_resolver] invalid region_id hex in database for app '{}' instance '{}': '{}'",
                    ids.app_id, ids.instance_id, region_id_hex
                );
                ResolveRegionIdError::Internal
            })?;

            let region = RegionsReadRepo::init()
                .find(&projections_pool, &region_id_hex)
                .await
                .map_err(|e| {
                    warn!("[region_resolver] projections database error: {e}");
                    ResolveRegionIdError::Internal
                })?;

            Ok(ResolvedRegion {
                region_id,
                slug: region.as_ref().and_then(|r| r.slug.clone()),
                name: region.as_ref().and_then(|r| r.name.clone()),
            })
        })
    })
}

/// Returns a [`lores_p2panda_server::ResolveNodeInfo`] callback that looks up
/// node metadata from the projections database, scoped to the caller's region.
pub fn make_node_resolver(node_data_pool: SqlitePool, projections_pool: SqlitePool) -> ResolveNodeInfo {
    Arc::new(move |ids: AppInstanceIds, node_id: String| {
        let node_data_pool = node_data_pool.clone();
        let projections_pool = projections_pool.clone();
        Box::pin(async move {
            let region_id_hex = resolve_region_id_hex(&node_data_pool, &ids).await?;

            let node = RegionNodesReadRepo::init()
                .find_by_keys(&projections_pool, &node_id, &region_id_hex)
                .await
                .map_err(|e| {
                    warn!("[node_resolver] projections database error: {e}");
                    ResolveRegionIdError::Internal
                })?
                .ok_or(ResolveRegionIdError::NotFound)?;

            Ok(NodeInfo {
                node_id: node.node_id,
                name: node.name,
                domain_on_internet: node.domain_on_internet,
            })
        })
    })
}
