//use rocket::response::content;
//use rocket_contrib::templates::Template;
//use crate::mounts::episodes;

//#[derive(Debug, Serialize, Deserialize)]
//struct TemplateContext {
    //episodes: Vec<mpv_webrpc::models::Episode>,
//}

//#[get("/<series_id>/<season_id>")]
//pub fn index(series_id: i32, season_id: i32) -> Template {
    //let episodes = episodes::get_episodes(series_id, season_id);
    //let return_context = TemplateContext { episodes: episodes };
    //Template::render("episodes", &return_context)
//}

//#[get("/<series_id>/<season_id>/<episode>")]
//pub fn detail(series_id: i32, season_id: i32, episode: i32) -> content::Json<String> {
    //let episode_details = episodes::get_detail(series_id, season_id, episode);

    //let j = match serde_json::to_string(&episode_details) {
        //Ok(j) => j,
        //Err(_) => panic!("could not connect to socket"),
    //};
    //content::Json(j)
//}
