use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
    PasswordHasher,
};
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

use crate::state::SharedState;

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize, FromRow)]
struct User {
    id: Uuid,
    email: String,
    username: String,
}

pub fn router() -> Router<SharedState> {
    Router::<SharedState>::new().route("/auth/register", post(register))
}

async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    if payload.email.trim().is_empty()
        || payload.username.trim().is_empty()
        || payload.password.trim().is_empty()

    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let password_hash = hash_password(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, username
        "#,
    )
    .bind(payload.email.trim())
    .bind(payload.username.trim())
    .bind(password_hash)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => {
            Err(StatusCode::CONFLICT)
        }
        Err(error) => {
            tracing::error!("failed to register user: {error}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}
