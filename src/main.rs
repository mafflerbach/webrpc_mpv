#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

extern crate execute;
use rocket::response::content;

mod mpv;


#[get("/resume")]
fn request_resume() -> content::Json<String> {
    return mpv::mpv::event_resume();
}

#[get("/pause")]
fn request_pause() -> content::Json<String>  {
    return mpv::mpv::event_pause();
}

#[get("/playlist?<target>")]
fn request_playlist(target: String) -> content::Json<String> {
    return mpv::mpv::event_play_from_list(target);
}


#[get("/load?<target>")]
fn request_load(target: String) -> &'static str {
    return mpv::mpv::event_play(target);
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![ request_load, request_pause, request_resume, request_playlist]).launch();
}
