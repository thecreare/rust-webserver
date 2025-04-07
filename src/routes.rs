use std::path::PathBuf;

use axum::{body::Body, response::IntoResponse};
use hyper::{header, StatusCode};
use tokio_util::io::ReaderStream;

use crate::util;

pub async fn md_test() -> impl IntoResponse {
    util::md_file_to_html(std::path::Path::new("assets/blogtest.md")).await
}

pub async fn get_file(
    axum::extract::Path(file_name): axum::extract::Path<String>
) -> impl IntoResponse {
    let mut path = PathBuf::from("assets");
    path.push(&file_name);
    let filename = match path.file_name() {
        Some(name) => name,
        None => return Err((StatusCode::BAD_REQUEST, "File name couldn't be determined".to_string()))
    };
    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err)))
    };
    let content_type = match mime_guess::from_path(&path).first_raw() {
        Some(mime) => mime,
        None => return Err((StatusCode::BAD_REQUEST, "MIME Type couldn't be determined".to_string()))
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