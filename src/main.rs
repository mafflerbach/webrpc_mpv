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

extern crate lazy_static;

use lazy_static::lazy_static;
use regex::Regex;
mod api_structs;
mod mpv;
mod settings;
mod stubs;
mod tmdb;

use glob::glob;
use std::collections::HashMap;
use std::collections::HashSet;
extern crate reqwest;
use crate::settings::SettingContext;
use rocket::response::content;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
struct TemplateContext {
    settings: SettingContext,
}

#[get("/scan")]
fn request_scan() -> content::Json<String> {
    scan_dir();
    let path_entries = get_first_level();

    let mut test = Vec::new();
    let mut tmdb_response;
    for entry in &path_entries {
        tmdb_response = tmdb::tmdb::search(entry.to_string());
        for result in tmdb_response.results {
            test.push(result);
        }
    }

    println!("{:?}", test);

    let me = serde_json::to_string(&test).unwrap();

    content::Json(me.to_string())
}

#[get("/")]
fn index() -> Template {
    let links_context = settings::init();
    let template_context = TemplateContext {
        settings: links_context,
    };
    Template::render("index", &template_context)
}

use std::{fs, io};
fn get_first_level() -> Vec<String> {
    let settings = settings::init();
    println!("{}", settings.scan_dir);
    let base_path = settings.scan_dir.clone();
    let mut entries = fs::read_dir(base_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    entries.sort();
    let path_part = settings.scan_dir.clone();
    let mut stack = Vec::new();
    for entry in &entries {
        let foo = entry.display().to_string().replace(&path_part, "");
        stack.push(foo)
    }

    return stack;
}

fn scan_dir() {
    let settings = settings::init();
    let pattern = format!("{}/**/*.mkv", settings.scan_dir);

    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                parsing_season_and_episode(&path.clone().into_os_string().into_string().unwrap());
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
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
                mounts::player::request_start_video,
            ],
        )
        .mount(
            "/",
            routes![
                index,
                mounts::volume::request_change_volume,
                mounts::volume::request_volume,
                request_scan
            ],
        )
}

fn parsing_season_and_episode(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"S(\d{1,2})E(\d{1,2})").unwrap();
    }

    let _foo: HashSet<&str> = RE.find_iter(text).map(|mat| mat.as_str()).collect();

    //println!("{:?}", foo);

    RE.is_match(text)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let setting_file = &args[1];

    env::set_var("SETTINGS", setting_file);

    mpv::mpv::init();
    rocket().launch();
}
