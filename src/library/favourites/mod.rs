use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
extern crate reqwest;
use crate::settings;
use serde::{Serialize, Deserialize};

pub fn get_favourites(query: serde_json::Value) -> MediathekViewWeb {
    let settings2 = &settings::config();
    let mediathekviewweb_query_url: String =
        settings2.as_ref().unwrap().tmdb["mediathekviewweb_query_url"].to_string();
    let res = send_request(mediathekviewweb_query_url, query);
    let mut results: MediathekViewWeb = serde_json::from_str(&res).unwrap();
    let mut obj_vec = Vec::new();
    for obj in results.result.results {
        let new_obj = Object {
            title: obj.title,
            topic: obj.topic,
            description: obj.description,
            timestamp: obj.timestamp,
            duration: obj.duration,
            channel: obj.channel,
            url_video_hd: obj.url_video_hd,
            url_video: obj.url_video,
            url_video_low: obj.url_video_low,
            id: obj.id,
            human_duration: Some(human_duration(obj.duration)),
            date: Some(format_date(obj.timestamp))
        };
        obj_vec.push(new_obj);
    }
    results.result.results = obj_vec;
    results
}

use crate::stubs;
fn send_request(target: String, query: serde_json::Value) -> String {
    let settings2 = &settings::config();
    let debug = settings2.as_ref().unwrap().debug.to_string();
    if debug == "true" {
        let response = stubs::read_fixture_file(&target.clone().to_string());
        return response;
    }
    let client = reqwest::Client::new();
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers
    }
    match client
        .post(&target)
        .body(query.to_string())
        .headers(construct_headers())
        .send()
        .unwrap()
        .text()
    {
        Ok(r) => {return r},
        Err(_) => return "".to_string(),
    }
}

fn human_duration(duration: u64) -> String {
    let seconds = duration % 60;
    let minutes = duration / 60 + (if seconds > 0 { 1 } else { 0 });
    return format!("{} Minuten", minutes)
}

use chrono::{DateTime, Local, TimeZone};
fn format_date(timestamp: i64) -> String {
    let datetime: DateTime<Local> = Local.timestamp(timestamp, 0);
    return datetime.format("%d.%m.%Y").to_string();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediathekViewWeb {
    pub result: Results,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    pub title: String,
    pub topic: String,
    pub description: String,
    pub timestamp: i64,
    pub duration: u64,
    pub channel: String,
    pub human_duration: Option<String>,
    pub date: Option<String>,
    pub url_video_hd: String,
    pub url_video_low: String,
    pub url_video: String,
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Results {
    pub results: Vec<Object>,
}
