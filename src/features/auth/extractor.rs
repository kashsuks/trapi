use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::SharedState;

pub struct AuthUser {
    pub user_id: Uuid,
}

#[derive(Deserialize, Clone)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

impl FromRequestParts<SharedState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
            parts: &mut Parts,
            state: &SharedState,
        ) -> Result<Self, Self::Rejection> {
        let authorization = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = authorization
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Self {
            user_id: token_data.claims.sub,
        })
    }
}
