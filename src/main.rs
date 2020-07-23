#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)]
mod tests;


#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate execute;

mod mpv;
mod settings;
mod api_structs;


use std::collections::HashMap;
//use std::time::Duration;
//use reqwest::Client;
//use reqwest::ClientBuilder;
extern crate reqwest;
use std::vec::Vec;
use std::env;

use crate::api_structs::UrlForm;
use crate::api_structs::PlaylistControl;
use crate::api_structs::VolumeControl;
use crate::settings::SettingContext;
use rocket::request::Form;
use rocket::response::content;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::fs::OpenOptions;
use std::io::Write;
use url::form_urlencoded::{ parse};

///
///
/// Resume a video after a pause
///
/// # Example
/// ```sh
/// curl 'http://localhost:8000/resume'
/// ```
#[get("/resume")]
fn request_resume() -> content::Json<String> {
    let resume_response = mpv::mpv::event_resume().unwrap();
    println!("{}", resume_response);
    content::Json(resume_response)
}


#[post("/volume", data="<request_content>")]
fn request_change_volume(request_content: Json<VolumeControl>) -> content::Json<String> {

    let volume_response = mpv::mpv::event_volume_change(request_content).unwrap();
    println!("{}", volume_response);
    content::Json(volume_response)
}

#[get("/volume")]
fn request_volume() -> content::Json<String> {
    let volume_response = mpv::mpv::event_volume().unwrap();
    println!("{}", volume_response);
    content::Json(volume_response)
}

/// Pause a video after
///
/// # Example
/// ```sh
/// curl 'http://localhost:8000/pause'
/// ```
#[get("/pause")]
fn request_pause() -> content::Json<String> {
    let pause_response = mpv::mpv::event_pause().unwrap();
    println!("{}",pause_response );
    content::Json(pause_response)
}

/// Load a playlist on the host system
///
/// # Example
/// ```sh
/// curl 'http://localhost:8000/playlist?/local/path/to/playlist'
/// ```
/// We are able to load a playlist file.
/// * target is a absolute path on the host
///
#[get("/playlist")]
fn request_playlist() -> content::Json<String> {
    let playlist_response = mpv::mpv::event_play_from_list(String::from("/tmp/playlist")).unwrap();
    println!("{}", playlist_response);
    content::Json(playlist_response)
}

/// Stopt the actual video and starting a new video based on url/path
///
/// # Example
/// ```sh
/// curl 'http://localhost:8000/load?/local/path/to/playlist'
/// curl 'http://localhost:8000/load?target=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DP3UIpTlFtc4y'
/// ```
/// We are able to load different targes
/// * target is a absolute path on the host or a Encoded url
///
/// ## Further examples
/// ```sh
/// http://localhost:8000/play?target=/home/maren/Downloads/ytFiles/The Best Way To Practice Chords.webm
/// http://localhost:8000/play?target=%2Fhome%joe%2FDownloads%2FytFiles%2FThe%20Best%20Way%20To%20Practice%20Chords.webm
/// http://localhost:8000/play?target=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DP3UIpTlFtc4
/// ```
/// will FAIL:
/// ```
/// http://localhost:8000/load?target=https://www.youtube.com/watch?v=P3UIpTlFtc4
/// ```
#[get("/play?<target>")]
fn request_load(target: String) -> content::Json<String> {
    let load_response = mpv::mpv::event_load(target).unwrap();
    println!("{}", load_response);
    content::Json(load_response)
}

///
/// play a video from url from form
///
#[post("/", data = "<url>")]
fn request_play_from_url(url: Json<UrlForm>) -> content::Json<String> {
    let target = url.target.to_string();
    let client = url.client.clone();

    let decoded: String = parse(target.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    let mut play_response = json!({
        "data": "ok",
        "error":"NULL",
        "request_id": 0
    }).to_string();
    ;

    println!("CLIENT ID: {}", client);


    if client == 0.to_string() {
        println!("PLAY ON CLIENT");
        play_response = mpv::mpv::event_load(target.clone()).unwrap();
    } else {
        println!("PLAY ON REMOTE");
        let client_url =  get_client(client);
        let mut map = HashMap::new();
        map.insert("target".to_string(), target.clone());
        map.insert("client".to_string(), 0.to_string());

        let res = send_request(client_url, map);
        play_response = res.unwrap();
    }

    // decode url

    content::Json(play_response)
}

fn get_client(client : String) -> String {
    let settings = settings::config();
    let childs =  settings.unwrap().childs;
    for client_setting in childs {
        if client_setting.id == client {
            return  client_setting.url;
        }

    }
    return "".to_string();
}


fn send_request(target : String, map: HashMap<String, String>) -> Result<String, reqwest::Error>{
    //TODO change to post, add fields target for video url and id = 0 for local 

    let client = reqwest::Client::new();
    client.post(&target.clone().to_string())
        .json(&map)
        .send()?.text()
}


#[post("/add", data = "<request_content>")]
fn event_add_to_playlist(request_content: Json<PlaylistControl>)-> content::Json<String> {
    let client = request_content.client.clone();
    let mut message: String = "".to_string();


    if client == 0.to_string() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("/tmp/playlist")
            .unwrap();

        message = "Added to playlist".to_string();
        if let Err(e) = writeln!(file, "{}", request_content.value) {
            message = format!("Couldn't write to file: {}", e);
            eprintln!("Couldn't write to file: {}", e);
        }

    } else {
        println!("PLAY ON REMOTE");
        let client_url =  get_client(client);
        let mut map = HashMap::new();
        map.insert("value".to_string(), request_content.value.clone());
        map.insert("client".to_string(), 0.to_string());

        message = format!("FORWARDING");
        let res = send_request(client_url, map);
    }

    let test = json!({
        "data": "ok",
        "error": message,
        "request_id": 0
    });
    content::Json(test.to_string())

}

#[derive(Serialize, Deserialize)]
struct TemplateContext {
    settings : SettingContext
}

#[get("/")]
fn hello() -> Template {
    let links_context = settings::init();
    let template_context = TemplateContext {
        settings : links_context
    };

    Template::render("index", &template_context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/public", StaticFiles::from("templates/public"))
        .mount(
            "/",
            routes![
            hello,
            request_load,
            request_change_volume,
            request_pause,
            request_resume,
            event_add_to_playlist,
            request_volume,
            request_play_from_url,
            request_playlist
            ],
            )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let setting_file = &args[1];

    env::set_var("SETTINGS", setting_file);


    mpv::mpv::init();
    rocket().launch();
}


