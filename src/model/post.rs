use anyhow::{Ok, Result};
use chrono::{DateTime, Local};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    id: PostId,
    title: PostTitle,
    content: PostContent,
    tags: Vec<Tag>,
    posted_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>,
}

impl Post {
    pub fn from_markdown(id: PostId, markdown: &str) -> Result<Self> {
        let (frontmatter, body) = Self::partite_to_frontmatter_and_body(markdown).unwrap();

        let frontmatter = Self::parse_frontmatter(frontmatter).unwrap();
        let body = Self::convert_to_html(body).unwrap(); // convert to html

        Ok(Self {
            id,
            title: PostTitle::new(&frontmatter.title).unwrap(),
            content: PostContent::new(body.as_str()).unwrap(),
            tags: frontmatter
                .tags
                .into_iter()
                .map(|tag| Tag::new(tag.as_str()).unwrap())
                .collect(),
            posted_at: Local::now(),
            updated_at: None,
        })
    }

    pub fn id(&self) -> &PostId {
        &self.id
    }

    pub fn title(&self) -> &PostTitle {
        &self.title
    }

    pub fn content(&self) -> &PostContent {
        &self.content
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn posted_at(&self) -> &DateTime<Local> {
        &self.posted_at
    }

    pub fn updated_at(&self) -> &Option<DateTime<Local>> {
        &self.updated_at
    }

    fn partite_to_frontmatter_and_body(markdown: &str) -> Result<(&str, &str)> {
        let re = Regex::new(r"^([\s]*)---([\s\S]*)---([\s\S]*)$").unwrap();

        let cap = re.captures_iter(markdown).nth(0).unwrap();
        let frontmatter = cap.get(2).map(|mat| mat.as_str()).unwrap().trim();
        let body = cap.get(3).map(|mat| mat.as_str()).unwrap().trim();

        Ok((frontmatter, body))
    }

    fn parse_frontmatter(frontmatter: &str) -> Result<FrontMatter> {
        let frontmatter: FrontMatter = serde_yaml::from_str(frontmatter).unwrap();

        Ok(frontmatter)
    }

    fn convert_to_html(markdown: &str) -> Result<String> {
        let options = Options::empty();
        let parser = Parser::new_ext(markdown, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(html_output)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct FrontMatter {
    title: String,
    tags: Vec<String>,
}

#[test]
fn test_partite_to_frontmatter_and_body() {
    let markdown = r###"---
title: "タイトル"
tags:
    - タグ1
    - タグ2
---

#This is body."###;

    let (frontmatter, body) = Post::partite_to_frontmatter_and_body(markdown).unwrap();

    assert_eq!(
        r#"title: "タイトル"
tags:
    - タグ1
    - タグ2"#,
        frontmatter
    );
    assert_eq!("#This is body.", body)
}

#[test]
fn test_parse_frontmatter() {
    let frontmatter = r#"title: "タイトル"
tags:
    - タグ1
    - タグ2"#;

    let frontmatter = Post::parse_frontmatter(frontmatter).unwrap();
    let expected = FrontMatter {
        title: "タイトル".to_string(),
        tags: vec!["タグ1".to_string(), "タグ2".to_string()],
    };

    assert_eq!(frontmatter, expected)
}

#[test]
fn test_convert_to_html() {
    let markdown = "# Hello world";

    let expected = "<h1>Hello world</h1>\n";
    let html = Post::convert_to_html(markdown).unwrap();

    assert_eq!(expected, html)
}

#[test]
#[ignore]
fn test_from_markdown() {
    let markdown = r###"---
title: "タイトル"
tags:
    - タグ1
    - タグ2
---

# Hello world
"###;

    let post = Post::from_markdown(PostId::new("sample-id").unwrap(), markdown);

    println!("{:?}", post);
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PostId(String);

impl PostId {
    pub fn new(value: &str) -> Result<Self> {
        let len = value.len();

        if len >= 1 && len <= 50 {
            Ok(Self(value.to_string()))
        } else {
            todo!()
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PostTitle(String);

impl PostTitle {
    pub fn new(value: &str) -> Result<Self> {
        let len = value.len();

        if len >= 1 && len <= 200 {
            Ok(Self(value.to_string()))
        } else {
            todo!()
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PostContent(String);

impl PostContent {
    pub fn new(value: &str) -> Result<Self> {
        let len = value.len();

        if len >= 1 && len <= 50000 {
            Ok(Self(value.to_string()))
        } else {
            todo!()
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tag(String);

impl Tag {
    pub fn new(value: &str) -> Result<Self> {
        let len = value.len();

        if len >= 1 && len <= 50 {
            Ok(Self(value.to_string()))
        } else {
            todo!()
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
