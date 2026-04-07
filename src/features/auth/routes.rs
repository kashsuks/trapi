use axum::{
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::state::SharedState;

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    message: &'static str,
}

pub fn router() -> Router<SharedState> {
    Router::<SharedState>::new().route("/auth/register", post(register))
}

async fn register(
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), StatusCode> {
    let _ = payload;

    Ok((
            StatusCode::CREATED,
            Json(RegisterResponse { 
                message: "register route scaffolded", 
            }),
    ))
}
