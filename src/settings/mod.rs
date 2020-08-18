use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Childs {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Streams {
    pub name: String,
    pub url: String,
    pub image: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub childs: Vec<Childs>,
    pub db: String,
    pub debug: bool,
    pub scan_dir_movies: String,
    pub scan_dir_series: String,
    pub socket: String,
    pub stream_urls: Vec<Streams>,
    pub tmdb: HashMap<String, String>,
    pub tmdb_key: String,
}

pub fn config() -> Result<Settings, Box<dyn Error>> {
    let setting_file = env::var("SETTINGS");
    read_settings_file(setting_file.unwrap())
}

pub fn init() -> Settings {
    let setting_file = env::var("SETTINGS");
    let res = read_settings_file(setting_file.unwrap());

    return res.unwrap();
}

fn read_settings_file<P: AsRef<Path>>(path: P) -> Result<Settings, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Settings = serde_json::from_reader(reader)?;

    Ok(u)
}

