use lores_p2panda_server::{AppInstanceIds, ResolveRegionId, ResolveRegionIdError, ResolvedRegion};
use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::warn;

use crate::data::node_data::local_apps_repo::LocalAppsRepo;
use crate::data::projections_read::regions::RegionsReadRepo;

/// Returns a [`lores_p2panda_server::ResolveRegionId`] callback that looks up
/// the `bound_to_region_id` for a given app/instance pair from the node_data
/// database, then enriches with slug/name from the projections database.
pub fn make_region_resolver(node_data_pool: SqlitePool, projections_pool: SqlitePool) -> ResolveRegionId {
    Arc::new(move |ids: AppInstanceIds| {
        let node_data_pool = node_data_pool.clone();
        let projections_pool = projections_pool.clone();
        Box::pin(async move {
            let row = LocalAppsRepo::init()
                .find(&node_data_pool, &ids.app_id, &Some(ids.instance_id.clone()))
                .await
                .map_err(|e| {
                    warn!("[region_resolver] database error: {e}");
                    ResolveRegionIdError::Internal
                })?;

            let region_id_hex = row.and_then(|app| app.bound_to_region_id).ok_or(ResolveRegionIdError::NotFound)?;

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
