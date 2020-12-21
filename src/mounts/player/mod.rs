//use crate::api_structs::PlaylistControl;
use crate::settings;
use std::collections::HashMap;
//use std::fs::OpenOptions;
//use std::io::prelude::*;
//use std::process::Command;
//use url::form_urlencoded::parse;
use serde::{Serialize, Deserialize};
use actix_web::web::Query;  
use actix_web::{ HttpResponse, web};
//
// #[get("/pause")]
//pub fn request_pause() -> content::Json<String> {
//let pause_response = mpv::mpv::event_pause().unwrap();
//println!("{}", pause_response);
//content::Json(pause_response)
//}

//// #[get("/stop")]
//pub fn request_stop() -> content::Json<String> {
//let pause_response = mpv::mpv::event_stop().unwrap();
//println!("{}", pause_response);
//content::Json(pause_response)
//}

//// #[get("/resume")]
//pub fn request_resume() -> content::Json<String> {
//let resume_response = mpv::mpv::event_resume().unwrap();
//println!("{}", resume_response);
//content::Json(resume_response)
//}

//use crate::api_structs::Property;
// #[post("/propery", data = "<request_content>")]

//pub fn request_set_property(request_content: Json<Property>) -> content::Json<String> {
//let load_response = mpv::mpv::event_set_property(
//request_content.propery.clone(),
//request_content.value.clone(),
//)
//.unwrap();
//println!("{}", load_response);
//content::Json(load_response)
//}

// #[get("/propery?<target>")]
//pub fn request_get_property(target: String) -> content::Json<String> {
//let load_response = mpv::mpv::event_get_property(target).unwrap();
//println!("{}", load_response);
//content::Json(load_response)
//}

//// #[get("/shutdown")]
//pub fn request_shutdown() {
//let mut mpv = Command::new("shutdown");
//mpv.arg("-h")
//.arg("now")
//.spawn()
//.expect("OK");
//}

#[derive(Deserialize)]
pub struct Info {
    pub target: String,
}

pub async fn request_start_video(info: Query<Info>) -> HttpResponse {
    let target = info.target.to_string();
    let load_response = mpv::mpv::event_load(target);
    HttpResponse::Ok().json(load_response) // <- send response
}

//use crate::api_structs::Status;
//use crate::library;
// #[post("/status", data = "<path>")]
//pub async fn request_video_status(body: web::Bytes) -> HttpResponse {
//let result : Status = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
//println!("{:?}",result);
//let video_status = library::get_video_status(result);

//let response = format!("{{\"stauts\": \"{}\"}}",video_status);
//HttpResponse::Ok().json(response) // <- send response
//}


#[derive( Debug, Serialize, Deserialize)]
pub struct PropertyComand {
    pub propery : String,
    pub value : Option<String> 
}

pub async fn request_property(body: web::Bytes) -> HttpResponse {
    let result : PropertyComand = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    let command = &result.propery;
    let mpv_response : mpv::mpv::Property;
    match command.as_ref() {
        "time-pos" => {
            match result.value {
                None => mpv_response = mpv::mpv::event_property("time-pos".to_string(), None),
                Some(value) => {
                    mpv_response = mpv::mpv::event_property("time-pos".to_string(), Some(value) )
                },
            };
        },
        "duration" => {
            mpv_response = mpv::mpv::event_property("duration".to_string(), None)
        }


        _ => {
            let tjson = json!({ "error": "property not allowed" });
            return HttpResponse::MethodNotAllowed().json(tjson.to_string())
        },
    }

    let err_property :String = mpv_response.error.to_string();
    if err_property != "success".to_string() {
        let tjson = json!({ "error": "2 Something went wrong" });
        return HttpResponse::InternalServerError().json(tjson.to_string())
    }

    HttpResponse::Ok().json(serde_json::to_string(&mpv_response).unwrap()) // <- send response
}



#[derive( Debug, Serialize, Deserialize)]
pub struct PlayerComand {
    pub command : String,
    pub value : Option<String> 
}
use crate::mpv;
pub async fn request_player(body: web::Bytes) -> HttpResponse {
    let result : PlayerComand = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    println!("{:?}",result);
    let error = false;
    let command = &result.command;
    let mpv_response :mpv::mpv::Property;
    match command.as_ref() {
        "pause" => mpv_response = mpv::mpv::event_pause(),
        "stop" => mpv_response = mpv::mpv::event_stop(),
        "play" => {
            let target = match result.value {
                Some(v) => v ,
                None => {
                    let tjson = json!({ "error": "target undefined" });
                    return HttpResponse::BadRequest().json(tjson.to_string())
                },
            };
            mpv_response = mpv::mpv::event_load(target)
        },
        "resume" => mpv_response = mpv::mpv::event_resume(),
        _ => {
            let tjson = json!({ "error": "method not allowed" });
            return HttpResponse::MethodNotAllowed().json(tjson.to_string())
        },
    }
    if mpv_response.error.replace("\"", "") != "success" {
        return HttpResponse::InternalServerError().json(serde_json::to_string(&mpv_response).unwrap())
    }

    HttpResponse::Ok().json(serde_json::to_string(&mpv_response).unwrap()) // <- send response
}
//// #[derive(Serialize, Deserialize)]
//pub struct UrlForm {
//pub target: String,
//pub client: String,
//}
//// #[post("/", data = "<url>")]
//pub fn request_play_from_url(url: Json<UrlForm>) -> content::Json<String> {
//let target = url.target.to_string();
//let client = url.client.clone();

//let decoded: String = parse(target.as_bytes())
//.map(|(key, val)| [key, val].concat())
//.collect();

//let play_response;

//println!("CLIENT ID: {}", client);

//if client == "null".to_string() {
//println!("PLAY ON CLIENT");

//println!("VIDEO URL: {}", decoded);
//play_response = mpv::mpv::event_load(target).unwrap();
//} else {
//println!("PLAY ON REMOTE");
//let client_url = get_client(client);
//let mut map = HashMap::new();
//map.insert("target".to_string(), target.clone());
//map.insert("client".to_string(), 0.to_string());

//let res = send_request(client_url, map);
//play_response = res.unwrap();
//}

//content::Json(play_response)
//}

//// #[post("/add", data = "<request_content>")]
//pub fn event_add_to_playlist(request_content: Json<PlaylistControl>) -> content::Json<String> {
//let client = request_content.client.clone();
//let mut message;

//if client == "null".to_string() {
//let mut file = OpenOptions::new()
//.write(true)
//.create(true)
//.append(true)
//.open("/tmp/playlist")
//.unwrap();

//message = "Added to playlist".to_string();
//if let Err(e) = writeln!(file, "{}", request_content.value) {
//message = format!("Couldn't write to file: {}", e);
//eprintln!("Couldn't write to file: {}", e);
//}
//} else {
//println!("PLAY ON REMOTE");
//let client_url = get_client(client);
//let mut map = HashMap::new();
//map.insert("value".to_string(), request_content.value.clone());
//map.insert("client".to_string(), 0.to_string());
//message = format!("FORWARDING");
//let _res = send_request(format!("{}/add", client_url), map);
//}

//let test = json!({
//"data": "ok",
//"message": message,
//"request_id": 0
//});
//content::Json(test.to_string())
//}

//// #[get("/playlist")]
//pub fn request_playlist() -> content::Json<String> {
//let playlist_response = mpv::mpv::event_play_from_list(String::from("/tmp/playlist")).unwrap();
//println!("{}", playlist_response);
//content::Json(playlist_response)
//}

fn send_request(target: String, map: HashMap<String, String>) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post(&target.clone().to_string())
        .json(&map)
        .send()?
        .text()
}

fn get_client(client: String) -> String {
    let settings = settings::config();
    let childs = settings.unwrap().childs;
    for client_setting in childs {
        if client_setting.id == client {
            return client_setting.url;
        }
    }
    return "".to_string();
}
