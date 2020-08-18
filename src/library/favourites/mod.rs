use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use rocket_contrib::json::Json;
extern crate reqwest;

pub fn get_favourites() -> Result {
    let client = reqwest::Client::new();
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers
    }
    let tjson = json!({"queries":[{"fields":["title","topic"],"query":"Die Anstalt"},{"fields":["channel"],"query":"3sat"}],"sortBy":"timestamp","sortOrder":"desc"});

    let res = client
        .post("https://mediathekviewweb.de/api/query")
        .body(tjson.to_string())
        .headers(construct_headers())
        .send()
        .unwrap()
        .text();

    let results: Result = serde_json::from_str(&res.unwrap()).unwrap();
    results
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub result: Results,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub channel: String,
    pub topic: String,
    pub title: String,
    pub description: String,
    pub timestamp: i32,
    pub duration: i32,
    pub size: i32,
    pub url_website: String,
    pub url_subtitle: String,
    pub url_video: String,
    pub url_video_low: String,
    pub url_video_hd: String,
    pub filmlisteTimestamp: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
    pub results: Vec<Object>,
}
