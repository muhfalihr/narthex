use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpsertLabelReq {
    pub key: String,
    pub value: String,
}
