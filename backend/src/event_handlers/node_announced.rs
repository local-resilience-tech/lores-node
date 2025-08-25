use sqlx::{Sqlite, SqlitePool};

use crate::{
    data::{entities::Node, projections_write::nodes::NodesWriteRepo},
    event_handlers::handler_utilities::{
        handle_db_write_error, read_node_updated_event, HandlerResult,
    },
    panda_comms::lores_events::{LoResEventHeader, NodeAnnouncedDataV1},
};

pub struct NodeAnnouncedHandler {}

impl NodeAnnouncedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeAnnouncedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
        println!("Node announced: {:?}", payload);

        let author_node_id = header.author_node_id.clone();
        let result = Self::write_projections(header, payload, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(pool, author_node_id).await,
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn write_projections(
        header: LoResEventHeader,
        payload: NodeAnnouncedDataV1,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = NodesWriteRepo::init();

        let node = Node {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
        };
        repo.upsert(pool, node).await?;
        Ok(())
    }
}
