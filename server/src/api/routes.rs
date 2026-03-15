use axum::{Router, routing::get};

use crate::{AppState, api::handlers};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/galaxy/{owner}/{repo}", get(handlers::get_galaxy))
        .route("/api/repo/{owner}/{repo}", get(handlers::get_repository))
        .route("/api/search", get(handlers::search_repositories))
        .with_state(state)
}
