pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    
    let mut database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if is_dev() {
        database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");
    }

    println!("{}", database_url);
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn is_dev() -> bool {
    match env::var("TEST") {
        Ok(s) => s == "yes",
        _ => false,
    }
}
