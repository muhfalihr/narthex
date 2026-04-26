use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;
use super::models::Label;

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

    pub async fn list_labels(&self, group_id: Uuid) -> Result<Vec<Label>> {
        let labels = sqlx::query_as::<_, Label>("SELECT id, group_id, label_key as key, label_value as value, created_at FROM labels WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;

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
