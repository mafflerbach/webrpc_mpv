use crate::library::favourites;
use crate::library::favourites::MediathekViewWeb;
use crate::settings;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize)]
pub struct RequestContent {
    pub search_term: String,
}

#[post("/search", data = "<request_content>")]
pub fn search(request_content: Json<RequestContent>) -> Template {
    let search_term = request_content.search_term.to_string();
    let query = json!( {
            "queries": [{
                "fields": ["title", "topic", "descption"],
                "query": search_term
            }],
            "duration_min": 2000,
            "sortBy": "timestamp",
            "sortOrder": "desc"
    });
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Context {
        pub context: MediathekViewWeb,
    };
    println!("{:?}", query);
    let search_result = favourites::get_favourites(query);
    let foo = Context {
        context: search_result,
    };
    Template::render("mediathekResult", &foo)
}

#[get("/")]
pub fn index() -> Template {
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
    let foo = Context { context: context };
    Template::render("favourites", &foo)
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
