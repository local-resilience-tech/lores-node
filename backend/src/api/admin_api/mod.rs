use utoipa_axum::router::OpenApiRouter;

mod admin_node_routes;
mod admin_node_stewards_routes;

pub fn admin_api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/node_stewards", admin_node_stewards_routes::router())
        .nest("/node", admin_node_routes::router())
}
