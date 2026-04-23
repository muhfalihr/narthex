use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::app_state::AppState;
use super::models::{CreateGroupReq, UpdateGroupReq};

pub async fn create_group(
    State(state): State<AppState>,
    Json(payload): Json<CreateGroupReq>,
) -> impl IntoResponse {
    match state.group_service.create_group(&payload.name, payload.description.as_deref()).await {
        Ok(id) => (StatusCode::CREATED, Json(json!({ "id": id }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create group: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create group"}))).into_response()
        }
    }
}

pub async fn list_groups(State(state): State<AppState>) -> impl IntoResponse {
    match state.group_service.get_groups().await {
        Ok(groups) => (StatusCode::OK, Json(groups)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list groups: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to list groups"}))).into_response()
        }
    }
}

pub async fn get_group(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.group_service.get_group(id).await {
        Ok(Some(group)) => (StatusCode::OK, Json(group)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Group not found"}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to get group: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to get group"}))).into_response()
        }
    }
}

pub async fn update_group(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateGroupReq>,
) -> impl IntoResponse {
    match state.group_service.update_group(id, &payload.name, payload.description.as_deref()).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            tracing::error!("Failed to update group: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to update group"}))).into_response()
        }
    }
}

pub async fn delete_group(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.group_service.delete_group(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Failed to delete group: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to delete group"}))).into_response()
        }
    }
}
