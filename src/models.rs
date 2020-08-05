
#[derive(Queryable)]
pub struct Episode {
    pub id: i32,
    pub path: String,
    pub serie_id: i32,
    pub season_id: i32,
    pub episode_id: i32,
    pub tmdb_id: i32,
    pub title: String,
    pub description: String,
}

use crate::schema::episode;
#[derive(Insertable)]
#[table_name = "episode"]
pub struct NewEpisode<'a> {
    pub path: &'a String,
    pub serie_id: &'a i32,
    pub season_id: &'a i32,
    pub episode_id: &'a i32,
    pub tmdb_id: &'a i32,
    pub title: &'a String,
    pub description: &'a String,
}


#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub path: String,
    pub title: String,
    pub imagepath: String,
    pub description: String,
}

use crate::schema::movie;
#[derive(Insertable)]
#[table_name = "movie"]
pub struct NewMovie<'a> {
    pub path: &'a String,
    pub title: &'a String,
    pub imagepath: &'a String,
    pub description: &'a String,
}

use crate::schema::season;
#[derive(Insertable)]
#[table_name = "season"]
pub struct NewSeason<'a> {
    pub imagepath: &'a String,
    pub tmdb_id: &'a i32,
    pub description: &'a String,
    pub title: &'a String,
}

#[derive(Queryable)]
pub struct Season {
    pub id: i32,
    pub imagepath:String,
    pub tmdb_id:i32,
    pub title:String,
    pub description:String,
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
use crate::establish_connection;

#[derive(Insertable, Debug)]
#[table_name = "serie"]
pub struct NewSerie<'a> {
    pub imagepath: &'a String,
    pub tmdb_id: &'a i32,
    pub title: &'a String,
    pub description: &'a String,
}

#[derive(Queryable)]
pub struct Serie {
    pub id: i32,
    pub imagepath: String,
    pub tmdb_id: i32,
    pub title: String,
    pub description: String,
}

impl NewSerie<'_> {

pub fn check_serie(&self, id_to_check: i32) -> bool {
    use crate::schema::serie::dsl::*;

    use diesel::prelude::*;
    use crate::models::*;
    let connection = establish_connection();
        let results = serie
            .filter(tmdb_id.eq(id_to_check))
            .load::<Serie>(&connection)
            .expect("Error loading Serie Table");

        if results.len() >= 1 {
            return true;
        }
        return false;
    }


}

