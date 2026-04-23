use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddTargetReq {
    pub address: String,
}
