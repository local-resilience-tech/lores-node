use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

pub fn cors_fairing() -> rocket_cors::Cors {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::some(&["Authorization", "Accept", "Content-type"]))
        .allow_credentials(true);

    cors.to_cors().unwrap()
}
