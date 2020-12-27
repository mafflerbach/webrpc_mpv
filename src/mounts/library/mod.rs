extern crate lazy_static;

use actix_web::{error, web, Error, HttpResponse};

use serde::{Serialize, Deserialize};
pub mod episodes;
pub mod series;

use crate::library;
use crate::settings;
use crate::tmdb;
use diesel::prelude::*;
use mpv_webrpc::models::*;
use serde_json::json;

pub async fn request_scan(
    tmpl: web::Data<tera::Tera>,
    ) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();


    let path_entries = library::get_first_level();
    let settings = settings::init();
    let mut test = Vec::new();
    let mut tmdb_response;

    for entry in &path_entries {
        tmdb_response = tmdb::tmdb::search(entry.to_string());
        for mut result in tmdb_response.results {
            if !library::check_tmdb_id(result.id) {
                let file_path: Option<String> =
                    Some(format!("{}{}", settings.scan_dir_series, entry));
                result.file_path = file_path;
                result.type_of = serde::export::Some("tv".to_string());
                test.push(result);
            }
        }
    }

    let testdas = library::scan_movies(test).to_vec();

    #[derive(Debug, Serialize, Deserialize)]
    struct TemplateContext {
        results: Vec<tmdb::tmdb::SearchResult>,
    }

    let return_context = TemplateContext { results: testdas };

    ctx.insert("movies", &return_context);
    let output = tmpl.render("searchResult.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}




fn add_serie(request_content: LibraryRequest) -> serde_json::Value {
    fn first<T>(v: &Vec<T>) -> Option<&T> {
        v.first()
    }
    let external_id = tmdb::tmdb::get_external_id(request_content.tmdb_id);

    let serie_information = tmdb::tmdb::find_by_external_id(external_id.tvdb_id);

    let info_vec = &first(&serie_information.tv_results).unwrap();
    let serie_info = NewSerie {
        imagepath: &info_vec.poster_path.as_ref().unwrap(),
        tmdb_id: &request_content.tmdb_id,
        title: &info_vec.name,
        description: &info_vec.overview.as_ref().unwrap(),
    };

    if !serie_info.check_serie() {
        let connection = mpv_webrpc::establish_connection();
        use mpv_webrpc::schema::serie;
        let _insert_result = diesel::insert_into(serie::table)
            .values(&serie_info)
            .execute(&connection);

        library::sync_episodes(request_content.path.unwrap(), request_content.tmdb_id);
    }

    json!({
        "data": "Added serie to DB",
        "message": "",
        "request_id": 0
    })
}

fn add_movie(request_content: LibraryRequest) -> serde_json::Value {
    // TODO gui for adding movies via search on tmdb
    let movie_details = tmdb::tmdb::movie_get_detail_by_id(&request_content.tmdb_id);

    let movie_info = NewMovie {
        imagepath: &movie_details.poster_path.unwrap(),
        title: &movie_details.title,
        path: &request_content.path.unwrap(),
        description: &movie_details.overview.unwrap(),
        tmdb_id: &movie_details.id,
    };

    let connection = mpv_webrpc::establish_connection();
    use mpv_webrpc::schema::movie;
    let _insert_result = diesel::insert_into(movie::table)
        .values(&movie_info)
        .execute(&connection);

    json!({
        "data": "Added movie to DB",
        "message": "",
        "request_id": 0
    })
}


pub async fn request_add(body: web::Bytes) -> HttpResponse {
    let result : LibraryRequest = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    if result.schema.is_none() {
        let tjson = json!({ "error": "missing schema wait for serie or movie" });
        return HttpResponse::BadRequest().json(tjson.to_string())
    }
    if result.path.is_none() {
        let tjson = json!({ "error": "missing path" });
        return HttpResponse::BadRequest().json(tjson.to_string())
    }

    let schema = result.schema.unwrap();
    let response; 

    if schema == String::from("movie") {
        let movie : LibraryRequest = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
        response = add_movie(movie);
        return HttpResponse::Ok().json(response) // <- send response
    }
    if schema == String::from("serie") {
        let serie : LibraryRequest = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
        response = add_serie(serie);
        return HttpResponse::Ok().json(response) // <- send response
    }

    let tjson = json!({ "error": "Something went wrong" });
    return HttpResponse::BadRequest().json(tjson.to_string())
}


#[derive(Serialize, Deserialize)]
pub struct LibraryRequest {
    pub tmdb_id: i32,
    pub schema: Option<String>,
    pub path: Option<String>
}

pub async fn request_ignore_serie(body: web::Bytes) -> HttpResponse {
    let result : LibraryRequest = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    use mpv_webrpc::schema::ignored;
    let connection = mpv_webrpc::establish_connection();

    let ignore_serie = NewIgnored {
        tmdb_id: &result.tmdb_id,
    };

    let insert_result = diesel::insert_into(ignored::table)
        .values(&ignore_serie)
        .execute(&connection);
    let message;

    match insert_result {
        Ok(v) => message = format!("{:?}", v),
        Err(e) => message = format!("{:?}", e),
    }

    let test = json!({
        "data": "ok",
        "message": message,
        "request_id": 0
    });


    HttpResponse::Ok().json(test) // <- send response
}

