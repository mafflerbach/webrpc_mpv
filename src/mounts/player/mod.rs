use crate::mpv;
use rocket::response::content;

#[get("/pause")]
pub fn request_pause() -> content::Json<String> {
    let pause_response = mpv::mpv::event_pause().unwrap();
    println!("{}", pause_response);
    content::Json(pause_response)
}

#[get("/stop")]
pub fn request_stop() -> content::Json<String> {
    let pause_response = mpv::mpv::event_stop().unwrap();
    println!("{}", pause_response);
    content::Json(pause_response)
}

#[get("/resume")]
pub fn request_resume() -> content::Json<String> {
    let resume_response = mpv::mpv::event_resume().unwrap();
    println!("{}", resume_response);
    content::Json(resume_response)
}

#[get("/propery?<target>")]
pub fn request_get_property(target: String) -> content::Json<String> {
    let load_response = mpv::mpv::event_get_property(target).unwrap();
    println!("{}", load_response);
    content::Json(load_response)
}

#[get("/play?<target>")]
pub fn request_start_video(target: String) -> content::Json<String> {
    let load_response = mpv::mpv::event_load(target).unwrap();
    println!("{}", load_response);
    content::Json(load_response)
}

use crate::api_structs::UrlForm;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use url::form_urlencoded::parse;
#[post("/", data = "<url>")]
pub fn request_play_from_url(url: Json<UrlForm>) -> content::Json<String> {
    let target = url.target.to_string();
    let client = url.client.clone();

    let decoded: String = parse(target.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();


    let play_response;

    println!("CLIENT ID: {}", client);

    if client == "null".to_string() {
        println!("PLAY ON CLIENT");

        println!("VIDEO URL: {}", decoded);
        play_response = mpv::mpv::event_load(target).unwrap();
    } else {
        println!("PLAY ON REMOTE");
        let client_url = get_client(client);
        let mut map = HashMap::new();
        map.insert("target".to_string(), target.clone());
        map.insert("client".to_string(), 0.to_string());

        let res = send_request(client_url, map);
        play_response = res.unwrap();
    }

    content::Json(play_response)
}

fn send_request(target: String, map: HashMap<String, String>) -> Result<String, reqwest::Error> {
    //TODO change to post, add fields target for video url and id = 0 for local

    let client = reqwest::Client::new();
    client
        .post(&target.clone().to_string())
        .json(&map)
        .send()?
        .text()
}

use crate::settings;
fn get_client(client: String) -> String {
    let settings = settings::config();
    let childs = settings.unwrap().childs;
    for client_setting in childs {
        if client_setting.id == client {
            return client_setting.url;
        }
    }
    return "".to_string();
}

use crate::api_structs::PlaylistControl;
use std::fs::OpenOptions;
use std::io::prelude::*;
#[post("/add", data = "<request_content>")]
pub fn event_add_to_playlist(request_content: Json<PlaylistControl>) -> content::Json<String> {
    let client = request_content.client.clone();
    let mut message;

    if client == "null".to_string() {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
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
        let client_url = get_client(client);
        let mut map = HashMap::new();
        map.insert("value".to_string(), request_content.value.clone());
        map.insert("client".to_string(), 0.to_string());
        message = format!("FORWARDING");
        let _res = send_request(format!("{}/add", client_url), map);
    }

    let test = json!({
        "data": "ok",
        "message": message,
        "request_id": 0
    });
    content::Json(test.to_string())
}

#[get("/playlist")]
pub fn request_playlist() -> content::Json<String> {
    let playlist_response = mpv::mpv::event_play_from_list(String::from("/tmp/playlist")).unwrap();
    println!("{}", playlist_response);
    content::Json(playlist_response)
}
