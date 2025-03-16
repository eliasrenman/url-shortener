mod db;
mod dto;
mod handlers;

use db::establish_connection;
use handlers::url::{handle_delete, handle_redirect, handle_upsert};
use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::{http::Status, response::Redirect};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![shortner,])
        .mount("/api", routes![upsert, delete])
        .mount("/", FileServer::from(relative!("web/dist")))
}
#[get("/<url>")]
fn shortner(url: &str) -> Result<Redirect, (Status, &'static str)> {
    handle_redirect(url)
}

#[post("/write", format = "json", data = "<upsert>")]
fn upsert(upsert: Json<dto::UpsertUrlDto<'_>>) -> (Status, &'static str) {
    handle_upsert(upsert.0)
}

#[delete("/<url>")]
fn delete(url: &str) -> (Status, &'static str) {
    handle_delete(url)
}
