use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use url::form_urlencoded::byte_serialize;
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub db: String,
    pub tmdb_key: String,
    pub debug: bool,
    pub scan_dir_movies: String,
    pub scan_dir_series: String,
    pub socket: String,
    pub childs: Vec<Childs>,
    pub stream_urls: Vec<Streams>,
}
#[derive(Serialize, Deserialize)]
pub struct SettingContext {
    pub db: String,
    pub tmdb_key: String,
    pub scan_dir_movies: String,
    pub scan_dir_series: String,
    pub socket: String,
    pub clients: HashMap<String, String>,
    pub streams: HashMap<String, String>,
}

pub fn config() -> Result<Settings, Box<dyn Error>> {
    let setting_file = env::var("SETTINGS");
    println!("SETTINGS config: {:?}", setting_file);
    read_settings_file(setting_file.unwrap())
}

pub fn init() -> SettingContext {
    let setting_file = env::var("SETTINGS");
    let res = read_settings_file(setting_file.unwrap());
    println!("SETTINGS init: {:?}", res);

    let mut streaming_links = HashMap::new();
    let mut clients = HashMap::new();
    let mut db = String::new();
    let mut tmdb_key = String::new();
    let mut scan_dir_movies = String::new();
    let mut scan_dir_series = String::new();
    let mut socket = String::new();

    if res.is_ok() {
        let s: Settings = res.unwrap();
        socket = s.socket;
        scan_dir_series = s.scan_dir_series;
        scan_dir_movies = s.scan_dir_movies;
        tmdb_key = s.tmdb_key;
        db = s.db;
        for i in &s.childs {
            clients.insert(i.id.to_string(), i.name.to_string());
        }

        for i in &s.stream_urls {
            let urlencoded: String = byte_serialize(i.url.as_bytes()).collect();
            streaming_links.insert(i.name.to_string(), urlencoded);
        }
    } else {
        println!("{:?}", res);
    }

    let links_context = SettingContext {
        db: db,
        scan_dir_movies: scan_dir_movies,
        scan_dir_series: scan_dir_series,
        tmdb_key: tmdb_key,
        socket: socket,
        clients: clients,
        streams: streaming_links,
    };

    return links_context;
}

fn read_settings_file<P: AsRef<Path>>(path: P) -> Result<Settings, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    println!("SETTINGS read_settings: {:?}", reader);

    let u: Settings = serde_json::from_reader(reader)?;

    Ok(u)
}
