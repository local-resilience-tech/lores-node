use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use self::{
    client_events::ClientEvent,
    routes::{this_node, this_p2panda_node, this_region},
};

pub mod client_events;
pub mod realtime;
mod routes;

pub fn api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/this_region", this_region::router())
        .nest("/this_p2panda_node", this_p2panda_node::router())
        .nest("/this_node", this_node::router())
        .routes(routes!(dummy_event))
}

#[utoipa::path(get, path = "/dummy_event", responses(
    (status = 200, body = Option<ClientEvent>),
))]
async fn dummy_event() -> impl IntoResponse {
    (StatusCode::OK, Json(None::<ClientEvent>)).into_response()
}
