use axum::{
    body::Body, extract::Query, http::{header, StatusCode}, response::{Html, IntoResponse}, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;
use tower_http::{services::ServeFile, trace::TraceLayer};
use std::{net::SocketAddr, path::PathBuf};


#[tokio::main]
async fn main() {
    // Setup logging
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            // .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )
    .unwrap();

    // Build the Axum application
    let app = Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/images/{image}", get(get_image))
        .route("/md_test", get(md_test))
        .route("/about", get(about))
        .route("/contact", post(contact))
        .fallback(handler_404);
    // Define the address for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 35838)); // Remember to change this

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    // Run the Axum server
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("<img src=\"https://http.cat/404\" alt=\"http cat 404\"></img>"))
}

// TODO: Should be cached, maybe generate these at compile time
async fn md_file_to_html(path: &std::path::Path) -> impl IntoResponse {
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

async fn md_test() -> impl IntoResponse {
    md_file_to_html(std::path::Path::new("assets/blogtest.md")).await
}
// Homepage handler
// async fn homepage() -> &'static str {
//     "Welcome to My Rust Website!"
// }

// About page handler
async fn about() -> &'static str {
    "This is the About Page of the Rust Website."
}
// Contact form handler
#[derive(Deserialize)]
struct ContactForm {
    name: String,
    message: String,
}
#[derive(Serialize)]
struct ResponseMessage {
    status: String,
    message: String,
}
async fn contact(Json(payload): Json<ContactForm>) -> Json<ResponseMessage> {
    Json(ResponseMessage {
        status: "success".to_string(),
        message: format!("Thanks for reaching out, {}!", payload.name),
    })
}

// #[derive(Deserialize)]
// pub struct QueryParams {
//     path: String,
// }

// pub async fn get_image(
//     Query(QueryParams { path }): Query<QueryParams>
// ) -> impl IntoResponse {

pub async fn get_image(
    axum::extract::Path(image_name): axum::extract::Path<String>
) -> impl IntoResponse {
    let mut path = PathBuf::from("assets/images");
    path.push(&image_name);
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