use actix_web::{
    HttpResponse, web
};
use crate::mpv;
use crate::api_structs::VolumeControl;

// get
pub async fn request_volume() -> HttpResponse {
    let volume_response = mpv::mpv::event_volume();
    HttpResponse::Ok().json(volume_response) // <- send response
}

// post 
pub async fn request_change_volume(body: web::Bytes) -> HttpResponse {
    let result : VolumeControl = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    let volume_response = mpv::mpv::event_volume_change(result);
    HttpResponse::Ok().json(volume_response) // <- send response
}
