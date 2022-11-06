use anyhow::Result;
use bson::doc;
use futures::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::Database;

use crate::core::model::post::Post;

pub struct ListPostService {
    pub db: Database,
}

impl ListPostService {
    pub async fn execute(&self) -> Result<Vec<Post>> {
        let options = FindOptions::builder().sort(doc! {"posted_at": -1}).build();

        let cursor = self
            .db
            .collection::<Post>("posts")
            .find(None, options)
            .await?;

        let posts = cursor.try_collect().await?;

        Ok(posts)
    }
}
