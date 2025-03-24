use self::db::models::Urls;
use super::schema::urls::dsl::*;
use super::schema::urls::*;
use crate::*;
use chrono::NaiveDateTime;
use diesel::{associations::HasTable, delete, dsl::insert_into, prelude::*, result::Error, select};

pub fn get_entry(path: &str) -> Result<Urls, Error> {
    let connection = &mut establish_connection();
    urls.find(path).select(Urls::as_select()).first(connection)
}

pub fn upsert_entry(
    owned_by_id: &str,
    path: &str,
    dest: &str,
    time_to_live: Option<NaiveDateTime>,
) -> Result<usize, Error> {
    let connection = &mut establish_connection();
    insert_into(table)
        .values((
            url.eq(path),
            destination_url.eq(dest),
            ttl.eq(time_to_live),
            owned_by.eq(owned_by_id),
        ))
        .on_conflict(url)
        .do_update()
        .set((
            destination_url.eq(dest),
            ttl.eq(time_to_live),
            owned_by.eq(owned_by_id),
        ))
        .execute(connection)
}

pub fn delete_entry(owned_by_id: &str, path: &str) -> Result<usize, Error> {
    let connection = &mut establish_connection();
    delete(urls.filter(url.eq(path)).filter(owned_by.eq(owned_by_id))).execute(connection)
}

pub fn list(owned_by_id: &str) -> Vec<Urls> {
    let connection = &mut establish_connection();
    urls.filter(owned_by.eq(owned_by_id))
        .select(Urls::as_select())
        .load(connection)
        .expect("Error loading urls")
}
