use mongodb::{Client, options::ClientOptions};

use crate::{app::save_post::SavePostService, model::post::PostRepository, adapter::persistence::post::MongoPostRepository};

pub struct Container {
    pub post_repository: Box<dyn PostRepository + Sync + Send>,
    pub save_post_service: SavePostService,
}

impl Container {
    pub async fn new() -> Self {
        let options = ClientOptions::parse("mongodb://root:example@mongo:27017")
            .await
            .unwrap();

        let db = Client::with_options(options)
            .unwrap()
            .database("ytakada_dev");

        let post_repoisotry = MongoPostRepository::new(db).unwrap();

        let save_post_service = SavePostService { repo: Box::new(post_repoisotry.clone()) };

        Self {
            post_repository: Box::new(post_repoisotry),
            save_post_service,
        }
    }
}
