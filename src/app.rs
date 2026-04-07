use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{features::health, state::SharedState};

pub fn build_app(state: SharedState) -> Router {
    let app = Router::<SharedState>::new()
        .merge(health::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    app.with_state(state)
}
