use std::sync::Arc;

use lores_p2panda::RegionId;
use sqlx::SqlitePool;

use crate::data::node_data::app_instances_repo::AppInstancesRepo;

/// Returns a callback suitable for passing to `PandaService::new` that records
/// each newly-seen app instance into the node_data database.
pub fn make_instance_seen_callback(
    pool: SqlitePool,
) -> Arc<dyn Fn(String, Vec<u8>, RegionId) + Send + Sync> {
    Arc::new(
        move |app_namespace: String, instance_id: Vec<u8>, region_id: RegionId| {
            let pool = pool.clone();
            let region_hex = region_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = AppInstancesRepo::init()
                    .record_app_instance(&pool, &app_namespace, &instance_id, &region_hex)
                    .await
                {
                    eprintln!("[app_instances] failed to record instance: {}", e);
                }
            });
        },
    )
}
