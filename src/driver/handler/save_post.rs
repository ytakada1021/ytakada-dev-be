use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};

use crate::driver::container::Container;

pub async fn handler(req: Request) -> Result<Response<String>, Error> {
    let container = Container::new().await;

    match req.headers().get("x-api-key") {
        Some(subject) => {
            container
                .authorizer
                .authorize(subject.to_str().unwrap())
                .unwrap();
        },
        None => todo!()
    }

    let post_id = req
        .path_parameters()
        .first("id")
        .expect("Post id should be provided.")
        .to_string();

    let body = match req.body() {
        Body::Text(text) => Ok(text),
        _ => Err(()),
    }
    .unwrap();

    let post = &container
        .save_post_service
        .execute(&post_id, body)
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
