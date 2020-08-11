use crate::mounts::library::movies;
use rocket::response::content;
use rocket_contrib::templates::Template;

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

#[get("/detail/<id>")]
pub fn detail(id: i32) -> content::Json<String> {
    let movie_detail = movies::get_detail(id);

    let j = match serde_json::to_string(&movie_detail) {
        Ok(j) => j,
        Err(_) => panic!("could not connect to socket"),
    };
    content::Json(j)
}
