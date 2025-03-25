use rocket::{http::Status, response::Redirect};

use crate::{
    db::url::{delete_entry, get_entry, list, upsert_entry},
    dto::UpsertUrlDto,
};

pub fn handle_redirect(url: &str) -> Result<Redirect, (Status, &'static str)> {
    let row = get_entry(url).map_err(|_| (Status::NotFound, "Redirect Not found"))?;
    Ok(Redirect::to(row.destination_url))
}

pub fn handle_upsert(owned_by: &str, url: &str, dto: UpsertUrlDto<'_>) -> (Status, &'static str) {
    let naive_local_datetime = dto.ttl.map(|dt| dt.naive_utc());
    let row = upsert_entry(owned_by, url, dto.destination_url, naive_local_datetime);
    if row.is_err() {
        return (Status::BadRequest, "Failed to upsert redirect");
    }
    (Status::Ok, "Successfully upserted redirect")
}

pub fn handle_delete(owned_by: &str, url: &str) -> (Status, &'static str) {
    let row = delete_entry(owned_by, url);
    if row.is_err() {
        return (Status::BadRequest, "Failed to delete redirect");
    }
    (Status::Ok, "Successfully deleted redirect")
}

pub fn handle_list(owned_by: &str) -> String {
    let urls = list(owned_by);
    serde_json::to_string(&urls).unwrap()
}
