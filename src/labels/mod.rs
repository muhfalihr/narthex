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
        .route("/{group_id}/labels", get(handlers::list_labels).post(handlers::upsert_label))
        .route("/{group_id}/labels/{key}", delete(handlers::remove_label))
}
