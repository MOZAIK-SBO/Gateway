use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct IndexResponse {
    pub name: String,
    pub version: String,
    pub contact: Vec<String>,
}

pub type IngestBatch = Vec<IngestMetricEvent>;

#[derive(Serialize, Deserialize, Debug)]
pub struct IngestMetricEvent {
    pub timestamp: Option<i32>,
    pub metric: String,
    pub value: Vec<u8>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub location: Option<Location>,
    pub elevation: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: i32,
    pub lng: i32,
}
