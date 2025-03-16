use rocket::{http::Status, response::Redirect};

use crate::db::url::get_entry;

pub fn handle_redirect(url: &str) -> Result<Redirect, (Status, &'static str)> {
    let row = get_entry(url).map_err(|_| (Status::NotFound, "Redirect Not found"))?;
    Ok(Redirect::to(row.destination_url))
}
