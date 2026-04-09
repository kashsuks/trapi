use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(db: PgPool, jwt_secret: String) -> SharedState {
        Arc::new(Self { db, jwt_secret })
    }
}
