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

    let database_file = get_database_filename();
    SqliteConnection::establish(&database_file)
        .expect(&format!("Error connecting to database '{}'.", database_file))
}

fn is_dev() -> bool {
    match env::var("TEST") {
        Ok(_s) => true,
        _ => false,
    }
}

fn get_database_filename() -> String {
    let filename = env::var("MEDIAMATE_DB");

    match filename {
        Ok(filename) => { filename }
        Err(_) => {
            env::var("HOME").unwrap() + "/.local/mediamate/database.db"
        }
    }
}
