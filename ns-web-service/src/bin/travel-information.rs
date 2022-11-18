use actix_web::{middleware::Logger, middleware::NormalizePath, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::{env, io};

#[path = "../travel-information/models/ns-api/mod.rs"]
mod api_models;
#[path = "../travel-information/database/mod.rs"]
mod database;
#[path = "../travel-information/download/mod.rs"]
mod download;
#[path = "../travel-information/error/errors.rs"]
mod errors;
#[path = "../travel-information/handlers/mod.rs"]
mod handlers;
#[path = "../travel-information/models/mod.rs"]
mod models;
#[path = "../travel-information/routing/routes.rs"]
mod routes;
#[path = "../travel-information/routing/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = generate_database_url();

    let db_pool = MySqlPool::connect(&database_url).await.unwrap();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("sqlx::query", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()
        .unwrap();

    let shared_data = web::Data::new(AppState { pool: db_pool });

    let app = move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(departure_routes)
            .default_service(web::to(handlers::error::error_page_handler))
    };

    let app_url = env::var("APP_URL").expect("APP_URL is not set in the .env file.");

    println!("Server is running on: {}", app_url);

    HttpServer::new(app).bind(&app_url)?.run().await
}

fn generate_database_url() -> String {
    let connection = env::var("DB_CONNECTION").expect("DB_CONNECTION is not set in the .env file.");
    let host = env::var("DB_HOST").expect("DB_HOST is not set in the .env file.");
    let port = env::var("DB_PORT").expect("DB_PORT is not set in the .env file.");
    let database = env::var("DB_DATABASE").expect("DB_DATABASE is not set in the .env file.");
    let username = env::var("DB_USERNAME").expect("DB_USERNAME is not set in the .env file.");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD is not set in the .env file.");

    let result = format!(
        "{}://{}:{}@{}:{}/{}",
        connection, username, password, host, port, database
    );

    result
}
