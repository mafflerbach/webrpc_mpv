use crate::library::movies;
use actix_web::{error, web, Error,HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::tmdb;

pub async fn index(
    tmpl: web::Data<tera::Tera>,
    ) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let movies = movies::get_movies();
    ctx.insert("movies", &movies);
    let output = tmpl.render("movies.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}


pub async fn detail(
    req: HttpRequest
    ) -> HttpResponse {
    let id = req.match_info().get("tmdb_id").unwrap();
    let movie_detail_id : i32 = id.parse().unwrap();
    let movie_detail = movies::get_detail(movie_detail_id);

    let j = match serde_json::to_string(&movie_detail) {
        Ok(j) => j,
        Err(_) => panic!("could not connect to socket"),
    };

    HttpResponse::Ok().json(j)
}


#[derive(Serialize, Deserialize)]
pub struct TmdbSearchTerm {
    pub term: String,
}

pub async fn search_movie_term (
    tmpl: web::Data<tera::Tera>,
    body: web::Bytes
    )  -> Result<HttpResponse, Error> {

    let result : TmdbSearchTerm = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    let tmdb_response = tmdb::tmdb::search_movie(result.term.to_string());

    let mut ctx = tera::Context::new();
    ctx.insert("results", &tmdb_response);
    let output = tmpl.render("searchMovieResult.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(output))

}
