use utoipa_axum::router::OpenApiRouter;

use crate::routes::this_region;

pub fn api_router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/this_region", this_region::router())
}
