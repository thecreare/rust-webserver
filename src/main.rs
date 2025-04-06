use axum::{
    http::StatusCode, response::{Html, IntoResponse}, routing::get, Router
};

use tower_http::{services::ServeFile, trace::TraceLayer};

mod routes;
mod util;

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
        .route("/images/{image}", get(routes::get_image))
        .route("/md_test", get(routes::md_test))
        .fallback(handler_404);
    // Define the address for the server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8001));

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