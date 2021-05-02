use mpv::mpv::Property;
use serde::{Serialize, Deserialize};
use actix_web::{ HttpResponse, web};
use crate::mpv;
use crate::library;
use crate::api_structs::PropertyComand;

pub async fn request_property(body: web::Bytes) -> HttpResponse {
    let result : PropertyComand = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    let mpv_response = property_handle(result);
    let err_property :String = mpv_response.error.to_string();
    if err_property != "success".to_string() {
        let tjson = json!({ "error": "Something went wrong" });
        return HttpResponse::InternalServerError().json(tjson)
    }

    HttpResponse::Ok().json(mpv_response) // <- send response
}


pub fn property_handle(property_command : PropertyComand) -> Property{

    let command = &property_command.property;
    let mpv_response : mpv::mpv::Property;
    println!("XXXXX {:?}", property_command);
    match command.as_ref() {
        "time-pos" => {
            match property_command.value {
                None => mpv_response = mpv::mpv::event_property("time-pos".to_string(), None),
                Some(value) => {
                    mpv_response = mpv::mpv::event_property("time-pos".to_string(), Some(value))
                },
            };
        },
        "duration" => {
            mpv_response = mpv::mpv::event_property("duration".to_string(), None)
        }
        _ => {
            mpv_response = Property { 
                error : "error".to_string(),
                data : "property not allowed".to_string()
            }
        },
    }
    return mpv_response;
}



#[derive( Debug, Serialize, Deserialize)]
pub struct PlayerComand {
    pub command : String,
    pub value : Option<String> 
}
pub async fn request_player(body: web::Bytes) -> HttpResponse {
    let result : PlayerComand = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    let command = &result.command;
    let mpv_response :mpv::mpv::Property;
    match command.as_ref() {
        "pause" => mpv_response = mpv::mpv::event_pause(),
        "status" => {
            let target = match result.value {
                Some(v) => v ,
                None => {
                    let tjson = json!({ "error": "target undefined" });
                    return HttpResponse::BadRequest().json(tjson)
                },
            };

            mpv_response = library::get_video_status(target);
        },
        "stop" => mpv_response = mpv::mpv::event_stop(),
        "play" => {
            let target = match result.value {
                Some(v) => v ,
                None => {
                    let tjson = json!({ "error": "target undefined" });
                    return HttpResponse::BadRequest().json(tjson)
                },
            };
            mpv_response = mpv::mpv::event_load(&target, "replace");
            // Append backdrop to playlist
            mpv::mpv::event_load("osd/black.png", "append");

        },
        "resume" => mpv_response = mpv::mpv::event_resume(),
        _ => {
            let tjson = json!({ "error": "method not allowed" });
            return HttpResponse::MethodNotAllowed().json(tjson)
        },
    }
    if mpv_response.error.replace("\"", "") != "success" {
        return HttpResponse::InternalServerError().json(&mpv_response)
    }

    HttpResponse::Ok().json(mpv_response) // <- send response
}

#[derive(Deserialize)]
pub struct Info {
    pub target: String,
}


