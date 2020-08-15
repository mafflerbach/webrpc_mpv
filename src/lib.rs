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
        env::set_var("DATABASE_URL", "db/restmpv_test.db");
        database_url="db/restmpv_test.db".to_string();
    }

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn is_dev() -> bool {
    match env::var("TEST") {
        Ok(_s) => true,
        _ => false,
    }
}
