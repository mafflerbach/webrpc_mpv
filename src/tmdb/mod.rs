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

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SearchResult {
        pub name: String,
        pub id: i32,
        pub file_path: Option<String>,
        pub poster_path: Option<String>,
        pub overview: Option<String>,
        pub type_of: Option<String>,
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

        let settings2 = &settings::config();
        let mut tmdb_search_tv: String = settings2.as_ref().unwrap().tmdb["search_tv"].to_string();
        tmdb_search_tv = tmdb_search_tv.replace("##1##", &settings.tmdb_key);
        tmdb_search_tv = tmdb_search_tv.replace("##2##", &decoded_search_term);

        println!("Used Uri:{}", tmdb_search_tv);
        let response = send_request(tmdb_search_tv);
        let p: SearchResultResponse = serde_json::from_str(response.as_str()).unwrap();

        println!("{:#?}", p);
        if p.results.len() == 0 {
            println!("No Entries for: {}", search_term);
        }

        return p;
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SearchMovieResultResponse {
        pub results: Vec<SearchMovieResult>,
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SearchMovieResult {
        pub title: String,
        pub id: i32,
        pub file_path: Option<String>,
        pub poster_path: Option<String>,
        pub overview: Option<String>,
        pub type_of: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TemplateContext {
        results: Vec<SearchResult>,
    }
    pub fn search_movie(search_term: String) -> SearchMovieResultResponse {
        let settings = settings::init();
        // url decode for search
        let decoded_search_term: String = parse(search_term.as_bytes())
            .map(|(key, val)| [key, val].concat())
            .collect();

        let settings2 = &settings::config();
        let mut tmdb_search_movie: String = settings2.as_ref().unwrap().tmdb["search_movie"].to_string();
        tmdb_search_movie = tmdb_search_movie.replace("##1##", &settings.tmdb_key);
        tmdb_search_movie = tmdb_search_movie.replace("##2##", &decoded_search_term);

        let response = send_request(tmdb_search_movie.to_string());
        let p: SearchMovieResultResponse = serde_json::from_str(response.as_str()).unwrap();

        if p.results.len() == 0 {
            println!("Empty result for seach Term: {}", search_term);
        }

        return p;
    }

    pub fn movie_get_detail_by_id(id: &i32) -> SearchMovieResult {
        let settings = settings::init();

        let settings2 = &settings::config();
        let mut tmdb_search_movie_by_detail: String = settings2.as_ref().unwrap().tmdb["movie_get_detail_by_id"].to_string();
        tmdb_search_movie_by_detail = tmdb_search_movie_by_detail.replace("##1##", &id.to_string());
        tmdb_search_movie_by_detail = tmdb_search_movie_by_detail.replace("##2##", &settings.tmdb_key);

        let response = send_request(tmdb_search_movie_by_detail.to_string());
        let p: SearchMovieResult = serde_json::from_str(response.as_str()).unwrap();

        return p;
    }
    /// {
    ///   "id": 1403,
    ///   "imdb_id": "tt2364582",
    ///   "freebase_mid": "/m/0lqhm3l",
    ///   "freebase_id": null,
    ///   "tvdb_id": 263365,
    ///   "tvrage_id": 32656,
    ///   "facebook_id": "AgentsofShield",
    ///   "instagram_id": "agentsofshield",
    ///   "twitter_id": "AgentsofSHIELD"
    /// }    
    /// https://api.themoviedb.org/3/tv/1403/external_ids?api_key=924375ec86dee2a9a78b5033367f4fe1&language=en-US

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetExternalIdResponse {
        pub tvdb_id: i32,
    }

    pub fn get_external_id(tmdb_id: i32) -> GetExternalIdResponse {
        let settings = settings::init();
        // url decode for search

        let settings2 = &settings::config();
        let mut get_external_id: String = settings2.as_ref().unwrap().tmdb["get_external_id"].to_string();
        get_external_id = get_external_id.replace("##1##", &tmdb_id.to_string());
        get_external_id = get_external_id.replace("##2##", &settings.tmdb_key);

        let response = send_request(get_external_id);

        let p: GetExternalIdResponse = serde_json::from_str(response.as_str()).unwrap();
        return p;
    }

    /// {
    ///   "movie_results": [],
    ///   "person_results": [],
    ///   "tv_results": [
    ///     {
    ///       "original_name": "Marvel's Agents of S.H.I.E.L.D.",
    ///       "id": 1403,
    ///       "name": "Marvel's Agents of S.H.I.E.L.D.",
    ///       "vote_count": 1905,
    ///       "vote_average": 7.2,
    ///       "first_air_date": "2013-09-24",
    ///       "poster_path": "/gHUCCMy1vvj58tzE3dZqeC9SXus.jpg",
    ///       "genre_ids": [
    ///         18,
    ///         10759,
    ///         10765
    ///       ],
    ///       "original_language": "en",
    ///       "backdrop_path": "/mUCV0W6TaAk8UWA5yAmqCywC63F.jpg",
    ///       "overview": "Agent Phil Coulson of S.H.I.E.L.D. (Strategic Homeland Intervention, Enforcement and Logistics Division) puts together a team of agents to investigate the new, the strange and the unknown around the globe, protecting the ordinary from the extraordinary.",
    ///       "origin_country": [
    ///         "US"
    ///       ],
    ///       "popularity": 126.704
    ///     }
    ///   ],
    ///   "tv_episode_results": [
    ///     {
    ///       "air_date": "1998-11-20",
    ///       "episode_number": 7,
    ///       "id": 143977,
    ///       "name": "Episode 6-07",
    ///       "overview": "New Conservative Leader   Newly elected Conservative Party leader Joe Clark demands a recount    Martha Stewart News   Martha Stewart chats with birthday boy Prince Charles    Videopion - Co-ed Scouts   Old boys Ernie and Harold share their thoughts on co-ed scouting    Quebec Debate '98   The Quebec election debate pits Lucien Bouchard and Jean Charest against moderator, Mike from Canmore    Saddam and Bobbie   At the Saddam Hussein Golf Classic, UN inspectors putt for show and explode for dough    Sadsack Psychiatrist   A psychiatrist attempts to help a man who has a dysfunctional family    Jock McBile   Jock scans the headlines and rants about Jean Chretien in Kuala Lumpur, Doug Flutie, Eaton's new boss, Zippergate, the Reform Party, BC Cigarette Tax, Toronto Police strip searches, Quebec Medicare, and The Grey Cup",
    ///       "production_code": "",
    ///       "season_number": 6,
    ///       "show_id": 2091,
    ///       "still_path": "/opsY5u9kPoMN4YPLl8Y5I4Xl8Uk.jpg",
    ///       "vote_average": 0,
    ///       "vote_count": 0
    ///     }
    ///   ],
    ///   "tv_season_results": []
    /// }
    /// https://api.themoviedb.org/3/find/263365?api_key=924375ec86dee2a9a78b5033367f4fe1&language=en-US&external_source=tvdb_id
    #[derive(Serialize, Deserialize, Debug)]
    pub struct FindByExternalIdResponse {
        pub tv_results: Vec<SearchResult>,
    }
    pub fn find_by_external_id(external_id: i32) -> FindByExternalIdResponse {
        let settings = settings::init();
        // url decode for search

        let settings2 = &settings::config();
        let mut find_by_external_id: String = settings2.as_ref().unwrap().tmdb["find_by_external_id"].to_string();
        find_by_external_id = find_by_external_id.replace("##1##", &external_id.to_string());
        find_by_external_id = find_by_external_id.replace("##2##", &settings.tmdb_key);

        let response = send_request(find_by_external_id);

        let p: FindByExternalIdResponse = serde_json::from_str(response.as_str()).unwrap();
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
    pub struct SeasonResult {
        pub episodes: Vec<Episode>,
        pub name: String,
        pub overview: String,
        pub id: i32,
        pub poster_path: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Episode {
        pub id: i32,
        pub name: String,
        pub overview: String,
        pub season_number: i32,
        pub episode_number: i32,
    }
    pub fn tv_season_get_details(tmdb_id: i32, season_id: i32) -> SeasonResult {
        let settings = settings::init();
        // url decode for search

        let settings2 = &settings::config();
        let mut tv_season_get_detail: String = settings2.as_ref().unwrap().tmdb["tv_season_get_details"].to_string();
        tv_season_get_detail = tv_season_get_detail.replace("##1##", &tmdb_id.to_string());
        tv_season_get_detail = tv_season_get_detail.replace("##2##", &season_id.to_string());
        tv_season_get_detail = tv_season_get_detail.replace("##3##", &settings.tmdb_key);

        let response = send_request(tv_season_get_detail);

        let p: SeasonResult = serde_json::from_str(response.as_str()).unwrap();
        return p;
    }

    pub fn tv_episodes_get_details(tmdb_id: i32, season: i32, episode: i32) -> Episode {
        let settings = settings::init();
        // url decode for search

        let settings2 = &settings::config();
        let mut tv_episodes_get_details: String = settings2.as_ref().unwrap().tmdb["tv_episodes_get_details"].to_string();
        tv_episodes_get_details = tv_episodes_get_details.replace("##1##", &tmdb_id.to_string());
        tv_episodes_get_details = tv_episodes_get_details.replace("##2##", &season.to_string());
        tv_episodes_get_details = tv_episodes_get_details.replace("##3##", &episode.to_string());
        tv_episodes_get_details = tv_episodes_get_details.replace("##4##", &settings.tmdb_key);

        let response = send_request(tv_episodes_get_details);

        let p: Episode = serde_json::from_str(response.as_str()).unwrap();
        return p;
    }

    extern crate reqwest;
    fn send_request(target: String) -> String {

        let settings2 = &settings::config();
        let debug = settings2.as_ref().unwrap().debug.to_string();
        if debug == "true" {
            let response = stubs::read_fixture_file(&target.clone().to_string());
            return response;
        }
        let client = reqwest::Client::new();
        let result = client.get(&target.clone().to_string()).send().unwrap().text();
        println!("RESPONSE: {:?}", result);
        return result.unwrap();
    }
}
