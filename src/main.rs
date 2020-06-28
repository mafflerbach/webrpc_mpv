#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

#[cfg(test)]
mod tests;

//use rocket::response::content;
extern crate execute;

mod mpv;
mod settings;
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::content;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use url::form_urlencoded::{byte_serialize, parse};
/// Resume a video after a pause
///
/// # Example
/// ```sh
/// curl 'http://localhost:8000/resume'
/// ```
#[get("/resume")]
fn request_resume() -> content::Json<String> {
    let resume_response = mpv::mpv::event_resume();
    content::Json(resume_response.unwrap())
}

#[derive(Serialize, Deserialize)]
struct Response {
    data: String,
    error: String,
    request_id: i32,
}

#[derive(Serialize, Deserialize)]
struct VolumResponse {
    data: String,
    error: String,
    request_id: i32,
}

#[get("/volume")]
fn request_volume() -> content::Json<String> {
    let fooo = mpv::mpv::event_volume();
    content::Json(fooo.unwrap())
}

/// Pause a video after
///
/// # Example
/// ```sh
/// curl 'http://localhost:8000/pause'
/// ```
#[get("/pause")]
fn request_pause() -> content::Json<String> {
    let pause_response = mpv::mpv::event_pause();
    content::Json(pause_response.unwrap())
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
    let playlist_response = mpv::mpv::event_play_from_list(String::from("/tmp/playlist"));
    content::Json(playlist_response.unwrap())
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
    let load_response = mpv::mpv::event_load(target);
    content::Json(load_response.unwrap())
}

///
/// play a video from url from form
///
#[post("/", data = "<url>")]
fn request_play_from_url(url: Form<UrlForm<'_>>) -> content::Json<String> {
    let target = url.target.to_string();
    // decode url
    let decoded: String = parse(target.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();

    let play_response = mpv::mpv::event_load(decoded);
    content::Json(play_response.unwrap())
}


#[post("/add", data = "<request_content>")]
fn event_add_to_playlist(request_content: Form<UrlForm<'_>>)-> content::Json<String> {

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/playlist")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", request_content.target) {
        eprintln!("Couldn't write to file: {}", e);
    }

    content::Json(String::from("{
        'data': 'ok',
        'error': 'success',
        'request_id': 0
    }"))

}

#[get("/")]
fn hello() -> Template {
    let mut streaming_links = HashMap::new();

    let ard: String =
        byte_serialize("https://mcdn.daserste.de/daserste/de/master.m3u8?arn=1".as_bytes())
            .collect();
    let zdf: String = byte_serialize(
        "https://zdf-hls-01.akamaized.net/hls/live/2002460/de/high/master.m3u8".as_bytes(),
    )
    .collect();
    let arte: String = byte_serialize(
        "https://artelive-lh.akamaihd.net/i/artelive_de@393591/index_1_av-p.m3u8".as_bytes(),
    )
    .collect();
    let kika: String = byte_serialize(
        "https://kikageohls-i.akamaihd.net/hls/live/1006268/livetvkika_de/master.m3u8".as_bytes(),
    )
    .collect();
    let drei_sat: String = byte_serialize(
        "https://zdfhls18-i.akamaihd.net/hls/live/744751/dach/high/master.m3u8".as_bytes(),
    )
    .collect();
    let phoenix: String = byte_serialize(
        "https://zdfhls19-i.akamaihd.net/hls/live/744752/de/high/master.m3u8".as_bytes(),
    )
    .collect();

    streaming_links.insert("ARD".to_string(), ard.to_string());
    streaming_links.insert("ZDF".to_string(), zdf.to_string());
    streaming_links.insert("Arte".to_string(), arte.to_string());
    streaming_links.insert("kika".to_string(), kika.to_string());
    streaming_links.insert("3Sat".to_string(), drei_sat.to_string());
    streaming_links.insert("phoenix".to_string(), phoenix.to_string());

    let context = TemplateContext {
        items: streaming_links,
    };

    Template::render("index", &context)
}

fn main() {
    mpv::mpv::init();
    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                hello,
                request_load,
                request_pause,
                request_resume,
                request_volume,
                request_play_from_url,
                request_playlist
            ],
        )
        .launch();
}

#[derive(FromForm)]
struct UrlForm<'r> {
    target: &'r RawStr,
}


#[derive(Serialize)]
struct TemplateContext {
    items: HashMap<String, String>,
}
