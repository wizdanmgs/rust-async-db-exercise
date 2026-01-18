use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

mod url;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/{code}", get(url::redirect))
        .route("/shorten", post(url::shorten_url))
        .with_state(state)
}
