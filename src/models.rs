
#[derive(Queryable)]
pub struct Episode {
    pub id: i32,
    pub video_id: i32,
    pub title: String,
    pub description: String,
}

use crate::schema::episode;
#[derive(Insertable)]
#[table_name = "episode"]
pub struct NewEpisode<'a> {
    pub video_id: &'a i32,
    pub title: &'a String,
    pub description: &'a String,
}


#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub video_id: i32,
    pub title: String,
    pub description: String,
}

use crate::schema::movie;
#[derive(Insertable)]
#[table_name = "movie"]
pub struct NewMovie<'a> {
    pub video_id: &'a i32,
    pub title: &'a String,
    pub description: &'a String,
}

#[derive(Queryable)]
pub struct Video {
    pub id: i32,
    pub path: String,
    pub imagepath: String,
}

use crate::schema::video;
#[derive(Insertable)]
#[table_name = "video"]
pub struct NewVideo<'a> {
    pub path: &'a String,
    pub imagepath: &'a String,
}

use crate::schema::season;
#[derive(Insertable)]
#[table_name = "season"]
pub struct NewSeason<'a> {
    pub video_id: &'a i32,
    pub serie_id: &'a i32,
}

#[derive(Queryable)]
pub struct Season {
    pub id: i32,
    pub video_id: i32,
    pub serie_id: i32,
}

use crate::schema::ignored;
#[derive(Insertable)]
#[table_name = "ignored"]
pub struct NewIgnored<'a> {
    pub tmdb_id: &'a i32,
}

#[derive(Queryable)]
pub struct Ignored {
    pub id: i32,
    pub tmdb_id: i32,
}
use crate::schema::serie;
#[derive(Insertable)]
#[table_name = "serie"]
pub struct NewSerie<'a> {
    pub tmdb_id: &'a i32,
    pub title: &'a String,
    pub imagepath: &'a String,
    pub description: &'a String,
}

#[derive(Queryable)]
pub struct Serie {
    pub id: i32,
    pub tmdb_id: i32,
    pub title: String,
    pub imagepath: String,
    pub description: String,
}
