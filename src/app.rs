use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    features::{auth, docs, health, workouts},
    state::SharedState,
};

pub fn build_app(state: SharedState) -> Router {
    let app = Router::<SharedState>::new()
        .merge(docs::router())
        .merge(auth::router())
        .merge(health::router())
        .merge(workouts::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    app.with_state(state)
}
