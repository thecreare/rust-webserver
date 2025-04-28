use std::{ffi::OsString, fs::FileType, path::{Path, PathBuf}, str::FromStr};

use axum::{extract, response::IntoResponse};
use axum_template::RenderHtml;
use serde::{Deserialize, Serialize};
use tokio::fs;
use toml::value::Datetime;
use tracing::error;

use crate::{util::md_to_string, AppEngine};

#[derive(Debug, Serialize)]
struct BlogPost {
    markdown: String,
}
pub async fn blog(
    engine: AppEngine,
    extract::Path(post): extract::Path<String>
) -> impl IntoResponse {

    let markdown_path = PathBuf::from("assets")
        .join("posts")
        .join(post)
        .with_extension("md");

    let markdown = match md_to_string(&markdown_path).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let post = BlogPost { markdown };

    Ok(RenderHtml("blog.html", engine, post))
}

#[derive(Debug, Deserialize, Serialize)]
struct TomlPost {
    title: String,
    date: Datetime,
    short_description: String,
}

#[derive(Debug, Serialize)]
struct Post {
    meta: TomlPost,
    path: String,
}

#[derive(Debug, Serialize)]
struct PostList {
    posts: Vec<Post>,
}

pub async fn list_posts(
    engine: AppEngine,
) -> impl IntoResponse {
    let posts_path = PathBuf::from("assets")
        .join("posts");

    let mut paths = fs::read_dir(posts_path).await.unwrap();

    let mut post_list = PostList {
        posts: Vec::new(),
    };

    while let Some(path) = paths.next_entry().await.unwrap() {
        if Path::new(&path.file_name()).extension().unwrap_or(&OsString::default()) == "toml" {
            let toml_str = fs::read_to_string(path.path()).await.unwrap();
            let toml_post: TomlPost = toml::from_str(&toml_str).unwrap();
            println!("{path:?} {}", toml_post.title);

            
            let path_string = path
                .path()
                .file_stem()
                .and_then(|stem| {
                    stem.to_str()
                }).and_then(|str| {
                    Some(str.to_string())
                });
            let path_string = match path_string {
                Some(path_string) => path_string,
                None => {
                    error!("Post at {path:?} has invalid unicode");
                    continue;
                },
            };

            post_list.posts.push(Post {
                meta: toml_post,
                path: path_string,
            });
        }
    }

    // Ok(RenderHtml("posts_list.html", engine, post_list))
    RenderHtml("posts_list.html", engine, post_list)
}