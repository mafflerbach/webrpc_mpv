use crate::tmdb;
use rocket::response::content;
#[get("/scan")]
pub fn request_scan() -> content::Json<String> {
    scan_dir();
    let path_entries = get_first_level();

    let mut test = Vec::new();
    let mut tmdb_response;
    for entry in &path_entries {
        tmdb_response = tmdb::tmdb::search(entry.to_string());
        for result in tmdb_response.results {
            if !check_tmdb_id(result.id) {
                test.push(result);
            }
        }
    }

    let me = serde_json::to_string(&test).unwrap();

    content::Json(me.to_string())
}

use crate::settings;
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
#[derive(Serialize, Deserialize)]
pub struct LibraryRequest {
    pub tmdb_id: i32,
}

use rocket_contrib::json::Json;
#[post("/add", data = "<request_content>")]
pub fn request_add_serie(request_content: Json<LibraryRequest>) -> content::Json<String> {
    let test = json!({
        "data": "ok",
        "message": "",
        "request_id": 0
    });
    content::Json(test.to_string())
}

use diesel::prelude::*;
use mpv_webrpc::models::*;
fn check_tmdb_id(id_to_check: i32) -> bool {
    fn check_ignore(id_to_check: i32, connection: &SqliteConnection) -> bool {
        use mpv_webrpc::schema::ignored::dsl::*;
        let results = ignored
            .filter(tmdb_id.eq(id_to_check))
            .load::<Ignored>(connection)
            .expect("Error loading ingnored Table");

        if results.len() >= 1 {
            return true;
        }
        return false;
    }

    fn check_serie(id_to_check: i32, connection: &SqliteConnection) -> bool {
        use mpv_webrpc::schema::serie::dsl::*;
        let results = serie
            .filter(tmdb_id.eq(id_to_check))
            .load::<Serie>(connection)
            .expect("Error loading ingnored Table");
        if results.len() >= 1 {
            return true;
        }
        return false;
    }

    let connection = mpv_webrpc::establish_connection();
    let serie_exists = check_serie(id_to_check.clone(), &connection);
    let is_ignored = check_ignore(id_to_check.clone(), &connection);

    if serie_exists || is_ignored {
        return true;
    }

    return false;
}

#[post("/ignore", data = "<request_content>")]
pub fn request_ignore_serie(request_content: Json<LibraryRequest>) -> content::Json<String> {
    use mpv_webrpc::schema::ignored;
    let connection = mpv_webrpc::establish_connection();

    let ignore_serie = NewIgnored {
        tmdb_id: &request_content.tmdb_id,
    };

    let insert_result = diesel::insert_into(ignored::table)
        .values(&ignore_serie)
        .execute(&connection);
    let message;

    match insert_result {
        Ok(v) => message = format!("{:?}", v),
        Err(e) => message = format!("{:?}", e),
    }

    let test = json!({
        "data": "ok",
        "message": message,
        "request_id": 0
    });
    content::Json(test.to_string())
}

use glob::glob;
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

extern crate lazy_static;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
fn parsing_season_and_episode(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"S(\d{1,2})E(\d{1,2})").unwrap();
    }

    let _foo: HashSet<&str> = RE.find_iter(text).map(|mat| mat.as_str()).collect();

    //println!("{:?}", foo);

    RE.is_match(text)
}
