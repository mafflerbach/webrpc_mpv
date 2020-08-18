use diesel::prelude::*;
use mpv_webrpc::models::*;
use mpv_webrpc::schema::serie::dsl::*;

use serde::{Deserialize, Serialize};

pub fn get_series() -> Vec<mpv_webrpc::models::Serie> {
    let connection = mpv_webrpc::establish_connection();
    serie
        .load::<Serie>(&connection)
        .expect("Error loading Serie Table")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerieSeasonAggregation {
    serie: mpv_webrpc::models::Serie,
    seasons: Vec<Vec<mpv_webrpc::models::Season>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerieSeasonAggregationCollection {
    series_collection: Vec<SerieSeasonAggregation>,
}

pub fn get_series_season_aggregation() -> SerieSeasonAggregationCollection {
    use mpv_webrpc::schema::season::dsl::*;
    let connection = mpv_webrpc::establish_connection();
    let seriesa = serie
        .load::<Serie>(&connection)
        .expect("Error loading Serie Table");

    let mut collection = Vec::new();

    for s in seriesa {
        let seasons = season
            .filter(tmdb_id.eq(s.tmdb_id))
            .load::<Season>(&connection)
            .expect("Error loading Season Table");

        let mut season_vec = Vec::new();
        season_vec.push(seasons);
        let agrre = SerieSeasonAggregation {
            serie: s,
            seasons: season_vec,
        };

        collection.push(agrre);
    }

    let return_struct = SerieSeasonAggregationCollection { series_collection: collection };

    return_struct
}

pub fn get_detail(series_id: i32) -> Vec<mpv_webrpc::models::Serie> {
    let connection = mpv_webrpc::establish_connection();
    serie
        .filter(tmdb_id.eq(series_id))
        .load::<Serie>(&connection)
        .expect("Error loading Serie Table")
}
