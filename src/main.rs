#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)] mod tests;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate execute;

mod api_structs;
mod mpv;
mod settings;
mod stubs;
mod tmdb;

extern crate reqwest;
use crate::settings::Settings;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
struct TemplateContext {
    settings: Settings,
}

#[get("/")]
fn index() -> Template {
    let links_context = settings::init();
    let template_context = TemplateContext {
        settings: links_context,
    };
    Template::render("index", &template_context)
}

mod library;
mod mounts;
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/public", StaticFiles::from("templates/public"))
        .mount(
            "/player",
            routes![
                mounts::player::event_add_to_playlist,
                mounts::player::request_stop,
                mounts::player::request_pause,
                mounts::player::request_play_from_url,
                mounts::player::request_playlist,
                mounts::player::request_resume,
                mounts::player::request_get_property,
                mounts::player::request_set_property,
                mounts::player::request_start_video,
            ],
        )
        .mount(
            "/library",
            routes![
                mounts::library::request_scan,
                mounts::library::request_add_movie,
                mounts::library::request_add_serie,
                mounts::library::request_ignore_serie
            ],
        )
        .mount(
            "/series",
            routes![mounts::series::index, mounts::series::detail,],
        )
        .mount(
            "/movies",
            routes![
                mounts::movies::detail,
                mounts::movies::index,
                mounts::movies::request_search_movie_post,
            ],
        )
        .mount(
            "/favourites",
            routes![mounts::favourites::index],
        )
        .mount(
            "/episodes",
            routes![mounts::library::episodes::detail, mounts::library::episodes::index],
        )
        .mount(
            "/",
            routes![
                index,
                mounts::volume::request_change_volume,
                mounts::volume::request_volume,
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
