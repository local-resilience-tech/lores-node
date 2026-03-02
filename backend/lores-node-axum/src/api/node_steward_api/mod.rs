use utoipa_axum::router::OpenApiRouter;

mod routes;

pub fn node_steward_api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/my_region_nodes", routes::my_region_nodes::router())
        .nest("/local_apps", routes::local_apps::router())
        .nest("/my_regions", routes::my_regions::router())
}
