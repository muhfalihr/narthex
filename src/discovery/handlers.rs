use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::app_state::AppState;
use super::models::NarthexSDResp;

#[derive(serde::Deserialize)]
pub struct SDQuery {
    pub group_id: Option<uuid::Uuid>,
}

#[utoipa::path(
    get,
    path = "/api/v1/targets",
    responses(
        (status = 200, description = "Successful response", body = Vec<NarthexSDResp>),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn get_sd_configs(
    State(state): State<AppState>,
    Query(query): Query<SDQuery>,
) -> impl IntoResponse {
    match state.discovery_service.get_sd_configs(query.group_id).await {
        Ok(configs) => {
            (StatusCode::OK, Json(configs)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get SD configs: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal Server Error"}))).into_response()
        }
    }
}
