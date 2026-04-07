use axum::{
    extract::State,
    http::StatusCode,
    routing::get, 
    Json, 
    Router
};
use serde::Serialize;

use crate::state::SharedState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    database: &'static str,
}

pub fn router() -> Router<SharedState> {
    Router::<SharedState>::new().route("/health", get(health))
}

async fn health(
    State(state): State<SharedState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .map_err(|error| {
            tracing::error!("database health check failed: {error}");
                StatusCode::SERVICE_UNAVAILABLE
        })?;

    Ok(Json(HealthResponse { 
        status: "ok", 
        database: "ok", 
    }))
}
