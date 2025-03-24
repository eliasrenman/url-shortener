use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertUrlDto<'r> {
    pub destination_url: &'r str,
    pub ttl: Option<NaiveDateTime>,
}
