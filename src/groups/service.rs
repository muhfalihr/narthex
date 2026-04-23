use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::TargetGroup;

#[derive(Clone)]
pub struct GroupService {
    pool: PgPool,
}

impl GroupService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_group(&self, name: &str, description: Option<&str>) -> Result<Uuid> {
        let row = sqlx::query(
            "INSERT INTO target_groups (name, description) VALUES ($1, $2) RETURNING id"
        )
        .bind(name)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get("id"))
    }

    pub async fn get_groups(&self) -> Result<Vec<TargetGroup>> {
        let groups = sqlx::query_as::<_, TargetGroup>(
            "SELECT id, name, description, created_at, updated_at FROM target_groups ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(groups)
    }

    pub async fn get_group(&self, id: Uuid) -> Result<Option<TargetGroup>> {
        let group = sqlx::query_as::<_, TargetGroup>(
            "SELECT id, name, description, created_at, updated_at FROM target_groups WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(group)
    }

    pub async fn update_group(&self, id: Uuid, name: &str, description: Option<&str>) -> Result<()> {
        sqlx::query(
            "UPDATE target_groups SET name = $1, description = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3"
        )
        .bind(name)
        .bind(description)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_group(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM target_groups WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
