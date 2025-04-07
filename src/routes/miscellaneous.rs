use std::path::PathBuf;

use axum::{body::Body, extract, response::IntoResponse};
use axum_template::RenderHtml;
use hyper::{header, StatusCode};
use serde::Serialize;
use tokio_util::io::ReaderStream;

use crate::{util::{md_to_string, open_file}, AppEngine};

#[derive(Debug, Serialize)]
struct Index {
    markdown: String,
}

pub async fn index(
    engine: AppEngine,
) -> impl IntoResponse {
    let markdown = match md_to_string(std::path::Path::new("assets/markdown/home.md")).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let page = Index { markdown };

    Ok(RenderHtml("plain_markdown_page.html", engine, page))
}

pub async fn get_file_endpoint(
    extract::Path(file_name): extract::Path<String>
) -> impl IntoResponse {
    let mut path = PathBuf::from("assets");
    path.push(&file_name);
    
    let (file, content_type, filename) = match open_file(&path).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, content_type),
        (
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{:?}\"", filename),
        ),
    ];

    Ok((headers, body).into_response())
}

#[derive(Debug, Serialize)]
struct NotFound {}
pub async fn handler_404(engine: AppEngine) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, RenderHtml("404.html", engine, NotFound {}))
}