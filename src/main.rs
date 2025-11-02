use dioxus::prelude::*;

use crate::views::App;

mod components;
#[cfg(feature = "server")]
mod config;
mod server;
#[cfg(feature = "server")]
mod state;
mod views;
mod utils;
mod models;

fn main() {
    #[cfg(feature = "web")]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                launch_server(App).await;
            });
    }
}

#[cfg(feature = "server")]
async fn launch_server(component: fn() -> Element) {
    use std::sync::Arc;
    use axum::{extract::Request, middleware::Next, routing::post};

    use crate::{config::Config, server::login_handler, state::AppState};

    dotenvy::dotenv().ok();

    create_tracing_subscriber();

    let config = Config::load_config();
    let app_state = Arc::new(AppState::new(config));

    let address = dioxus::cli_config::fullstack_address_or_localhost();
    tracing::debug!("Starting server at {}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let router = axum::Router::new()
        .route("/api/login", post(login_handler))
        .with_state(app_state)
        .serve_dioxus_application(ServeConfig::default(), component)
        // .layer(axum::middleware::from_fn(
        //     |request: Request, next: Next| async move {

        //     }
        // ))
        .into_make_service();

    axum::serve(listener, router).await.unwrap();
}

#[cfg(feature = "server")]
fn create_tracing_subscriber() {
    // Read log level from environment variable, defaulting to "info"
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase();

    // Map the string log level to the tracing Level enum
    let level = match log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO, // Default to INFO for invalid values
    };

    // Build the subscriber with the configured log level
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    // Set as the global default subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set default tracing subscriber");
}
