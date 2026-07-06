use lores_p2panda_server::{AppInstanceIds, ResolveRegionId, ResolveRegionIdError};
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::data::node_data::local_apps_repo::LocalAppsRepo;

/// Returns a [`lores_p2panda_server::ResolveRegionId`] callback that looks up
/// the `bound_to_region_id` for a given app/instance pair from the node_data
/// database.  Returns `Status::not_found` if no binding exists and
/// `Status::internal` if the stored value is malformed.
pub fn make_region_resolver(pool: SqlitePool) -> ResolveRegionId {
    Arc::new(move |ids: AppInstanceIds| {
        let pool = pool.clone();
        Box::pin(async move {
            let row = LocalAppsRepo::init()
                .find(&pool, &ids.app_id, &Some(ids.instance_id.clone()))
                .await
                .map_err(|e| {
                    eprintln!("[region_resolver] database error: {e}");
                    ResolveRegionIdError::Internal
                })?;

            let region_id_hex = row
                .and_then(|app| app.bound_to_region_id)
                .ok_or(ResolveRegionIdError::NotFound)?;

            lores_p2panda::RegionId::from_hex(&region_id_hex)
                .map_err(|_| {
                    eprintln!(
                        "[region_resolver] invalid region_id hex in database for app '{}' instance '{}': '{}'",
                        ids.app_id, ids.instance_id, region_id_hex
                    );
                    ResolveRegionIdError::Internal
                })
        })
    })
}
