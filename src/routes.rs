use rocket::serde::json::Json;

use crate::types::IndexResponse;

#[get("/")]
pub fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        contact: env!("CARGO_PKG_AUTHORS")
            .split(':')
            .map(str::to_string)
            .collect(),
    })
}

// TODO: encrypt ingested data and forward to MOZAIK-Obelisk ingest endpoint
#[post("/")]
pub fn ingest() -> &'static str {
    "ingest"
}
