use super::schema::urls;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Selectable, AsChangeset)]
#[serde(rename_all(serialize = "camelCase"))]
#[diesel(table_name = urls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Urls {
    pub url: String,
    pub destination_url: String,
    pub ttl: Option<NaiveDateTime>,
    pub owned_by: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
