use anyhow::Result;
use async_trait::async_trait;

use crate::core::model::post::{PostId, Post};

#[async_trait]
pub trait PostRepository {
    async fn find(&self, id: &PostId) -> Result<Option<Post>>;
    async fn save(&self, post: &Post) -> Result<()>;
    async fn delete(&self, id: &PostId) -> Result<()>;
}
