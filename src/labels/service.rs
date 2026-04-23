use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct LabelService {
    pool: PgPool,
}

impl LabelService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert_label(&self, group_id: Uuid, label_key: &str, label_value: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO labels (group_id, label_key, label_value)
            VALUES ($1, $2, $3)
            ON CONFLICT (group_id, label_key) 
            DO UPDATE SET label_value = EXCLUDED.label_value
            "#
        )
        .bind(group_id)
        .bind(label_key)
        .bind(label_value)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_labels(&self, group_id: Uuid) -> Result<HashMap<String, String>> {
        let rows = sqlx::query("SELECT label_key, label_value FROM labels WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;

        let mut labels = HashMap::new();
        for row in rows {
            labels.insert(row.get("label_key"), row.get("label_value"));
        }
        Ok(labels)
    }

    pub async fn remove_label(&self, group_id: Uuid, label_key: &str) -> Result<()> {
        sqlx::query("DELETE FROM labels WHERE group_id = $1 AND label_key = $2")
            .bind(group_id)
            .bind(label_key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
