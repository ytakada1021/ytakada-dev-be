use lambda_http::{http::StatusCode, Error, Request, Response};

use crate::driver::container::Container;

use super::response_403;

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
        }
        None => return Ok(response_403("Invalid api key.")),
    }

    let posts = container.list_post_service.execute().await.unwrap();

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
        .header("Access-Control-Allow-Credential", "true")
        .header("Access-Control-Allow-Origin", "*")
        .body(serde_json::to_string(&posts).unwrap())
        .expect("failed to render response");

    Ok(res)
}
