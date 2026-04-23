pub mod models;
pub mod service;
pub mod handlers;

use axum::{routing::get, Router};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/targets", get(handlers::get_sd_configs))
}
