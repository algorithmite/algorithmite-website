#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derive_enum;
extern crate chrono;
use rocket_sync_db_pools::{database};
//pub mod schema;
pub mod models;
pub mod action_schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
