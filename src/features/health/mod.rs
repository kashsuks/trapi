mod routes;

use axum::Router;

pub fn router() -> Router {
    routes::router()
}
