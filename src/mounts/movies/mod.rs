use crate::library::movies;
use rocket::response::content;
use rocket_contrib::templates::Template;

use crate::tmdb;
#[derive(Debug, Serialize, Deserialize)]
struct TemplateContext {
    movies: Vec<mpv_webrpc::models::Movie>,
}
#[get("/")]
pub fn index() -> Template {
    let movies = movies::get_movies();

    let return_context = TemplateContext { movies: movies };
    Template::render("movies", &return_context)
}

#[get("/<id>")]
pub fn detail(id: i32) -> content::Json<String> {
    let movie_detail = movies::get_detail(id);

    let j = match serde_json::to_string(&movie_detail) {
        Ok(j) => j,
        Err(_) => panic!("could not connect to socket"),
    };
    content::Json(j)
}

#[derive(Serialize, Deserialize)]
pub struct TmdbSearchTerm {
    pub term: String,
}

use rocket_contrib::json::Json;
#[post("/search-movie", data = "<request_content>")]
pub fn request_search_movie_post(request_content: Json<TmdbSearchTerm>) -> Template {
    let term = &request_content.term;
    let tmdb_response = tmdb::tmdb::search_movie(term.to_string());

    #[derive(Serialize, Deserialize)]
    struct TemplateContext {
        results: tmdb::tmdb::SearchMovieResultResponse,
    }
    //let return_context = TemplateContext { results: tmdb_response };
    Template::render("searchMovieResult", &tmdb_response)
}
