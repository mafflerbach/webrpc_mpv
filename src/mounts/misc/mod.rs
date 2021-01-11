use actix_web::{ HttpResponse};
use std::process::Command;

pub async fn heartbeat() -> HttpResponse {
    let tjson = json!({ "alive": "true" });
    return HttpResponse::Ok().json(tjson)
}


pub async fn shutdown() -> HttpResponse {
    let mut mpv = Command::new("shutdown");
    mpv.arg("-h")
        .arg("now")
        .spawn()
        .expect("OK");
    let tjson = json!({ "shutdown": "triggered" });
    return HttpResponse::Ok().json(tjson)
}
