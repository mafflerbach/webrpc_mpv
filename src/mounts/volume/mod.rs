use crate::api_structs::VolumeControl;
use crate::mpv;
use rocket::response::content;
use rocket_contrib::json::Json;

#[post("/volume", data = "<request_content>")]
pub fn request_change_volume(request_content: Json<VolumeControl>) -> content::Json<String> {
    let volume_response = mpv::mpv::event_volume_change(request_content).unwrap();
    println!("{}", volume_response);
    content::Json(volume_response)
}

#[get("/volume")]
pub fn request_volume() -> content::Json<String> {
    let volume_response = mpv::mpv::event_volume().unwrap();
    println!("{}", volume_response);
    content::Json(volume_response)
}



