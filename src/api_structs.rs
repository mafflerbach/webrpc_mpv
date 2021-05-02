use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub time: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub path: String,
}

#[derive(Serialize)]
pub struct TemplateContext {
    pub items: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct PlaylistControl {
    pub client: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Property {
    pub property: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeControl {
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    data: String,
    error: String,
    request_id: i32,
}

#[derive( Debug, Serialize, Deserialize)]
pub struct PropertyComand {
    pub property : String,
    pub value : Option<String> 
}
