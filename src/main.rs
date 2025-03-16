use db::establish_connection;
use handlers::url::handle_redirect;
use rocket::{http::Status, response::Redirect};
mod db;
mod handlers;
use rocket::fs::{FileServer, relative};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![shortner])
        .mount("/", FileServer::from(relative!("web/dist")))
}
#[get("/<url>")]
fn shortner(url: &str) -> Result<Redirect, (Status, &'static str)> {
    handle_redirect(url)
}
