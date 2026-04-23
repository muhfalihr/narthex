pub mod models;
pub mod service;
pub mod handlers;

use axum::{
    routing::{get, delete},
    Router,
};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{group_id}/targets", get(handlers::list_targets).post(handlers::add_target))
        .route("/{group_id}/targets/{address}", delete(handlers::remove_target))
}
