use crate::library::favourites;
use rocket::response::content;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    let favourites = favourites::get_favourites();

    Template::render("favourites", &favourites)
}

