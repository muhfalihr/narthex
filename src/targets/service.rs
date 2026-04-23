use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct TargetService {
    pool: PgPool,
}

impl TargetService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_target(&self, group_id: Uuid, address: &str) -> Result<Uuid> {
        let row = sqlx::query(
            "INSERT INTO targets (group_id, address) VALUES ($1, $2) RETURNING id"
        )
        .bind(group_id)
        .bind(address)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("id"))
    }

    pub async fn list_targets(&self, group_id: Uuid) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT address FROM targets WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;

        let targets = rows.into_iter().map(|r| r.get("address")).collect();
        Ok(targets)
    }

    pub async fn remove_target(&self, group_id: Uuid, address: &str) -> Result<()> {
        sqlx::query("DELETE FROM targets WHERE group_id = $1 AND address = $2")
            .bind(group_id)
            .bind(address)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
