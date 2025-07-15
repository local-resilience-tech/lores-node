use std::env;

use axum::{
    body::Body,
    http::{Request, Response, StatusCode, Uri},
};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

lazy_static! {
    pub static ref FRONTEND_PATH: String =
        env::var("FRONTEND_PATH").unwrap_or_else(|_| "../frontend/dist".to_string());
}

pub async fn frontend_handler(uri: Uri) -> Result<Response<Body>, (StatusCode, String)> {
    if uri.path().starts_with("/api") {
        println!(
            "API call detected, not serving static files for URI: {}",
            uri
        );
        return Err((StatusCode::NOT_FOUND, "API endpoint not found".to_string()));
    }

    let res = serve_dir(uri.clone(), FRONTEND_PATH.clone()).await?;

    match res.status() {
        StatusCode::OK => Ok(res),
        StatusCode::NOT_FOUND => {
            // serve the HTML file for SPA routing
            let spa_file_path_string = FRONTEND_PATH.clone() + "/index.html";
            println!(
                "Serving SPA index.html for URI: {}, using path {}",
                uri.clone(),
                spa_file_path_string
            );
            return serve_file(uri.clone(), spa_file_path_string).await;
        }
        other => {
            // If the status is not OK or NOT_FOUND, return the response as is
            println!("Got other status: {}", other);
            return Ok(res);
        }
    }
}

async fn serve_file(
    uri: Uri,
    spa_file_path_string: String,
) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    match ServeFile::new(spa_file_path_string).oneshot(req).await {
        Ok(res) => Ok(res.map(Body::new)),
        Err(err) => {
            println!("Error serving static file: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            ));
        }
    }
}

async fn serve_dir(uri: Uri, static_dir: String) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    match ServeDir::new(static_dir).oneshot(req).await {
        Ok(res) => Ok(res.map(Body::new)),
        Err(err) => {
            println!("Error serving static file: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            ));
        }
    }
}
