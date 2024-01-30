use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    pub name: String,
    pub version: String,
    pub contact: Vec<String>,
}
