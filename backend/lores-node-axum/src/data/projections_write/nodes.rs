use sqlx::SqlitePool;

pub struct NodesWriteRepo {}

impl NodesWriteRepo {
    pub fn init() -> Self {
        NodesWriteRepo {}
    }

    pub async fn upsert_id(&self, pool: &SqlitePool, node_id: &String) -> Result<(), sqlx::Error> {
        let _node = sqlx::query!(
            "INSERT INTO nodes (id)
            VALUES (?)
            ON CONFLICT(id) DO NOTHING",
            node_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
