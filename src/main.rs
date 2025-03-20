mod auth;
mod db;
mod dto;
mod handlers;

use auth::github::github_fairing;
use auth::jwts::jwts_decode;
use db::establish_connection;
use dotenvy::dotenv;
use handlers::url::{handle_delete, handle_redirect, handle_upsert};
use rocket::fs::{FileServer, relative};
use rocket::http::{Cookie, CookieJar};
use rocket::request;
use rocket::serde::json::Json;
use rocket::{http::Status, response::Redirect};
#[macro_use]
extern crate rocket;

struct User {
    pub id: String,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<User, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        if let Some(cookie) = cookies.get("access_token") {
            let decoded = jwts_decode(cookie.value());
            if decoded.is_ok() {
                let unwrapped = decoded.unwrap();
                return request::Outcome::Success(User {
                    username: unwrapped.claims.name,
                    id: unwrapped.claims.sub,
                });
            }
        }

        request::Outcome::Forward(Status::Unauthorized)
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![shortner, logout])
        .mount("/api", routes![upsert, delete])
        .mount("/", FileServer::from(relative!("web/dist")))
        .attach(github_fairing())
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("username"));
    Redirect::to("/")
}

#[get("/<url>")]
fn shortner(url: &str) -> Result<Redirect, (Status, &'static str)> {
    handle_redirect(url)
}

#[post("/write", format = "json", data = "<upsert>")]
fn upsert(user: User, upsert: Json<dto::UpsertUrlDto<'_>>) -> (Status, &'static str) {
    handle_upsert(&user.id.as_str(), upsert.0)
}

#[delete("/<url>")]
fn delete(user: User, url: &str) -> (Status, &'static str) {
    handle_delete(&user.id.as_str(), url)
}
