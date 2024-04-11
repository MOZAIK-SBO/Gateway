#[macro_use]
extern crate rocket;
use std::env;
use std::sync::Arc;

use client_auth::AuthToken;
use dotenv::dotenv;

use libmozaik_iot::DeviceState;
use rocket::tokio::sync::Mutex;
use routes::index;
use routes::ingest_authenticated;
use routes::ingest_unauthenticated;

pub mod guards;
pub mod routes;
pub mod types;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let auth_endpoint = env::var("AUTH_ENDPOINT").unwrap();
    let token_endpoint = env::var("TOKEN_ENDPOINT").unwrap();

    // Auth token
    let auth_token = Arc::new(Mutex::new(
        AuthToken::new(client_id, client_secret, auth_endpoint, token_endpoint).await,
    ));

    // nonce + key
    let nonce = [
        0x73, 0x3f, 0x77, 0x3e, 0x1d, 0x5f, 0xa3, 0xdf, 0x5e, 0x05, 0x6b, 0xf5,
    ]; // this should be a fresh nonce

    let key = [
        0x8a, 0x47, 0xc0, 0x45, 0x16, 0x7b, 0x1a, 0xd4, 0x49, 0x46, 0x85, 0xa5, 0x20, 0xd0, 0xd6,
        0x9e,
    ]; // this should be a fresh device key

    let device_state = Arc::new(Mutex::new(DeviceState::new(nonce, key)));

    rocket::build()
        .manage(auth_token)
        .manage(device_state)
        .mount("/", routes![index])
        .mount(
            "/ingest",
            routes![ingest_authenticated, ingest_unauthenticated],
        )
}
