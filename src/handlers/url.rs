use rocket::{http::Status, response::Redirect};

use crate::{
    db::url::{delete_entry, get_entry, upsert_entry},
    dto::UpsertUrlDto,
};

pub fn handle_redirect(url: &str) -> Result<Redirect, (Status, &'static str)> {
    let row = get_entry(url).map_err(|_| (Status::NotFound, "Redirect Not found"))?;
    Ok(Redirect::to(row.destination_url))
}

pub fn handle_upsert(dto: UpsertUrlDto<'_>) -> (Status, &'static str) {
    let row = upsert_entry(dto.url, dto.destination_url, dto.ttl);
    if row.is_err() {
        return (Status::BadRequest, "Failed to upsert redirect");
    }
    (Status::Ok, "Successfully upserted redirect")
}

pub fn handle_delete(url: &str) -> (Status, &'static str) {
    let row = delete_entry(url);
    if row.is_err() {
        return (Status::BadRequest, "Failed to delete redirect");
    }
    (Status::Ok, "Successfully deleted redirect")
}
