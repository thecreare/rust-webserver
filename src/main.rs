use axum::{
    response::{Html, IntoResponse}, routing::{get, post}, Json, Router
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tower_http::{services::ServeFile, trace::TraceLayer};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build the Axum application
    let app = Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .route("/about", get(about))
        .route("/contact", post(contact))
        .fallback(handler_404);
    // Define the address for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 35838)); // Remember to change this

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    // Run the Axum server
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("<img src=\"https://http.cat/404\" alt=\"http cat 404\"></img>"))
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