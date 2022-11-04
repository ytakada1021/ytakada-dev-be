use anyhow::{Ok, Result};
use async_trait::async_trait;
use mongodb::{bson::doc, options::UpdateOptions, Collection, Database};
#[cfg(test)]
use mongodb::{options::ClientOptions, Client};

use crate::model::post::{Post, PostId, PostRepository};

pub struct MongoPostRepository {
    db: Database,
}

impl MongoPostRepository {
    pub fn new(db: Database) -> Result<Self> {
        Ok(Self { db })
    }

    fn collection(&self) -> Result<Collection<Post>> {
        let db_ref = &self.db.clone();

        Ok(db_ref.collection::<Post>("posts"))
    }
}

#[async_trait]
impl PostRepository for MongoPostRepository {
    async fn find(&self, id: &PostId) -> Result<Option<Post>> {
        let post = self
            .collection()
            .unwrap()
            .find_one(doc! {"id": id.value()}, None)
            .await
            .unwrap();

        Ok(post)
    }

    async fn save(&self, post: &Post) -> Result<()> {
        let options = UpdateOptions::builder().upsert(Some(true)).build();

        self.collection()
            .unwrap()
            .update_one(
                doc! {
                    "id": post.id().value(),
                },
                doc! {
                    "$set": bson::to_bson(post).unwrap()
                },
                Some(options),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn delete(&self, id: &PostId) -> Result<()> {
        self.collection()
            .unwrap()
            .delete_one(
                doc! {
                    "id": id.value(),
                },
                None,
            )
            .await
            .unwrap();

        Ok(())
    }
}

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test_connection() {
    let options = ClientOptions::parse("mongodb://root:example@mongo:27017")
        .await
        .unwrap();

    let client = Client::with_options(options).unwrap();

    let names = client
        .database("ytakada_dev")
        .list_collection_names(None)
        .await
        .unwrap();

    println!("{:?}", names);
}

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test_find() {
    let options = ClientOptions::parse("mongodb://root:example@mongo:27017")
        .await
        .unwrap();

    let db = Client::with_options(options)
        .unwrap()
        .database("ytakada_dev");

    let repository = MongoPostRepository::new(db).unwrap();

    let post = repository
        .find(&PostId::new("sample-id").unwrap())
        .await
        .unwrap();
    assert!(post.is_some());
    println!("{:?}", post);

    let post = repository
        .find(&PostId::new("nonexistent-id2").unwrap())
        .await
        .unwrap();
    assert!(post.is_none());
    println!("{:?}", post);
}

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test_save() {
    let options = ClientOptions::parse("mongodb://root:example@mongo:27017")
        .await
        .unwrap();

    let db = Client::with_options(options)
        .unwrap()
        .database("ytakada_dev");

    let repository = MongoPostRepository::new(db).unwrap();

    let markdown = r###"---
title: "タイトル"
tags:
    - タグ1
    - タグ2
---

# Hello world!!!
"###;

    let post = Post::from_markdown(PostId::new("sample-id").unwrap(), markdown).unwrap();

    repository.save(&post).await.unwrap();
}

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test_delete() {
    let options = ClientOptions::parse("mongodb://root:example@mongo:27017")
        .await
        .unwrap();

    let db = Client::with_options(options)
        .unwrap()
        .database("ytakada_dev");

    let repository = MongoPostRepository::new(db).unwrap();

    let id = PostId::new("sample-id").unwrap();

    repository.delete(&id).await.unwrap();
}
