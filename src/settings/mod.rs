#![allow(non_snake_case)]

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Streams {
    pub name: String,
    pub url: String,
    pub image: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Favourite {
    pub name: String,
    pub query: Queries,
    pub image: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Queries {
    pub sortBy: String,
    pub duration_max:Option<i32>,
    pub duration_min:Option<i32>,
    pub sortOrder: String,
    pub queries: Vec<Fields>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Fields {
    pub fields: Vec<String>,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub db: String,
    pub debug: bool,
    pub scan_dir_movies: String,
    pub scan_dir_series: String,
    pub socket: String,
    pub stream_urls: Vec<Streams>,
    pub favourites: Vec<Favourite>,
    pub tmdb: HashMap<String, String>,
    pub tmdb_key: String,
}

fn get_settings_filename() -> String {
    let filename = env::var("SETTINGS");

    match filename {
        Ok(filename) => { filename }
        Err(_) => {
            env::var("HOME").unwrap() + "/.config/mediamate/settings.json"
        }
    }
}

pub fn config() -> Result<Settings, Box<dyn Error>> {
    let setting_file = get_settings_filename();
    read_settings_file(setting_file)
}

pub fn init() -> Settings {
    return config().unwrap();
}

fn read_settings_file<P: AsRef<Path>>(path: P) -> Result<Settings, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Settings = serde_json::from_reader(reader)?;

    Ok(u)
}
