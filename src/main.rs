#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

extern crate execute;
use rocket::response::content;

mod mpv;

/// Resume a video after a pause 
///
/// # Example 
/// ```sh
/// curl 'http://localhost:8000/resume' 
/// ```
#[get("/resume")]
fn request_resume() -> content::Json<String> {
    return mpv::mpv::event_resume();
}

/// Pause a video after 
///
/// # Example 
/// ```sh
/// curl 'http://localhost:8000/pause' 
/// ```
#[get("/pause")]
fn request_pause() -> content::Json<String>  {
    return mpv::mpv::event_pause();
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
#[get("/playlist?<target>")]
fn request_playlist(target: String) -> content::Json<String> {
    return mpv::mpv::event_play_from_list(target);
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
/// http://localhost:8000/load?target=/home/joe/Downloads/ytFiles/The Best Way To Practice Chords.webm
/// http://localhost:8000/load?target=%2Fhome%joe%2FDownloads%2FytFiles%2FThe%20Best%20Way%20To%20Practice%20Chords.webm
/// http://localhost:8000/load?target=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DP3UIpTlFtc4
/// ```
/// will FAIL: 
/// ```
/// http://localhost:8000/load?target=https://www.youtube.com/watch?v=P3UIpTlFtc4
/// ```

#[get("/load?<target>")]
fn request_load(target: String) -> content::Json<String>  {
    return mpv::mpv::event_load(target);
}

/// Start a new insance of mpv and plays the source target
///
/// # Example 
/// ```sh 
/// curl 'http://localhost:8000/play?/local/path/to/playlist' 
/// curl 'http://localhost:8000/play?target=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DP3UIpTlFtc4y'
/// ```
/// We are able to load different targes
/// * target is a absolute path on the host or a Encoded url 
///
/// ## Further examples
/// ```sh
/// http://localhost:8000/load?target=/home/joe/Downloads/ytFiles/The Best Way To Practice Chords.webm
/// http://localhost:8000/load?target=%2Fhome%joe%2FDownloads%2FytFiles%2FThe%20Best%20Way%20To%20Practice%20Chords.webm
/// http://localhost:8000/load?target=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DP3UIpTlFtc4
/// ```
/// will FAIL: 
/// ```sh
/// http://localhost:8000/load?target=https://www.youtube.com/watch?v=P3UIpTlFtc4
/// ```
#[get("/play?<target>")]
fn request_play(target: String) -> content::Json<String>  {
    return mpv::mpv::event_load(target);
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![ 
                           request_load, 
                           request_play,
                           request_pause,
                           request_resume,
                           request_playlist
    ]).launch();
}
