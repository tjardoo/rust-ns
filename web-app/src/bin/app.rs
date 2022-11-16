#[path = "../app/mod.rs"]
mod app;

use actix_web::{middleware::Logger, middleware::NormalizePath, web, App, HttpServer};
use app::{handlers, routes, state};
use dotenv::dotenv;
use std::{env, io};
use tera::Tera;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let shared_data = web::Data::new(state::AppState {});

    let app = move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();

        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .app_data(shared_data.clone())
            .app_data(web::Data::new(tera))
            .configure(routes::general_routes)
            .configure(routes::departure_routes)
            .configure(routes::station_routes)
            .configure(routes::platform_routes)
            .default_service(web::to(handlers::error::error_page_handler))
    };

    let app_url = env::var("APP_URL").expect("APP_URL is not set in the .env file.");

    println!("Server is running on: {}", app_url);

    HttpServer::new(app).bind(&app_url)?.run().await
}
