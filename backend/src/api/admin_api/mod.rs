use utoipa_axum::router::OpenApiRouter;

mod admin_node_stewards_routes;

pub fn admin_api_router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/admin", admin_node_stewards_routes::router())
}
