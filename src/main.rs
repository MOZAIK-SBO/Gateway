#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use std::env;

use routes::index;
use routes::ingest;

pub mod routes;
pub mod types;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let OBELISK_ENDPOINT =
        env::var("MOZAIK_OBELISK_ENDPOINT").unwrap_or("http://localhost:3000/".to_string());

    rocket::build()
        .mount("/", routes![index])
        .mount("/ingest", routes![ingest])
}
