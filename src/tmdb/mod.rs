pub mod tmdb {
    use std::vec::Vec;
    /// {
    ///   "page": 1,
    ///   "total_results": 1,
    ///   "total_pages": 1,
    ///   "results": [
    ///     {
    ///       "original_name": "Marvel's Agents of S.H.I.E.L.D.",
    ///       "genre_ids": [
    ///         18,
    ///         10759,
    ///         10765
    ///       ],
    ///       "name": "Marvel's Agents of S.H.I.E.L.D.",
    ///       "popularity": 97.514,
    ///       "origin_country": [
    ///         "US"
    ///       ],
    ///       "vote_count": 1869,
    ///       "first_air_date": "2013-09-24",
    ///       "backdrop_path": "/mUCV0W6TaAk8UWA5yAmqCywC63F.jpg",
    ///       "original_language": "en",
    ///       "id": 1403,
    ///       "vote_average": 7.1,
    ///       "overview": "Agent Phil Coulson of S.H.I.E.L.D. (Strategic Homeland Intervention, Enforcement and Logistics Division) puts together a team of agents to investigate the new, the strange and the unknown around the globe, protecting the ordinary from the extraordinary.",
    ///       "poster_path": "/gHUCCMy1vvj58tzE3dZqeC9SXus.jpg"
    ///     }
    ///   ]
    /// }
    /// Result of https://api.themoviedb.org/3/search/tv?api_key=<APIKEY>&language=en-US&page=1&query=Marvel%20agent%20of%20shield&include_adult=false

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SearchResultResponse {
        pub results: Vec<SearchResult>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SearchResult {
        name: String,
        pub id: i32,
        poster_path: Option<String>,
        overview: Option<String>,
    }
    use crate::settings;
    use crate::stubs;
    use url::form_urlencoded::parse;
    pub fn search(search_term: String) -> SearchResultResponse {
        let settings = settings::init();
        // url decode for search
        let decoded_search_term: String = parse(search_term.as_bytes())
            .map(|(key, val)| [key, val].concat())
            .collect();
        let _test = format!( "https://api.themoviedb.org/3/search/tv?api_key={}&language=en-US&page=1&query={}&include_adult=false", settings.tmdb_key, decoded_search_term);
        //  let response =  send_request(test.to_string()).unwrap();

        let response =
            stubs::read_fixture_file("/home/maren/development/rust/mpv/test/searchFixture.json");
        let p: SearchResultResponse = serde_json::from_str(response.as_str()).unwrap();
        return p;
    }

    /// {
    ///   "_id": "5e963a12904f6d0013a57239",
    ///   "air_date": "2020-05-27",
    ///   "episodes": [
    ///     {
    ///       "air_date": "2020-05-27",
    ///       "episode_number": 1,
    ///       "id": 2226179,
    ///       "name": "The New Deal",
    ///       "overview": "Coulson and the Agents of S.H.I.E.L.D. are thrust backward in time and stranded in 1931 New York City. With the all-new Zephyr set to time-jump at any moment, the team must hurry to find out exactly what happened.",
    ///       "production_code": "",
    ///       "season_number": 7,
    ///       "show_id": 1403,
    ///       "still_path": "/gsqIC0yTTZNxxNIvf0NSsRDRHJy.jpg",
    ///       "vote_average": 6.833,
    ///       "vote_count": 6,
    ///       "crew": [],
    ///       "guest_stars": []
    ///     },
    ///     ...
    ///   ],
    ///   "name": "Season 7",
    ///   "overview": "Coulson and the Agents of S.H.I.E.L.D. are thrust backward in time and stranded in 1931 New York City. With the all-new Zephyr set to time-jump at any moment, the team must hurry to find out exactly what happened. If they fail, it would mean disaster for the past, present and future of the world.",
    ///   "id": 147976,
    ///   "poster_path": "/zu5HCP84rcBJJhoIQAafMXMeU2p.jpg",
    ///   "season_number": 7
    /// }
    ///
    /// result from https://api.themoviedb.org/3/tv/1403/season/7?api_key=<APIKEY>&language=en-US

    #[derive(Serialize, Deserialize, Debug)]
    struct SeasonResult {
        episodes: Vec<Episode>,
        overview: String,
        id: i32,
        poster_path: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Episode {
        id: i32,
        name: String,
        overview: String,
        season_number: i32,
        episode_number: i32,
    }
    //pub fn tv_season_result () -> SeasonResult {
    //}

    extern crate reqwest;
    fn send_request(target: String) -> Result<String, reqwest::Error> {
        //TODO change to post, add fields target for video url and id = 0 for local

        let client = reqwest::Client::new();
        let result = client.get(&target.clone().to_string()).send()?.text();
        println!("{:?}", result);
        return result;
    }
}
