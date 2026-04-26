use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddTargetReq {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Target {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub address: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
