use axum::{extract, response::IntoResponse};
use axum_template::RenderHtml;
use hyper::StatusCode;
use serde::Serialize;

use crate::{util::file_to_response, AppEngine};

pub async fn load_page(
    engine: AppEngine,
    extract::Path(path_name): extract::Path<String>,
) -> impl IntoResponse {
    return file_to_response(engine, "pages".to_string(), path_name).await;
}

pub async fn load_asset(
    engine: AppEngine,
    extract::Path(path_name): extract::Path<String>,
) -> impl IntoResponse {
    return file_to_response(engine, "assets".to_string(), path_name).await;
}

pub async fn load_index_page(engine: AppEngine) -> impl IntoResponse {
    return file_to_response(engine, "pages".to_string(), "index".to_string()).await;
}

#[derive(Debug, Serialize)]
struct NotFound {}
pub async fn handler_404(engine: AppEngine) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, RenderHtml("404.html", engine, NotFound {}))
}
