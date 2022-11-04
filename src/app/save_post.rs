use anyhow::{Ok, Result};

use crate::model::post::{Post, PostId, PostRepository};
#[cfg(test)]
use crate::driver::container::Container;

pub struct SavePostService {
    pub repo: Box<dyn PostRepository + Sync + Send>,
}

impl SavePostService {
    pub async fn execute(&self, post_id: &str, markdown: &str) -> Result<Post> {
        let post_id = PostId::new(post_id).unwrap();

        let post = Post::from_markdown(post_id, markdown).unwrap();

        let _ = &self.repo.save(&post).await.unwrap();

        Ok(post)
    }
}

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test_execute() {
    let container = Container::new().await;

    let markdown = r###"---
title: "タイトル"
tags:
    - タグ1
    - タグ2
---

# This is body."###;

    let post = container
        .save_post_service
        .execute("sample-id3", markdown)
        .await
        .unwrap();

    println!("{:?}", post);
}
