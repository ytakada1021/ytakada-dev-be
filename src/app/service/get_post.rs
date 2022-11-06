use bson::doc;
use mongodb::Database;

use crate::core::model::post::Post;

pub struct GetPostService {
    pub db: Database,
}

impl GetPostService {
    pub async fn execute(&self, id: &str) -> Option<Post> {
        self.db
            .collection::<Post>("posts")
            .find_one(Some(doc! {"id": id}), None)
            .await
            .unwrap()
    }
}

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test_execute() {
    use mongodb::{options::ClientOptions, Client};

    let options = ClientOptions::parse("mongodb://root:example@mongo:27017")
        .await
        .unwrap();

    let db = Client::with_options(options)
        .unwrap()
        .database("ytakada_dev");

    let post = db
        .collection::<Post>("posts")
        .find_one(Some(doc! {"id": "sample-id2"}), None)
        .await
        .unwrap();

    println!("{:?}", post)
}
