use crate::mounts::library::series;
use actix_web::{error, web, Error,HttpRequest, HttpResponse};


pub async fn index(
    tmpl: web::Data<tera::Tera>,
    ) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let series = series::get_series_season_aggregation();
    ctx.insert("series", &series);
    let output = tmpl.render("series.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}

pub async fn detail(
    tmpl: web::Data<tera::Tera>,
    req: HttpRequest
    ) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();
    let serie_detail : i32 = id.parse().unwrap();
    let j = match serde_json::to_string(&serie_detail) {
        Ok(j) => j,
        Err(_) => panic!("could not connect to socket"),
    };

    HttpResponse::Ok().json(serde_json::to_string(&j).unwrap()) // <- send response
}

