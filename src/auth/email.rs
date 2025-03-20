use anyhow::Error;
use rocket::fairing::{AdHoc, Fairing};
use rocket::get;
use rocket::http::{CookieJar, Status};
use rocket::response::Debug;
use rocket::response::Redirect;

use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Deserialize, Serialize)]
struct EmailUserInfo {
    email: String,
}
impl From<EmailUserInfo> for User {
    fn from(info: EmailUserInfo) -> Self {
        User {
            id: info.email.clone(),
            username: info.email,
        }
    }
}
#[get("/login/email")]
fn email_login<'a>(cookies: &'a CookieJar<'a>) -> (Status, &'a str) {
    // Create magic link
    // Send email containing magic link
    (Status::Ok, "Email link sent")
}

#[get("/auth/email")]
async fn email_callback(cookies: &CookieJar<'_>) -> Result<Redirect, Debug<Error>> {
    // Validate magic link
    // create token
    // Set a cookie with the user's name, and redirect to the home page.
    // cookies.add(
    //     Cookie::build(("access_token", create_user_token(user_info.into()).unwrap()))
    //         .same_site(SameSite::Lax)
    //         .build(),
    // );
    Ok(Redirect::to("/"))
}

pub fn email_fairing() -> impl Fairing {
    AdHoc::on_ignite("Email Auth", |rocket| async {
        rocket.mount("/api", rocket::routes![email_login, email_callback])
    })
}
