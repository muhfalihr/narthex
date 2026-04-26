use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpsertLabelReq {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Label {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub key: String,
    pub value: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
