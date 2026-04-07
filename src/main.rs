use axum::{
    routing::get,
    Json,
    Router,
};
use dotenvy::dotenv;
use serde::Serialize;
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

struct Config {
    host: String,
    port: u16,
    database_url: String,
    jwt_secret: String,
}

impl Config {
    fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(3000);

        let database_url = 
            env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let jwt_secret =
            env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Self {
            host,
            port,
            database_url,
            jwt_secret,
        }
    }

    fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("invalid HOST or PORT")
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "trapi=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    let app = Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = config.socket_addr();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("listening on {}", addr);
    tracing::debug!("database configured");
    tracing::debug!("jwt configured: {} chars", config.jwt_secret.len());

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
