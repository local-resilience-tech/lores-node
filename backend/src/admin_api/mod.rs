use self::routes::{this_node, this_p2panda_node, this_region};
use utoipa_axum::router::OpenApiRouter;

pub mod client_events;
pub mod realtime;
mod routes;

pub fn api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/this_region", this_region::router())
        .nest("/this_p2panda_node", this_p2panda_node::router())
        .nest("/this_node", this_node::router())
}
