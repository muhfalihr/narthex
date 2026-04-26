use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use super::models::Target;

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

    pub async fn list_targets(&self, group_id: Uuid) -> Result<Vec<Target>> {
        let targets = sqlx::query_as::<_, Target>("SELECT id, group_id, address, created_at FROM targets WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(&self.pool)
            .await?;

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
