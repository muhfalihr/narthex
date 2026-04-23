use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

use super::models::NarthexSDResp;

#[derive(Clone)]
pub struct DiscoveryService {
    pool: PgPool,
}

impl DiscoveryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_sd_configs(&self) -> Result<Vec<NarthexSDResp>> {
        let records = sqlx::query(
            r#"
            SELECT 
                g.id as group_id,
                COALESCE(json_agg(DISTINCT t.address) FILTER (WHERE t.address IS NOT NULL), '[]') as targets,
                COALESCE(json_object_agg(l.label_key, l.label_value) FILTER (WHERE l.label_key IS NOT NULL), '{}') as labels
            FROM target_groups g
            LEFT JOIN targets t ON t.group_id = g.id
            LEFT JOIN labels l ON l.group_id = g.id
            GROUP BY g.id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut results = Vec::new();

        for row in records {
            let targets_val: serde_json::Value = row.get("targets");
            let labels_val: serde_json::Value = row.get("labels");

            let targets: Vec<String> = serde_json::from_value(targets_val).unwrap_or_default();
            let labels: HashMap<String, String> = serde_json::from_value(labels_val).unwrap_or_default();

            results.push(NarthexSDResp { targets, labels });
        }

        Ok(results)
    }
}
