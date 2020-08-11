
    use std::process::Command;
use super::rocket;
use crate::mpv;
use rocket::http::*;
use rocket::local::Client;
use std::env;
use std::{thread, time};

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

use std::panic;


