use anyhow::Result;

use crate::{app::persistence::PostRepository, core::model::post::PostId};

pub struct DeletePostService {
    pub repo: Box<dyn PostRepository + Send + Sync>,
}

impl DeletePostService {
    pub async fn execute(&self, id: &str) -> Result<()> {
        let id = &PostId::new(id).unwrap();

        self.repo.delete(id).await
    }
}
