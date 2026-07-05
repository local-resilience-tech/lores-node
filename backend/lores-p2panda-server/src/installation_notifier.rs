use std::collections::HashSet;
use std::sync::Arc;

use lores_p2panda::RegionId;
use tokio::sync::RwLock;

/// Fires a callback the first time each unique
/// (app_namespace, installation_id, region_id) combination is seen within
/// this process lifetime. Subsequent calls for the same combination are
/// silently ignored.
pub struct InstallationNotifier {
    callback: Arc<dyn Fn(String, Vec<u8>, RegionId) + Send + Sync>,
    seen: RwLock<HashSet<(String, Vec<u8>, Vec<u8>)>>,
}

impl InstallationNotifier {
    pub fn new(callback: Arc<dyn Fn(String, Vec<u8>, RegionId) + Send + Sync>) -> Self {
        Self {
            callback,
            seen: RwLock::new(HashSet::new()),
        }
    }

    /// Notify at most once per unique (app_namespace, installation_id, region_id).
    pub async fn notify(&self, app_namespace: &str, installation_id: &[u8], region_id: &RegionId) {
        let key = (
            app_namespace.to_owned(),
            installation_id.to_owned(),
            region_id.to_bytes().to_vec(),
        );
        {
            if self.seen.read().await.contains(&key) {
                return;
            }
        }
        let mut seen = self.seen.write().await;
        if seen.insert(key) {
            (self.callback)(
                app_namespace.to_owned(),
                installation_id.to_owned(),
                region_id.clone(),
            );
        }
    }
}
