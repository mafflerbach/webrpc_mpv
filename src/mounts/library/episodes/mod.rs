//use rocket::response::content;
//use rocket_contrib::templates::Template;
use crate::mounts::episodes;
use actix_web::{error, web, Error,HttpRequest, HttpResponse};
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
pub async fn index(
    tmpl: web::Data<tera::Tera>,
    req: HttpRequest
    ) -> Result<HttpResponse, Error> {

    let series_id_req = req.match_info().get("series_id").unwrap();
    let series_id : i32 = series_id_req.parse().unwrap();

    let season_id_req = req.match_info().get("season_id").unwrap();
    let season_id : i32 = season_id_req.parse().unwrap();

    let episodes = episodes::get_episodes(series_id, season_id);

    let mut ctx = tera::Context::new();
    ctx.insert("episodes", &episodes);
    let output = tmpl.render("episodes.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}


pub async fn detail(
    tmpl: web::Data<tera::Tera>,
    req: HttpRequest
    ) -> HttpResponse {

    let series_id_req = req.match_info().get("series_id").unwrap();
    let series_id : i32 = series_id_req.parse().unwrap();

    let season_id_req = req.match_info().get("season_id").unwrap();
    let season_id : i32 = season_id_req.parse().unwrap();

    let episode_req = req.match_info().get("episode_id").unwrap();
    let episode : i32 = season_id_req.parse().unwrap();

    let episode_details = episodes::get_detail(series_id, season_id, episode);

    let j = match serde_json::to_string(&episode_details) {
        Ok(j) => j,
        Err(_) => panic!("could not connect to socket"),
    };

    HttpResponse::Ok().json(serde_json::to_string(&j).unwrap()) // <- send response
}
//#[get("/<series_id>/<season_id>/<episode>")]
//pub fn detail(series_id: i32, season_id: i32, episode: i32) -> content::Json<String> {
    //let episode_details = episodes::get_detail(series_id, season_id, episode);

    //let j = match serde_json::to_string(&episode_details) {
        //Ok(j) => j,
        //Err(_) => panic!("could not connect to socket"),
    //};
    //content::Json(j)
//}
