mod routes;

use axum::Router;

use crate::state::SharedState;

pub fn router() -> Router<SharedState> {
    routes::router()
}
