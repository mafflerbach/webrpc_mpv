use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
extern crate reqwest;

pub fn get_favourites(query: serde_json::Value) -> MediathekViewWeb {
    let client = reqwest::Client::new();
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers
    }
    let res = client
        .post("https://mediathekviewweb.de/api/query")
        .body(query.to_string())
        .headers(construct_headers())
        .send()
        .unwrap()
        .text();

    let mut results: MediathekViewWeb = serde_json::from_str(&res.unwrap()).unwrap();
    let mut obj_vec = Vec::new();
    for obj in results.result.results {
        let new_obj = Object {
            title: obj.title,
            description: obj.description,
            timestamp: obj.timestamp,
            duration: obj.duration,
            channel: obj.channel,
            url_video_hd: obj.url_video_hd,
            url_video: obj.url_video,
            url_video_low: obj.url_video_low,
            id: obj.id,
            human_duration: Some(human_duration(obj.duration)),
        };
        obj_vec.push(new_obj);
    }
    results.result.results = obj_vec;
    results
}

use humantime::format_duration;
use std::time::Duration;
fn human_duration(duration: u64) -> String {
    let val1 = Duration::new(duration, 0);
    format_duration(val1).to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediathekViewWeb {
    pub result: Results,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    pub title: String,
    pub description: String,
    pub timestamp: i32,
    pub duration: u64,
    pub channel: String,
    pub human_duration: Option<String>,
    pub url_video_hd: String,
    pub url_video_low: String,
    pub url_video: String,
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Results {
    pub results: Vec<Object>,
}
