use diesel::{Connection, sqlite::SqliteConnection};

use std::env;
pub mod models;
pub mod schema;
pub mod url;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
