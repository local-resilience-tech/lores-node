use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use self::{
    client_events::ClientEvent,
    routes::{
        local_apps, nodes, region_apps, stacks, this_p2panda_node, this_region, this_region_node,
    },
};

pub mod client_events;
pub mod realtime;
mod routes;

pub fn public_api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/this_region", this_region::router())
        .nest("/this_p2panda_node", this_p2panda_node::router())
        .nest("/this_region_node", this_region_node::router())
        .nest("/nodes", nodes::router())
        .nest("/local_apps", local_apps::router())
        .nest("/region_apps", region_apps::router())
        .nest("/stacks", stacks::router())
        .routes(routes!(dummy_event))
}

#[utoipa::path(get, path = "/dummy_event", responses(
    (status = 200, body = Option<ClientEvent>),
))]
async fn dummy_event() -> impl IntoResponse {
    (StatusCode::OK, Json(None::<ClientEvent>)).into_response()
}
