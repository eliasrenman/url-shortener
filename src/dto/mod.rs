use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
pub struct UpsertUrlDto<'r> {
    pub url: &'r str,
    pub destination_url: &'r str,
    pub ttl: Option<NaiveDateTime>,
}
