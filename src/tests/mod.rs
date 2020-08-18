use super::rocket;
use crate::stubs;
use rocket::http::*;
use rocket::local::Client;
use std::env;

#[test]
fn request_volume() {
    env::set_var("SETTINGS", "settings/settings_testing.json");
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/volume").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("{\"data\":100.000000,\"request_id\":0,\"error\":\"success\"}\n".into())
    );
}


#[test]
fn request_resume() {
    env::set_var("SETTINGS", "settings/settings_testing.json");
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/player/resume").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("{\"request_id\":0,\"error\":\"success\"}\n".into())
    );
}

#[test]
fn request_pause() {
    env::set_var("SETTINGS", "settings/settings_testing.json");
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/player/resume").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("{\"request_id\":0,\"error\":\"success\"}\n".into())
    );
}

#[test]
fn request_ignore() {
    env::set_var("SETTINGS", "settings/settings_testing.json");
    let client = Client::new(rocket()).expect("valid rocket instance");
    let body = json!({"tmdb_id":69088, "path" : ""}).to_string();
    let mut response = client.post("/library/ignore").body(body).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("{\"data\":\"ok\",\"message\":\"1\",\"request_id\":0}".into())
    );
}

#[test]
fn request_scan() {
    env::set_var("SETTINGS", "settings/settings_testing.json");
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/library/scan").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let scan_result: String =
        stubs::read_html_fixture_file("/home/maren/development/rust/mpv/test/scan_result.html");
    let right = strip_content(scan_result);
    let left = strip_content(response.body_string().unwrap());

    assert_eq!(right, left);
}

fn strip_content(mut content: String) -> String {
    content = content.replace("\n", "");
    content = content.replace(" ", "");
    content
}

use std::panic;
