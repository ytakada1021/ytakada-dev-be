use lambda_http::Error;

use ytakada_dev::driver::handler::list_post::handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = lambda_http::service_fn(handler);

    lambda_http::run(service).await?;

    Ok(())
}
