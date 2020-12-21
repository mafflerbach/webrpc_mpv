extern crate lazy_static;

pub mod episodes;
pub mod series;

//use crate::library;
//use crate::settings;
//use crate::tmdb;
//use diesel::prelude::*;
//use mpv_webrpc::models::*;

//#[get("/scan")]
//pub fn request_scan() -> Template {
    //let path_entries = library::get_first_level();
    //let settings = settings::init();
    //let mut test = Vec::new();
    //let mut tmdb_response;

    //for entry in &path_entries {
        //tmdb_response = tmdb::tmdb::search(entry.to_string());
        //for mut result in tmdb_response.results {
            //if !library::check_tmdb_id(result.id) {
                //let file_path: Option<String> =
                    //Some(format!("{:?}{:?}", settings.scan_dir_series, entry));
                //result.file_path = file_path;
                //result.type_of = serde::export::Some("tv".to_string());
                //test.push(result);
            //}
        //}
    //}

    //let testdas = library::scan_movies(test).to_vec();

    //#[derive(Debug, Serialize, Deserialize)]
    //struct TemplateContext {
        //results: Vec<tmdb::tmdb::SearchResult>,
    //}
    //let return_context = TemplateContext { results: testdas };

    //Template::render("searchResult", &return_context)
//}

//#[get("/rescan", data = "<request_content>")]
//pub fn request_rescan(request_content: Json<LibraryRequest>) {



//}


//#[derive(Serialize, Deserialize)]
//pub struct LibraryRequest {
    //pub tmdb_id: i32,
    //pub path: String,
//}

//#[post("/add", data = "<request_content>")]
//pub fn request_add_serie(request_content: Json<LibraryRequest>) -> content::Json<String> {
    //fn first<T>(v: &Vec<T>) -> Option<&T> {
        //v.first()
    //}
    //let external_id = tmdb::tmdb::get_external_id(request_content.tmdb_id);
    //println!("External id: {}", external_id.tvdb_id);

    //let serie_information = tmdb::tmdb::find_by_external_id(external_id.tvdb_id);

    //let info_vec = &first(&serie_information.tv_results).unwrap();
    //let serie_info = NewSerie {
        //imagepath: &info_vec.poster_path.as_ref().unwrap(),
        //tmdb_id: &request_content.tmdb_id,
        //title: &info_vec.name,
        //description: &info_vec.overview.as_ref().unwrap(),
    //};

    //if !serie_info.check_serie() {
        //let connection = mpv_webrpc::establish_connection();
        //use mpv_webrpc::schema::serie;
        //let _insert_result = diesel::insert_into(serie::table)
            //.values(&serie_info)
            //.execute(&connection);

        //library::sync_episodes(request_content.path.clone(), request_content.tmdb_id);
    //}

    //let test = json!({
        //"data": "ok",
        //"message": "",
        //"request_id": 0
    //});
    //content::Json(test.to_string())
//}

//#[post("/add-movie", data = "<request_content>")]
//pub fn request_add_movie(request_content: Json<LibraryRequest>) -> content::Json<String> {
    //// TODO gui for adding movies via search on tmdb
    //let movie_details = tmdb::tmdb::movie_get_detail_by_id(&request_content.tmdb_id);

    //let movie_info = NewMovie {
        //imagepath: &movie_details.poster_path.unwrap(),
        //title: &movie_details.title,
        //path: &request_content.path,
        //description: &movie_details.overview.unwrap(),
        //tmdb_id: &movie_details.id,
    //};
    //print!("INSERT");
    //print!("{}", &movie_details.title);

    //let connection = mpv_webrpc::establish_connection();
    //use mpv_webrpc::schema::movie;
    //let _insert_result = diesel::insert_into(movie::table)
        //.values(&movie_info)
        //.execute(&connection);

    //let test = json!({
        //"data": "ok",
        //"message": "",
        //"request_id": 0
    //});
    //content::Json(test.to_string())
//}


//#[post("/ignore", data = "<request_content>")]
//pub fn request_ignore_serie(request_content: Json<LibraryRequest>) -> content::Json<String> {
    //use mpv_webrpc::schema::ignored;
    //let connection = mpv_webrpc::establish_connection();

    //let ignore_serie = NewIgnored {
        //tmdb_id: &request_content.tmdb_id,
    //};

    //let insert_result = diesel::insert_into(ignored::table)
        //.values(&ignore_serie)
        //.execute(&connection);
    //let message;

    //match insert_result {
        //Ok(v) => message = format!("{:?}", v),
        //Err(e) => message = format!("{:?}", e),
    //}

    //let test = json!({
        //"data": "ok",
        //"message": message,
        //"request_id": 0
    //});
    //content::Json(test.to_string())
//}
