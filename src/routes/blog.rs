use std::path::PathBuf;

use axum::{extract, response::IntoResponse};
use axum_template::RenderHtml;
use serde::Serialize;

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