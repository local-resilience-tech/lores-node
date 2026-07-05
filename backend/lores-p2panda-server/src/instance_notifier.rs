use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::RwLock;

/// Fires a callback the first time each unique (app_id, instance_id) combination
/// is seen within this process lifetime. Subsequent calls for the same combination
/// are silently ignored.
pub struct InstanceNotifier {
    callback: Arc<dyn Fn(String, String) + Send + Sync>,
    seen: RwLock<HashSet<(String, String)>>,
}

impl InstanceNotifier {
    pub fn new(callback: Arc<dyn Fn(String, String) + Send + Sync>) -> Self {
        Self {
            callback,
            seen: RwLock::new(HashSet::new()),
        }
    }

    /// Notify at most once per unique (app_id, instance_id).
    pub async fn notify(&self, app_id: &str, instance_id: &str) {
        let key = (app_id.to_owned(), instance_id.to_owned());
        {
            if self.seen.read().await.contains(&key) {
                return;
            }
        }
        let mut seen = self.seen.write().await;
        if seen.insert(key) {
            (self.callback)(app_id.to_owned(), instance_id.to_owned());
        }
    }
}
