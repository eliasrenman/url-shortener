use self::db::models::Urls;
use super::schema::urls::dsl::*;
use super::schema::urls::*;
use crate::*;
use chrono::NaiveDateTime;
use diesel::{delete, dsl::insert_into, prelude::*, result::Error};

pub fn get_entry(path: &str) -> Result<Urls, Error> {
    let connection = &mut establish_connection();
    urls.find(path).select(Urls::as_select()).first(connection)
}

pub fn upsert_entry(
    username: &str,
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
            owned_by.eq(username),
        ))
        .on_conflict(url)
        .do_update()
        .set((
            destination_url.eq(dest),
            ttl.eq(time_to_live),
            owned_by.eq(username),
        ))
        .execute(connection)
}

pub fn delete_entry(path: &str) -> Result<usize, Error> {
    let connection = &mut establish_connection();
    delete(urls.filter(url.eq(path))).execute(connection)
}
