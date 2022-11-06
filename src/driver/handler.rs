use lambda_http::{Response, http::StatusCode};
use serde::Serialize;

pub mod delete_post;
pub mod get_post;
pub mod save_post;

pub fn response_400(msg: &str) -> Response<String> {
    let body = ErrorResponseBody {
        message: msg.to_string(),
    };

    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Failed to render response.")
}

pub fn response_403(msg: &str) -> Response<String> {
    let body = ErrorResponseBody {
        message: msg.to_string(),
    };

    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Failed to render response.")
}

pub fn response_404(msg: &str) -> Response<String> {
    let body = ErrorResponseBody {
        message: msg.to_string(),
    };

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Failed to render response.")
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseBody {
    message: String,
}
