use diesel::prelude::*;
use mpv_webrpc::models::*;
use mpv_webrpc::schema::episode::dsl::*;

pub fn get_episodes(ser_id:i32, sea_id:i32) -> Vec<mpv_webrpc::models::Episode> {
    let connection = mpv_webrpc::establish_connection();
    episode
        .filter(tmdb_id.eq(ser_id))
        .filter(season_id.eq(sea_id))
        .order_by(episode_id.desc())
        .load::<Episode>(&connection)
        .expect("Error loading Episode Table")
}

pub fn get_detail(ser_id:i32, sea_id:i32, epi_id:i32) -> Vec<mpv_webrpc::models::Episode> {
    let connection = mpv_webrpc::establish_connection();
    episode

        .filter(tmdb_id.eq(ser_id))
        .filter(season_id.eq(sea_id))
        .filter(episode_id.eq(epi_id))
        .order_by(episode_id.desc())
        .load::<Episode>(&connection)
        .expect("Error loading Episode Table")
}



