use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::features::health;

pub fn build_app() -> Router {
    Router::new()
        .merge(health::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
