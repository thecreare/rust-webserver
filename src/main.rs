use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build the Axum application
    let app = Router::new()
        .route("/", get(homepage))
        .route("/about", get(about))
        .route("/contact", post(contact));
    // Define the address for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 35837));
    println!("Server running at http://{}", addr);
    // Run the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// Homepage handler
async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}
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