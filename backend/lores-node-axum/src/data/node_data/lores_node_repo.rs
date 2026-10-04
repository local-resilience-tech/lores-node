use sqlx::{Sqlite, SqlitePool};

use crate::data::entities::LoresNode;

#[derive(sqlx::FromRow)]
struct LoresNodeRow {
    pub node_id: String,
    pub lores_version: Option<String>,
}

impl From<LoresNodeRow> for LoresNode {
    fn from(row: LoresNodeRow) -> Self {
        Self {
            node_id: row.node_id,
            lores_version: row.lores_version,
        }
    }
}

pub struct LoresNodeRepo {}

impl LoresNodeRepo {
    pub fn init() -> Self {
        LoresNodeRepo {}
    }

    pub async fn upsert(&self, pool: &SqlitePool, node: &LoresNode) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO lores_node (node_id, lores_version)
            VALUES(?, ?)
            ON CONFLICT(node_id) DO UPDATE SET lores_version = excluded.lores_version
            ",
        )
        .bind(&node.node_id)
        .bind(&node.lores_version)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find(&self, pool: &SqlitePool, node_id: &String) -> Result<Option<LoresNode>, sqlx::Error> {
        let row = sqlx::query_as::<Sqlite, LoresNodeRow>(
            "
            SELECT node_id, lores_version
            FROM lores_node
            WHERE node_id = ?
            ",
        )
        .bind(node_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(LoresNode::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations_nodedatadb")]
    async fn lores_node_repo_can_upsert_and_query(pool: SqlitePool) -> () {
        let repo = LoresNodeRepo::init();
        let node_id = "test_node_id".to_string();

        let result = repo.find(&pool, &node_id).await.unwrap();
        assert_eq!(&result.is_none(), &true);

        let mut node = LoresNode {
            node_id: node_id.clone(),
            lores_version: None,
        };

        repo.upsert(&pool, &node).await.unwrap();

        let result = repo.find(&pool, &node_id).await.unwrap().unwrap();
        assert_eq!(&result.lores_version.is_none(), &true);
        assert_eq!(&result.node_id, &node_id);

        node.lores_version = Some("0.23.0".to_string());
        repo.upsert(&pool, &node).await.unwrap();

        let result = repo.find(&pool, &node_id).await.unwrap().unwrap();
        assert_eq!(&result.lores_version, &Some("0.23.0".to_string()));
    }
}
