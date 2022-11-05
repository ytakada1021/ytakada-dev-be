use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use serde::Serialize;

use crate::driver::container::Container;

pub async fn handler(req: Request) -> Result<Response<String>, Error> {
    let container = Container::new().await;

    match req.headers().get("x-api-key") {
        Some(subject) => {
            if container
                .authorizer
                .authorize(subject.to_str().unwrap())
                .is_err()
            {
                return Ok(response_403("Invalid api key."));
            }
        },
        None => return Ok(response_403("Invalid api key.")),
    }

    let params = req.path_parameters();
    let post_id = params.first("id").clone();

    if post_id.is_none() {
        return Ok(response_400("Post id must be provided."))
    }

    let body = match req.body() {
        Body::Text(text) => text,
        _ => return Ok(response_400("Invalid request body.")),
    };

    let post = &container
        .save_post_service
        .execute(&post_id.unwrap(), body)
        .await
        .unwrap();

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
        .header("Access-Control-Allow-Credential", "true")
        .header("Access-Control-Allow-Origin", "*")
        .body(serde_json::to_string(&post).unwrap())
        .expect("failed to render response");

    Ok(res)
}

fn response_400(msg: &str) -> Response<String> {
    let body = ErrorResponseBody {
        message: msg.to_string(),
    };

    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Failed to render response.")
}

fn response_403(msg: &str) -> Response<String> {
    let body = ErrorResponseBody {
        message: msg.to_string(),
    };

    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .expect("Failed to render response.")
}

#[derive(Debug, Serialize)]
struct ErrorResponseBody {
    message: String,
}
