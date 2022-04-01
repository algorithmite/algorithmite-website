#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate argon2;
extern crate chrono;

use rocket::fs::{relative, FileServer, NamedFile};
use rocket_sync_db_pools::database;
use std::path::Path;

pub mod model;

#[database("algorithmite")]
struct AlgorithmiteDbConn(diesel::PgConnection);

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("../client/static/index.html")))
        .await
        .ok()
}

#[get("/pkg/algorithmite_website_client.wasm")]
async fn wasm() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!(
        "../client/pkg/algorithmite_website_client_bg.wasm"
    )))
    .await
    .ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("../client/static/favicon.ico")))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon, wasm])
        .mount("/static", FileServer::from(relative!("../client/static")))
        .attach(AlgorithmiteDbConn::fairing())
}
