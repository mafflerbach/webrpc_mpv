
#[macro_use]
extern crate serde_json;
extern crate execute;

use actix_files::Files;  // Taken from the guide
use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer, Result};
use tera::Tera;
mod mpv;
mod mounts;
mod library;
mod settings;
mod api_structs;
mod stubs;
mod tmdb;

// store tera template in application state
async fn index(
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
        let links_context = settings::init();
        // submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("settings", &links_context);
        let output = tmpl.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    mpv::mpv::init();
    // FIXME wait until socket exists
    std::thread::sleep(std::time::Duration::from_millis(3000));

    mpv::mpv::event_load("osd/black.png", "replace");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera =
            Tera::new("templates/**/*.html").unwrap();

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/heartbeat").route(web::get().to(mounts::misc::heartbeat)))
            .service(web::resource("/shutdown").route(web::get().to(mounts::misc::shutdown)))
            // provide volume get and set
            .service(
                web::resource("/volume")
                    .route(web::post().to(mounts::volume::request_change_volume))
                    .route(web::get().to(mounts::volume::request_volume)),

            )
            // append the favourites and search results
            .service(
                web::scope("/favourites")
                    .service(web::resource("").route(web::get().to(mounts::favourites::index)))
                    .service(web::resource("/search").route(web::post().to(mounts::favourites::search)))
            )
            // handles scan, ignore, add ,add-movie
            .service(
                web::scope("/library")
                    .service(web::resource("/ignore").route(web::post().to(mounts::library::request_ignore_serie)))
                    .service(web::resource("/scan").route(web::get().to(mounts::library::request_scan,)))
                    .service(web::resource("/add").route(web::post().to(mounts::library::request_add)))
            )
            // provide series informations
            .service(
                web::scope("/series")
                    .service(web::resource("").route(web::get().to(mounts::series::index)))
                    .service(web::resource("/detail/{id}").route(web::get().to(mounts::series::detail)))
            )
            // provide episode informations
            .service(
                web::scope("/episodes")
                    .service(web::resource("/{series_id}/{season_id}").route(web::get().to(mounts::library::episodes::index)))
                    .service(web::resource("/{series_id}/{season_id}/{episode_id}").route(web::get().to(mounts::library::episodes::detail)))
            )
            // provide movies informations
            .service(
                web::scope("/movies")
                    .service(web::resource("").route(web::get().to(mounts::movies::index)))
                    .service(web::resource("/search-movie").route(web::post().to(mounts::movies::search_movie_term)))
                    .service(web::resource("/{tmdb_id}").route(web::get().to(mounts::movies::detail)))
            )
            // handles play, pause, resume, stop
            .service(
                web::scope("/player")
                    .service(web::resource("").route(web::post().to(mounts::player::request_player)))
                    .service(web::resource("/property").route(web::post().to(mounts::player::request_property)))
            )
            .service(Files::new("/public", "templates/public"))
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let body = tera.render("error.html", &context);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
