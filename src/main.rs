mod app;
mod config;
mod db;
mod features;
mod state;

use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{app::build_app, config::Config, db::connect_db, state::AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let config = Config::from_env();
    let db = connect_db(&config.database_url).await;
    let state = AppState::new(db);
    let addr = config.socket_addr();
    let app = build_app(state);

    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("listening on {}", addr);
    tracing::debug!("database configured");
    tracing::debug!("jwt configured: {} chars", config.jwt_secret.len());

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "trapi=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
