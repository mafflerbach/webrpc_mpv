use diesel::prelude::*;
use mpv_webrpc::models::*;
use mpv_webrpc::schema::movie::dsl::*;


pub fn get_movies() -> Vec<mpv_webrpc::models::Movie> {
    let connection = mpv_webrpc::establish_connection();
    movie
        .load::<Movie>(&connection)
        .expect("Error loading Movie Table")
}

pub fn get_detail(movies_id: i32) -> Vec<mpv_webrpc::models::Movie> {
    let connection = mpv_webrpc::establish_connection();
    movie
        .filter(tmdb_id.eq(movies_id))
        .load::<Movie>(&connection)
        .expect("Error loading movie Table")
}
