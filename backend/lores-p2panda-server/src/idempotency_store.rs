use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lores_p2panda::{RegionAppTopic, RegionTopic};
use sqlx::SqlitePool;
use tokio::time::interval;
use tonic::Status;

pub struct IdempotencyStore {
    db: SqlitePool,
}

impl IdempotencyStore {
    pub async fn new(
        db: SqlitePool,
        cleanup_frequency: Duration,
        retention: Duration,
    ) -> Result<Self, sqlx::Error> {
        Self::setup_table(&db).await?;
        Self::spawn_cleanup(&db, cleanup_frequency, retention);
        Ok(Self { db })
    }

    /// Returns `true` if the idempotency key has already been recorded for
    /// this topic. Returns `false` immediately when no key is supplied.
    pub async fn is_duplicate(
        &self,
        region_app_topic: &RegionAppTopic,
        idempotency_key: &[u8],
    ) -> Result<bool, Status> {
        if idempotency_key.is_empty() {
            return Ok(false);
        }

        let topic_bytes = region_app_topic.p2panda_topic().to_bytes().to_vec();
        let exists: Option<(i64,)> =
            sqlx::query_as("SELECT 1 FROM publish_idempotency_keys WHERE topic = ? AND key = ?")
                .bind(&topic_bytes)
                .bind(idempotency_key)
                .fetch_optional(&self.db)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        Ok(exists.is_some())
    }

    /// Records an idempotency key as processed. No-op when the key is empty.
    pub async fn record(
        &self,
        region_app_topic: &RegionAppTopic,
        idempotency_key: &[u8],
    ) -> Result<(), Status> {
        if idempotency_key.is_empty() {
            return Ok(());
        }

        let topic_bytes = region_app_topic.p2panda_topic().to_bytes().to_vec();
        let seen_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        sqlx::query(
            "INSERT OR IGNORE INTO publish_idempotency_keys (topic, key, seen_at)
             VALUES (?, ?, ?)",
        )
        .bind(&topic_bytes)
        .bind(idempotency_key)
        .bind(seen_at)
        .execute(&self.db)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(())
    }

    async fn setup_table(db: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS publish_idempotency_keys (
                topic   BLOB    NOT NULL,
                key     BLOB    NOT NULL,
                seen_at INTEGER NOT NULL,
                PRIMARY KEY (topic, key)
            );
            CREATE INDEX IF NOT EXISTS idx_pik_seen_at
                ON publish_idempotency_keys(seen_at);",
        )
        .execute(db)
        .await?;
        Ok(())
    }

    fn spawn_cleanup(db: &SqlitePool, cleanup_frequency: Duration, retention: Duration) {
        let db = db.clone();
        let retention_secs = retention.as_secs() as i64;

        tokio::spawn(async move {
            let mut timer = interval(cleanup_frequency);
            loop {
                timer.tick().await;
                let cutoff = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
                    - retention_secs;
                let _ = sqlx::query("DELETE FROM publish_idempotency_keys WHERE seen_at < ?")
                    .bind(cutoff)
                    .execute(&db)
                    .await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lores_p2panda::{RegionAppTopic, RegionId};
    use sqlx::SqlitePool;

    async fn test_store() -> IdempotencyStore {
        let db = SqlitePool::connect("sqlite::memory:").await.unwrap();
        // Use a very long cleanup frequency so the background task never fires during tests.
        IdempotencyStore::new(
            db,
            Duration::from_secs(u64::MAX / 2),
            Duration::from_hours(48),
        )
        .await
        .unwrap()
    }

    fn topic(namespace: &str) -> RegionAppTopic {
        RegionAppTopic::new(RegionId::from([1u8; 32]), namespace)
    }

    // 1. is_duplicate returns false when key has not been recorded.
    #[tokio::test]
    async fn test_unknown_key_is_not_duplicate() {
        let store = test_store().await;
        let result = store.is_duplicate(&topic("app"), b"key-1").await.unwrap();
        assert!(!result);
    }

    // 2. is_duplicate returns false when no key is supplied.
    #[tokio::test]
    async fn test_empty_key_is_not_duplicate() {
        let store = test_store().await;
        let result = store.is_duplicate(&topic("app"), b"").await.unwrap();
        assert!(!result);
    }

    // 3. After record, is_duplicate returns true for the same key and topic.
    #[tokio::test]
    async fn test_recorded_key_is_duplicate() {
        let store = test_store().await;
        store.record(&topic("app"), b"key-1").await.unwrap();
        let result = store.is_duplicate(&topic("app"), b"key-1").await.unwrap();
        assert!(result);
    }

    // 4. Calling record twice with the same key does not error.
    #[tokio::test]
    async fn test_record_is_idempotent() {
        let store = test_store().await;
        store.record(&topic("app"), b"key-1").await.unwrap();
        store.record(&topic("app"), b"key-1").await.unwrap();
    }

    // 5. After manual cleanup removes an expired row, is_duplicate returns false.
    #[tokio::test]
    async fn test_expired_key_removed_by_cleanup() {
        let store = test_store().await;

        // Insert a row with seen_at in the distant past.
        let topic = topic("app");
        let topic_bytes = topic.p2panda_topic().to_bytes().to_vec();
        sqlx::query("INSERT INTO publish_idempotency_keys (topic, key, seen_at) VALUES (?, ?, ?)")
            .bind(&topic_bytes)
            .bind(b"old-key".as_slice())
            .bind(0i64) // epoch — definitely expired
            .execute(&store.db)
            .await
            .unwrap();

        // Confirm it looks like a duplicate before cleanup.
        assert!(store.is_duplicate(&topic, b"old-key").await.unwrap());

        // Run cleanup with a cutoff of now (removes anything seen_at < now).
        let cutoff = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        sqlx::query("DELETE FROM publish_idempotency_keys WHERE seen_at < ?")
            .bind(cutoff)
            .execute(&store.db)
            .await
            .unwrap();

        // Now the key should be gone.
        assert!(!store.is_duplicate(&topic, b"old-key").await.unwrap());
    }

    // 6. Keys are scoped by topic: the same key on different topics is independent.
    #[tokio::test]
    async fn test_keys_are_scoped_by_topic() {
        let store = test_store().await;
        let topic_a = topic("app-a");
        let topic_b = topic("app-b");

        store.record(&topic_a, b"key-1").await.unwrap();

        assert!(store.is_duplicate(&topic_a, b"key-1").await.unwrap());
        assert!(!store.is_duplicate(&topic_b, b"key-1").await.unwrap());
    }
}
