use lambda_http::{http::StatusCode, Error, Request, RequestExt, Response};

use crate::driver::container::Container;

use super::{response_400, response_403, response_404};

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

    let params = req.path_parameters();
    let post_id = params.first("id").clone();

    if post_id.is_none() {
        return Ok(response_400("Post id must be provided."));
    }

    let post = container.get_post_service.execute(post_id.unwrap()).await;

    match post {
        Some(post) => {
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
        None => {
            let res = response_404(&format!(
                "Post of provided id ({}) does not exist.",
                post_id.unwrap()
            ));

            Ok(res)
        }
    }
}
