use crate::mounts::library::series;
use rocket::response::content;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    let series = series::get_series_season_aggregation();

    Template::render("series", &series)
}

#[get("/detail/<id>")]
pub fn detail(id: i32) -> content::Json<String> {
    let serie_detail = series::get_detail(id);

    let j = match serde_json::to_string(&serie_detail) {
        Ok(j) => j,
        Err(_) => panic!("could not connect to socket"),
    };
    content::Json(j)
}
