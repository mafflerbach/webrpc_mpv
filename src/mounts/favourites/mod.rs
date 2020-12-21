use crate::library::favourites;
use crate::library::favourites::MediathekViewWeb;
use crate::settings;
use actix_web::{error, web, Error, HttpResponse};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct RequestContent {
    pub offset: i32,
    pub search_term: String,
}

pub async fn search(
    tmpl: web::Data<tera::Tera>,
    body: web::Bytes
) -> Result<HttpResponse, Error> {

    let request_content : RequestContent = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    let search_term = request_content.search_term.to_string();
    let offset = request_content.offset;
    let query = json!( {
        "queries": [{
            "fields": ["title", "topic", "descption"],
            "query": search_term, 
        }],
        "sortBy": "timestamp",
        "size":25,
        "offset": offset,
        "duration_min": 600,
        "sortOrder": "desc"
    });
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Context {
        pub context: MediathekViewWeb,
    };
    let search_result = favourites::get_favourites(query);

    let mut ctx = tera::Context::new();
    ctx.insert("context", &search_result);
    let output = tmpl.render("mediathekResult.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}

pub async fn index(
    tmpl: web::Data<tera::Tera>,
    ) -> Result<HttpResponse, Error> {
    let settings2 = &settings::config().unwrap();
    let mut context = Vec::new();
    for favourite in &settings2.favourites {
        let id = generate_id();
        let tjson = json!(favourite.query);
        let favourites = favourites::get_favourites(tjson);
        let test = Test {
            name: favourite.name.clone(),
            image: favourite.image.clone(),
            favourite: favourites,
            id: id,
        };
        context.push(test);
    }
    let mut ctx = tera::Context::new();
    ctx.insert("context", &context);
    let output = tmpl.render("favourites.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}

use rand::Rng;
fn generate_id() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[derive(Debug, Serialize, Deserialize)]
struct Context {
    context: Vec<Test>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Test {
    name: String,
    image: String,
    favourite: MediathekViewWeb,
    id: i32,
}
