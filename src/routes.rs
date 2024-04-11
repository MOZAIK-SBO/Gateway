use crate::{
    guards::JwtToken,
    types::{CipherTextValue, EncryptedIngestMetricEvent, IndexResponse, IngestMetricEvent},
};
use client_auth::AuthToken;
use libmozaik_iot::{protect, DeviceState, ProtectionAlgorithm};
use rocket::{http::Status, serde::json::Json, tokio::sync::Mutex, State};
use std::{borrow::BorrowMut, env, sync::Arc};

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

async fn encrypt_data(data: &Vec<u8>, device_state: Arc<Mutex<DeviceState>>) -> Vec<u8> {
    let client_id = env::var("CLIENT_ID").unwrap();

    let Ok(ct_metric_event) = protect(
        &client_id,
        device_state.lock().await.borrow_mut(),
        ProtectionAlgorithm::AesGcm128,
        data,
    ) else {
        panic!("Sample encryption error. Sample: {:02X?}", &data);
    };

    ct_metric_event
}

async fn ingest_obelisk(
    dataset_id: &str,
    ingest_event: &IngestMetricEvent,
    encrypted_data: &[u8],
    token: &String,
) -> Status {
    let mozaik_obelisk_endpoint =
        env::var("MOZAIK_OBELISK_ENDPOINT").unwrap_or("http://localhost:3000".to_string());

    let client = reqwest::Client::new();

    let Ok(res) = client
        .post(format!(
            "{mozaik_obelisk_endpoint}/data/ingest/{dataset_id}"
        ))
        .bearer_auth(token)
        .json(&vec![EncryptedIngestMetricEvent {
            timestamp: ingest_event.timestamp,
            metric: ingest_event.metric.clone(),
            value: CipherTextValue {
                c: encrypted_data.to_vec(),
            },
            source: ingest_event.source.clone(),
        }])
        .send()
        .await
    else {
        return Status::InternalServerError;
    };

    Status::from_code(res.status().as_u16()).unwrap_or(Status::InternalServerError)
}

// IoT device is authenticated using JWT
#[post("/<dataset_id>", format = "application/json", data = "<ingest_event>")]
pub async fn ingest_authenticated(
    dataset_id: &str,
    ingest_event: Json<IngestMetricEvent>,
    jwt_token: JwtToken,
    device_state: &State<Arc<Mutex<DeviceState>>>,
) -> Status {
    ingest_obelisk(
        dataset_id,
        &ingest_event,
        &encrypt_data(&ingest_event.value, device_state.inner().clone()).await,
        &jwt_token.0,
    )
    .await
}

// IoT device is not authenticated using JWT -> gateway will authenticate with API
#[post(
    "/<dataset_id>",
    format = "application/json",
    data = "<ingest_event>",
    rank = 2
)]
pub async fn ingest_unauthenticated(
    dataset_id: &str,
    ingest_event: Json<IngestMetricEvent>,
    auth: &State<Arc<Mutex<AuthToken>>>,
    device_state: &State<Arc<Mutex<DeviceState>>>,
) -> Status {
    ingest_obelisk(
        dataset_id,
        &ingest_event,
        &encrypt_data(&ingest_event.value, device_state.inner().clone()).await,
        &auth.inner().lock().await.token().await,
    )
    .await
}
