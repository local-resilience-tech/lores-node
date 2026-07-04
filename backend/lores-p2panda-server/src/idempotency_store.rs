use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lores_p2panda::{RegionAppTopic, RegionTopic};
use sqlx::SqlitePool;
use tokio::time::interval;
use tonic::Status;

pub struct IdempotencyStore {
    db: SqlitePool,
}

impl IdempotencyStore {
    pub async fn new(db: SqlitePool) -> Result<Self, sqlx::Error> {
        Self::setup_table(&db).await?;
        Self::spawn_cleanup(&db);
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

    fn spawn_cleanup(db: &SqlitePool) {
        let db = db.clone();
        const CLEANUP_FREQUENCY: Duration = Duration::from_hours(12);
        const RETENTION_SECS: i64 = Duration::from_hours(48).as_secs() as i64;

        tokio::spawn(async move {
            let mut timer = interval(CLEANUP_FREQUENCY);
            loop {
                timer.tick().await;
                let cutoff = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
                    - RETENTION_SECS;
                let _ = sqlx::query("DELETE FROM publish_idempotency_keys WHERE seen_at < ?")
                    .bind(cutoff)
                    .execute(&db)
                    .await;
            }
        });
    }
}
