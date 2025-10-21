use sqlx::{Sqlite, SqlitePool};

use crate::{
    data::{entities::Node, projections_write::nodes::NodesWriteRepo},
    event_handlers::handler_utilities::{
        handle_db_write_error, read_node_updated_event, HandlerResult,
    },
    panda_comms::lores_events::{LoResEventHeader, NodeUpdatedDataV1},
};

pub struct NodeUpdatedHandler {}

impl NodeUpdatedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeUpdatedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
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
        payload: NodeUpdatedDataV1,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = NodesWriteRepo::init();

        println!("Node updated: {:?}", payload);

        // Upsert the node for now. This wouldn't be needed if we had a preserved message log.
        let node = Node {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
            public_ipv4: Some(payload.public_ipv4.clone()),
            domain_on_local_network: payload.domain_on_local_network.clone(),
            domain_on_internet: payload.domain_on_internet.clone(),
        };
        repo.upsert(pool, &node).await?;

        repo.update(pool, &node).await?;

        Ok(())
    }
}
