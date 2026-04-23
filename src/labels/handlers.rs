use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::app_state::AppState;
use super::models::UpsertLabelReq;

pub async fn upsert_label(
    State(state): State<AppState>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<UpsertLabelReq>,
) -> impl IntoResponse {
    match state.label_service.upsert_label(group_id, &payload.key, &payload.value).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            tracing::error!("Failed to upsert label: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to upsert label"}))).into_response()
        }
    }
}

pub async fn list_labels(
    State(state): State<AppState>,
    Path(group_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.label_service.list_labels(group_id).await {
        Ok(labels) => (StatusCode::OK, Json(labels)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list labels: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to list labels"}))).into_response()
        }
    }
}

pub async fn remove_label(
    State(state): State<AppState>,
    Path((group_id, key)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    match state.label_service.remove_label(group_id, &key).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Failed to remove label: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to remove label"}))).into_response()
        }
    }
}
