use std::path::PathBuf;

use axum::{
    extract::FromRef, routing::get, Router
};

use axum_template::engine::Engine;
use minijinja::{Environment, path_loader};
use minijinja_autoreload::AutoReloader;
use tower_http::trace::TraceLayer;

mod routes;
mod util;

type AppEngine = Engine<AutoReloader>;

// Application shared state
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}


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

    // Set up the `minijinja` engine with the same route paths as the Axum router
    let jinja = AutoReloader::new(move |notifier| {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("templates");
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));
        notifier.set_fast_reload(true);
        notifier.watch_path(&template_path, true);
        Ok(env)
    });

    // Build the Axum application
    let app = Router::new()
        .route("/", get(routes::load_indexmd))
        .route("/assets/{*asset}", get(routes::get_file_endpoint))
        .route("/{*page}", get(routes::load_page))
        .fallback(routes::handler_404)
        // Create the application state
        .with_state(AppState {
            engine: Engine::from(jinja),
        });

    // Start the TCP listener on addr
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());

    // Run the Axum server
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}