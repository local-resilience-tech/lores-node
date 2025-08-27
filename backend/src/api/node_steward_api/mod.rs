use utoipa_axum::router::OpenApiRouter;

mod routes;

pub fn node_steward_api_router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/this_node", routes::this_node::router())
}
