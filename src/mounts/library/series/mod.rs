use diesel::prelude::*;
use mpv_webrpc::models::*;
use mpv_webrpc::schema::serie::dsl::*;

pub fn get_series() -> Vec<mpv_webrpc::models::Serie> {
    let connection = mpv_webrpc::establish_connection();
    serie
        .load::<Serie>(&connection)
        .expect("Error loading Serie Table")
}

pub fn get_detail(series_id: i32) -> Vec<mpv_webrpc::models::Serie> {
    let connection = mpv_webrpc::establish_connection();
    serie
        .filter(tmdb_id.eq(series_id))
        .load::<Serie>(&connection)
        .expect("Error loading Serie Table")
}
