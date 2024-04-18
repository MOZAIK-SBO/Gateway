use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct IndexResponse {
    pub name: String,
    pub version: String,
    pub contact: Vec<String>,
}

// pub type IngestBatch = Vec<IngestMetricEvent>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IngestMetricEvent {
    pub timestamp: u128,
    pub metric: String,
    pub value: Vec<u8>,
    pub source: String,
    // pub tags: Option<Vec<String>>,
    // pub location: Option<Location>,
    // pub elevation: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedIngestMetricEvent {
    pub timestamp: u128,
    pub metric: String,
    pub value: CipherTextValue,
    pub source: String,
    // pub tags: Option<Vec<String>>,
    // pub location: Option<Location>,
    // pub elevation: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Location {
    pub lat: i32,
    pub lng: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CipherTextValue {
    pub c: Vec<u8>,
}
