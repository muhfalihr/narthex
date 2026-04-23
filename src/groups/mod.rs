pub mod models;
pub mod service;
pub mod handlers;

use axum::{
    routing::get,
    Router,
};
use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_groups).post(handlers::create_group))
        .route("/{id}", get(handlers::get_group).put(handlers::update_group).delete(handlers::delete_group))
}
