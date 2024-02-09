#[macro_use]
extern crate rocket;
use dotenv::dotenv;

use routes::index;
use routes::ingest;

pub mod guards;
pub mod routes;
pub mod types;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .mount("/ingest", routes![ingest])
}
