use utoipa_axum::router::OpenApiRouter;

mod routes;

pub fn node_steward_api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/this_region_node", routes::this_region_node::router())
        .nest("/local_apps", routes::local_apps::router())
        .nest("/regions", routes::regions::router())
}
