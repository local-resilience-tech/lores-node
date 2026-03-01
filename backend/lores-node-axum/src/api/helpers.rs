use axum::{http::StatusCode, Json};

pub fn internal_server_error<E: std::fmt::Debug>(error: E) -> (StatusCode, Json<String>) {
    let stringified_error = format!("Internal server error: {:?}", error);

    (StatusCode::INTERNAL_SERVER_ERROR, Json(stringified_error))
}

pub fn bad_request<E: std::fmt::Debug>(error: E) -> (StatusCode, Json<String>) {
    let stringified_error = format!("Bad request: {:?}", error);

    (StatusCode::BAD_REQUEST, Json(stringified_error))
}
