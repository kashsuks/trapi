use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2,
    PasswordHasher,
};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json,
    Router,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    features::auth::extractor::AuthUser,
    state::SharedState,
};

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, FromRow)]
struct User {
    id: Uuid,
    email: String,
    username: String,
}

#[derive(FromRow)]
struct UserCredentials {
    id: Uuid,
    password_hash: String,
}

#[derive(Serialize, FromRow)]
struct MeReponse {
    id: Uuid,
    email: String,
    username: String,
    total_workout_count: i64,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub fn router() -> Router<SharedState> {
    Router::<SharedState>::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
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

async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if payload.email.trim().is_empty() || payload.password.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = sqlx::query_as::<_, UserCredentials>(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(payload.email.trim())
    .fetch_optional(&state.db)
    .await
    .map_err(|error| {
        tracing::error!("failed to fetch login user: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    verify_password(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = create_token(user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token }))
}

async fn me(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<MeReponse>, StatusCode> {
    let user = sqlx::query_as::<_, MeReponse>(
        r#"
        SELECT
            u.id,
            u.email,
            u.username,
            COUNT(w.id)::BIGINT AS total_workout_count
        FROM users u
        LEFT JOIN workouts w ON w.user_id = u.id
        WHERE u.id = $1
        GROUP BY u.id, u.email, u.username
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|error| {
        tracing::error!("failed to fetch current user: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}

fn create_token(user_id: Uuid, jwt_secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now() + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}
