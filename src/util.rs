use axum::response::{Html, IntoResponse};
use hyper::StatusCode;
use tokio::io::AsyncReadExt;

// TODO: Should be cached, maybe generate these at compile time
pub async fn md_file_to_html(path: &std::path::Path) -> impl IntoResponse {
    // Open file
    let mut file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err)))
    };

    // Read file
    let mut dst = String::new();
    match file.read_to_string(&mut dst).await {
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read file: {}", err))),
        Ok(_) => (),
    }

    // Parse markdown and return html
    Ok(Html(markdown::to_html(&dst)))
}