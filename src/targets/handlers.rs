use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::app_state::AppState;
use super::models::AddTargetReq;

pub async fn add_target(
    State(state): State<AppState>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<AddTargetReq>,
) -> impl IntoResponse {
    match state.target_service.add_target(group_id, &payload.address).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "id": id }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to add target: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to add target"}))).into_response()
        }
    }
}

pub async fn list_targets(
    State(state): State<AppState>,
    Path(group_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.target_service.list_targets(group_id).await {
        Ok(targets) => (StatusCode::OK, Json(targets)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list targets: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to list targets"}))).into_response()
        }
    }
}

pub async fn remove_target(
    State(state): State<AppState>,
    Path((group_id, address)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    match state.target_service.remove_target(group_id, &address).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Failed to remove target: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to remove target"}))).into_response()
        }
    }
}
