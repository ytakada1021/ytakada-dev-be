use std::env;

use mongodb::{options::ClientOptions, Client, Database};

use crate::{
    adapter::persistence::post::MongoPostRepository, app::{persistence::PostRepository, service::{get_post::GetPostService, delete_post::DeletePostService, list_post::ListPostService}},
    app::save_post::SavePostService,
};

use super::authorizer::Authorizer;

pub struct Container {
    pub db: Database,
    pub post_repository: Box<dyn PostRepository + Sync + Send>,
    pub save_post_service: SavePostService,
    pub get_post_service: GetPostService,
    pub delete_post_service: DeletePostService,
    pub list_post_service: ListPostService,
    pub authorizer: Authorizer,
}

impl Container {
    pub async fn new() -> Self {
        // DB_URL format
        // mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]
        // see: https://www.mongodb.com/docs/manual/reference/connection-string/
        let db_url = env::var("DB_URL").expect("DB_URL must be set as environment variable.");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set as environment variable.");

        let options = ClientOptions::parse(db_url).await.unwrap();

        let db = Client::with_options(options).unwrap().database(&db_name);

        let post_repoisotry = MongoPostRepository::new(db.clone()).unwrap();

        let save_post_service = SavePostService {
            repo: Box::new(post_repoisotry.clone()),
        };

        let get_post_service = GetPostService {
            db: db.clone(),
        };

        let delete_post_service = DeletePostService {
            repo: Box::new(post_repoisotry.clone()),
        };

        let list_post_service = ListPostService {
            db: db.clone(),
        };

        let api_key = env::var("API_KEY").expect("API_KEY must be set as environment variable.");

        Self {
            db,
            post_repository: Box::new(post_repoisotry),
            save_post_service,
            get_post_service,
            delete_post_service,
            list_post_service,
            authorizer: Authorizer::new(api_key.as_str()),
        }
    }
}
