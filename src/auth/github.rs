use super::jwts::{create_user_token, generate_id};
use crate::User;
use anyhow::{Context, Error};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::fairing::{AdHoc, Fairing};
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Debug;
use rocket::response::Redirect;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};

/// User information to be retrieved from the GitHub API.
#[derive(Deserialize, Serialize)]
struct GitHubUserInfo {
    name: String,
    id: i64,
}
impl From<GitHubUserInfo> for User {
    fn from(info: GitHubUserInfo) -> Self {
        User {
            username: info.name,
            id: generate_id("github", format!("{}", info.id).as_str()),
        }
    }
}
// NB: Here we are using the same struct as a type parameter to OAuth2 and
// TokenResponse as we use for the user's GitHub login details. For
// `TokenResponse` and `OAuth2` the actual type does not matter; only that they
// are matched up.
#[get("/login/github")]
fn github_login(oauth2: OAuth2<GitHubUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(cookies, &["user:read", "user:email"])
        .unwrap()
}

#[get("/auth/github")]
async fn github_callback(
    token: TokenResponse<GitHubUserInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    // Use the token to retrieve the user's GitHub account information.
    let user: GitHubUserInfo = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://api.github.com/user")
        .header(AUTHORIZATION, format!("token {}", token.access_token()))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "rocket_oauth2 demo application")
        .send()
        .await
        .context("failed to complete request")?
        .json()
        .await
        .context("failed to deserialize response")?;

    // Set a cookie with the user's name, and redirect to the home page.
    cookies.add(
        Cookie::build(("access_token", create_user_token(user.into()).unwrap()))
            .same_site(SameSite::Lax)
            .build(),
    );
    Ok(Redirect::to("/"))
}
pub fn github_fairing() -> impl Fairing {
    AdHoc::on_ignite("Github OAuth2", |rocket| async {
        rocket
            .mount("/", rocket::routes![github_login, github_callback])
            .attach(OAuth2::<GitHubUserInfo>::fairing("github"))
    })
}
