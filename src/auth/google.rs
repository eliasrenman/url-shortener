use anyhow::{Context, Error};
use reqwest::header::AUTHORIZATION;
use rocket::fairing::{AdHoc, Fairing};
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Debug, Redirect};
use rocket::serde::json::Value;
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::User;

use super::jwts::{create_user_token, generate_id};

/// User information to be retrieved from the Google People API.
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
struct GoogleUserInfo {
    names: Vec<Value>,
    resource_name: String,
}
impl From<GoogleUserInfo> for User {
    fn from(info: GoogleUserInfo) -> Self {
        User {
            username: info
                .names
                .iter()
                .find_map(|name| {
                    let metadata = name.get("metadata")?;
                    let primary = metadata.get("primary")?.as_bool()?;

                    if primary {
                        name.get("displayName")?.as_str().map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .unwrap(),
            id: generate_id("github", format!("{}", info.resource_name).as_str()),
        }
    }
}
#[get("/login/google")]
fn google_login(oauth2: OAuth2<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["profile"]).unwrap()
}

#[get("/auth/google")]
async fn google_callback(
    token: TokenResponse<GoogleUserInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    // Use the token to retrieve the user's Google account information.
    let user_info: GoogleUserInfo = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://people.googleapis.com/v1/people/me?personFields=names")
        .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
        .send()
        .await
        .context("failed to complete request")?
        .json()
        .await
        .context("failed to deserialize response")?;

    // Set a cookie with the user's name, and redirect to the home page.
    cookies.add(
        Cookie::build(("access_token", create_user_token(user_info.into()).unwrap()))
            .same_site(SameSite::Lax)
            .build(),
    );

    Ok(Redirect::to("/"))
}
pub fn google_fairing() -> impl Fairing {
    AdHoc::on_ignite("Google OAuth2", |rocket| async {
        rocket
            .mount("/", rocket::routes![google_login, google_callback])
            .attach(OAuth2::<GoogleUserInfo>::fairing("google"))
    })
}
