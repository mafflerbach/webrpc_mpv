use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
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

impl NewEpisode<'_> {
    pub fn check_episode(&self) -> bool {
        use crate::schema::episode::dsl::*;
        use diesel::prelude::*;
        let connection = establish_connection();
        let results = episode
            .filter(tmdb_id.eq(&self.tmdb_id))
            .filter(season_id.eq(&self.season_id))
            .filter(episode_id.eq(&self.episode_id))
            .load::<Episode>(&connection)
            .expect("Error loading episode Table");

        if results.len() >= 1 {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub path: String,
    pub title: String,
    pub imagepath: String,
    pub description: String,
    pub tmdb_id: i32,
}

use crate::schema::movie;
#[derive(Insertable)]
#[table_name = "movie"]
pub struct NewMovie<'a> {
    pub path: &'a String,
    pub title: &'a String,
    pub imagepath: &'a String,
    pub description: &'a String,
    pub tmdb_id: &'a i32,
}

use crate::schema::season;
#[derive(Insertable)]
#[table_name = "season"]
pub struct NewSeason<'a> {
    pub imagepath: &'a String,
    pub tmdb_id: &'a i32,
    pub description: &'a String,
    pub title: &'a String,
    pub season_id: &'a i32,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Season {
    pub id: i32,
    pub imagepath: String,
    pub tmdb_id: i32,
    pub title: String,
    pub description: String,
    pub season_id: i32,
}

impl NewSeason<'_> {
    pub fn check_season(&self) -> bool {
        use crate::schema::season::dsl::*;
        use diesel::prelude::*;

        let connection = establish_connection();
        let results = season
            .filter(tmdb_id.eq(&self.tmdb_id))
            .filter(season_id.eq(&self.season_id))
            .load::<Season>(&connection)
            .expect("Error loading ingnored Table");

        if results.len() >= 1 {
            return true;
        }
        return false;
    }
}

use crate::schema::ignored;
#[derive(Insertable)]
#[table_name = "ignored"]
pub struct NewIgnored<'a> {
    pub tmdb_id: &'a i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Ignored {
    pub id: i32,
    pub tmdb_id: i32,
}

impl NewIgnored<'_> {
    pub fn is_ignored(&self) -> bool {
        use crate::schema::ignored::dsl::*;
        use diesel::prelude::*;

        let connection = establish_connection();
        let results = ignored
            .filter(tmdb_id.eq(&self.tmdb_id))
            .load::<Ignored>(&connection)
            .expect("Error loading ingnored Table");

        if results.len() >= 1 {
            return true;
        }
        return false;
    }
}

use crate::establish_connection;
use crate::schema::serie;

#[derive(Insertable, Debug)]
#[table_name = "serie"]
pub struct NewSerie<'a> {
    pub imagepath: &'a String,
    pub tmdb_id: &'a i32,
    pub title: &'a String,
    pub description: &'a String,
}

#[derive(Queryable, Serialize, Clone, Deserialize, Debug)]
pub struct Serie {
    pub id: i32,
    pub imagepath: String,
    pub tmdb_id: i32,
    pub title: String,
    pub description: String,
}

impl NewSerie<'_> {
    pub fn check_serie(&self) -> bool {
        use crate::models::*;
        use crate::schema::serie::dsl::*;
        use diesel::prelude::*;

        let connection = establish_connection();
        let results = serie
            .filter(tmdb_id.eq(&self.tmdb_id))
            .load::<Serie>(&connection)
            .expect("Error loading Serie Table");

        if results.len() >= 1 {
            return true;
        }
        return false;
    }
}
