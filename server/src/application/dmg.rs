use axum::extract::DefaultBodyLimit;
use crate::usecases::dmg::{get_dmg, update_dmg};
use axum::routing::{get, post};
use axum::Router;

pub fn load_dmg_router() -> Router {
    Router::new()
        .route("/:path", get(get_dmg))
        .route("/update", post(update_dmg))
        .layer(DefaultBodyLimit::disable())
}
