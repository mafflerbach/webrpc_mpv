use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use url::form_urlencoded::{ byte_serialize};
use std::io::BufReader;
use std::path::Path;
use std::env;
#[derive(Serialize, Deserialize, Debug)]
pub struct Childs {
    pub    id: i16 ,
    pub  name: String,
    pub   url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Streams {
    pub name: String,
    pub  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub  debug: bool,
    pub  socket : String,
    pub  childs :Vec<Childs>,
    pub  stream_urls: Vec<Streams>

}
#[derive(Serialize, Deserialize)]
pub struct SettingContext {
    pub  socket : String,
    pub  clients : HashMap<String, String>,
    pub  streams: HashMap<String, String>
}


pub fn init() -> SettingContext {

    let setting_file = env::var("SETTINGS");
    let res = read_settings_file(setting_file.unwrap());

    let mut streaming_links = HashMap::new();
    let mut clients = HashMap::new();
    let mut socket = String::new();
    if res.is_ok() {
        let s: Settings = res.unwrap();
        socket = s.socket;
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
        socket : socket,
        clients : clients,
        streams: streaming_links
    };

    return links_context
}

fn read_settings_file<P: AsRef<Path>>(path: P) -> Result<Settings, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}


