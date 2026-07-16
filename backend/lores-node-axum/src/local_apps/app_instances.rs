use sqlx::SqlitePool;
use tracing::warn;
use std::sync::Arc;

use crate::data::node_data::app_instances_repo::AppInstancesRepo;

/// Returns a callback suitable for passing to `PandaService::new` that records
/// each newly-seen app instance into the node_data database.
pub fn make_instance_seen_callback(pool: SqlitePool) -> Arc<dyn Fn(String, String) + Send + Sync> {
    Arc::new(move |app_id: String, instance_id: String| {
        let pool = pool.clone();
        tokio::spawn(async move {
            if let Err(e) = AppInstancesRepo::init()
                .record_app_instance(&pool, &app_id, &instance_id)
                .await
            {
                warn!("[app_instances] failed to record instance: {}", e);
            }
        });
    })
}
