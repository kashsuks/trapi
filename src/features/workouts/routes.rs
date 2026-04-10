use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    features::auth::extractor::AuthUser,
    state::SharedState,
};

#[derive(Debug, Deserialize)]
struct CreateWorkoutRequest {
    workout_type: String,
    distance_km: Option<f64>,
    duration_seconds: Option<i32>,
    notes: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
struct Workout {
    id: Uuid,
    user_id: Uuid,
    workout_type: String,
    distance_km: Option<f64>,
    duration_seconds: Option<i32>,
    notes: Option<String>,
}

pub fn router() -> Router<SharedState> {
    Router::<SharedState>::new().route("/workouts", post(create_workout))
}

async fn create_workout(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateWorkoutRequest>,
) -> Result<(StatusCode, Json<Workout>), StatusCode> {
    if !is_valid_workout_type(&payload.workout_type) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.distance_km.is_some_and(|distance| distance < 0.0) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.duration_seconds.is_some_and(|duration| duration < 0) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = sqlx::query_as::<_, Workout>(
        r#"
        INSERT INTO workouts (user_id, workout_type, distance_km, duration_seconds, notes)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, workout_type, distance_km, duration_seconds, notes
        "#,
    )
    .bind(auth_user.user_id)
    .bind(payload.workout_type.trim())
    .bind(payload.distance_km)
    .bind(payload.duration_seconds)
    .bind(payload.notes.as_deref().map(str::trim))
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(workout) => Ok((StatusCode::CREATED, Json(workout))),
        Err(error) => {
            tracing::error!("failed to create workout: {error}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn is_valid_workout_type(workout_type: &str) -> bool {
    matches!(
        workout_type.trim(),
        "run" | "bike" | "swim" | "hike" | "lift" | "row"

    )
}
