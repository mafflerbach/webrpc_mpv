
use super::rocket;
use rocket::local::Client;
use rocket::http::*;
use std::process::Command;

#[test]
fn request_volume() {
    init();
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/volume").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"request_id\":0,\"error\":\"success\"}\n".into()));
}


//#[test]
//fn request_resume() {
    //init();
    //let client = Client::new(rocket()).expect("valid rocket instance");
    //let mut response = client.get("/player/resume").dispatch();
    //assert_eq!(response.status(), Status::Ok);
    //assert_eq!(response.body_string(), Some("{\"request_id\":0,\"error\":\"success\"}\n".into()));
//}

//#[test]
//fn request_pause() {
    //init();
    //let client = Client::new(rocket()).expect("valid rocket instance");
    //let mut response = client.get("/player/pause").dispatch();
    //assert_eq!(response.status(), Status::Ok);
    //assert_eq!(response.body_string(), Some("{\"request_id\":0,\"error\":\"success\"}\n".into()));
//}

fn init() {
        let mut mpv = Command::new("mpv");
        mpv.arg("--idle=yes")
            .arg("--input-ipc-server=/tmp/mpvsocket")
            .arg("--fs=yes")
            .spawn()
            .expect("OK");
    }




