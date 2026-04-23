use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NarthexSDResp {
    pub targets: Vec<String>,
    pub labels: HashMap<String, String>,
}
