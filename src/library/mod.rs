pub mod favourites;
pub mod movies;
use std::{fs, io};

use crate::settings;
use crate::tmdb;
use crate::tmdb::tmdb::SearchResult;
use diesel::prelude::*;
use glob::glob;
use lazy_static::lazy_static;
use mpv_webrpc::models::*;
use regex::Regex;
use std::path::Path;

pub fn scan_movies(mut results: Vec<SearchResult>) -> Vec<SearchResult> {
    let settings = settings::init();
    let path = settings.scan_dir_movies.clone();

    let mkv_pattern = format!("{}/**/*.mkv", path);
    let mp4_pattern = format!("{}/**/*.mp4", path);
    let webm_pattern = format!("{}/**/*.webm", path);

    for entry in glob(&mkv_pattern)
        .unwrap()
            .chain(glob(&mp4_pattern).unwrap())
            .chain(glob(&webm_pattern).unwrap())
            {
                let file_path = entry.unwrap().into_os_string().into_string().unwrap();
                let name_of_file = Path::new(&file_path).file_name();

                let connection = mpv_webrpc::establish_connection();
                use diesel::prelude::*;
                use mpv_webrpc::schema::movie::dsl::*;

                let movie_result = movie
                    .filter(path.eq(&file_path))
                    .load::<Movie>(&connection)
                    .expect("Error loading Movie Table");

                if movie_result.len() > 0 {
                    continue;
                }
                let tjson = SearchResult {
                    name: name_of_file.unwrap().to_str().unwrap().to_string(),
                    id: 0,
                    poster_path: serde::export::Some("".to_string()),
                    file_path: serde::export::Some(file_path),
                    overview: serde::export::Some("overview".to_string()),
                    type_of: serde::export::Some("movie".to_string()),
                };

                let mut test = Vec::new();
                test.push(tjson);
                results.append(&mut test);
            }

    results
}

use crate::mpv;
pub fn get_video_status(path_to_check : String) -> mpv::mpv::Property {
    let connection = mpv_webrpc::establish_connection();
    use diesel::prelude::*;
    use mpv_webrpc::schema::video_status::dsl::*;

    let status = video_status
        .filter(path.eq(&path_to_check))
        .load::<VideoStatus>(&connection)
        .expect("Error loading video_status Table");

    let mut video_time_status = 0.0;
    if status.len() >= 1 {
        for res in status {
            video_time_status = res.time;
        }
    }
    let me = mpv::mpv::Property {
        error : String::from("success"),
        data : video_time_status.to_string()
    };
    return me
}

pub fn get_first_level() -> Vec<String> {
    let settings = settings::init();
    let base_path = settings.scan_dir_series;

    let mut stack = Vec::new();

    let mut entries = fs::read_dir(&base_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    entries.sort();
    for entry in &entries {
        let foo = entry.display().to_string().replace(&base_path, "");
        stack.push(foo)
    }
    return stack;
}

fn sync_season(tmdb_id_to_insert: i32, season_id_to_insert: i32) {
    use mpv_webrpc::schema::season;
    let season_in = tmdb::tmdb::tv_season_get_details(tmdb_id_to_insert, season_id_to_insert);
    let season_info = NewSeason {
        season_id: &season_id_to_insert,
        tmdb_id: &tmdb_id_to_insert,
        title: &season_in.name,
        imagepath: &season_in.poster_path,
        description: &season_in.overview,
    };

    let connection = mpv_webrpc::establish_connection();
    let _insert_result = diesel::insert_into(season::table)
        .values(&season_info)
        .execute(&connection);
    println!("Insert done for season");
}



pub fn sync_episodes(path: String, tmdb_id: i32) {
    let mkv_pattern = format!("{}/**/*.mkv", path);
    let mp4_pattern = format!("{}/**/*.mp4", path);
    let webm_pattern = format!("{}/**/*.webm", path);

    use mpv_webrpc::schema::episode;
    for entry in glob(&mkv_pattern)
        .unwrap()
            .chain(glob(&mp4_pattern).unwrap())
            .chain(glob(&webm_pattern).unwrap())
            {
                match entry {
                    Ok(path) => {
                        println!("fetch for episodes and season in path: {:?}", path);
                        let file_name = &path.clone().into_os_string().into_string().unwrap();
                        let captures = parsing_season_and_episode(file_name);
                        //
                        if captures.is_none() {
                            println!("Pattern not match");
                            continue;
                        }

                        let unwrap_cap = captures.unwrap();
                        let s = unwrap_cap
                            .get(1)
                            .map_or("", |m| m.as_str())
                            .replace("S", "");
                        let season: i32 = s.replace("s", "").parse::<i32>().unwrap();

                        let e = unwrap_cap
                            .get(2)
                            .map_or("", |m| m.as_str())
                            .replace("E", "");
                        let episode: i32 = e.replace("e", "").parse::<i32>().unwrap();

                        let season_info = NewSeason {
                            season_id: &season,
                            tmdb_id: &tmdb_id,
                            title: &"".to_string(),
                            imagepath: &"".to_string(),
                            description: &"".to_string(),
                        };

                        if !season_info.check_season() {
                            println!("insert season information for {} {}", tmdb_id, season);
                            sync_season(tmdb_id, season);
                        }

                        let mut epi_info = NewEpisode {
                            path: file_name,
                            serie_id: &tmdb_id,
                            season_id: &season,
                            episode_id: &episode,
                            tmdb_id: &tmdb_id,
                            title: &"".to_string(),
                            description: &"".to_string(),
                        };
                        if !epi_info.check_episode() {
                            println!("insert episode information for {} {}", tmdb_id, season);
                            let episode_info =
                                tmdb::tmdb::tv_episodes_get_details(tmdb_id, season, episode);

                            epi_info.title = &episode_info.name;
                            epi_info.description = &episode_info.overview;

                            let connection = mpv_webrpc::establish_connection();
                            let _insert_result = diesel::insert_into(episode::table)
                                .values(&epi_info)
                                .execute(&connection);
                            println!("insert episode info {:?}", episode_info);
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
}

pub fn check_tmdb_id(id_to_check: i32) -> bool {
    let ignored = NewIgnored {
        tmdb_id: &id_to_check.clone(),
    };

    let is_ignored = ignored.is_ignored();
    if is_ignored {
        return true;
    }

    return false;
}

fn parsing_season_and_episode(text: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(S\d{1,2}|s\d{1,2})(E\d{1,2}|e\d{1,2})").unwrap();
    }

    RE.captures(text)
}
