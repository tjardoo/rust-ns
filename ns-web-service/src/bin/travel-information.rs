use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::{env, io};

#[path = "../travel-information/handlers/mod.rs"]
mod handlers;
#[path = "../travel-information/routing/routes.rs"]
mod routes;

use routes::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // get the variables from the .env file
    dotenv().ok();

    // generate the database URL
    let database_url = generate_database_url();

    // (optional) print the generated database URL
    // println!("{}", database_url);

    // create a new MySql connection and immediately establishes one connection
    let _db_pool = MySqlPool::connect(&database_url).await.unwrap();

    // create the application and configure the routes
    let app = move || App::new().configure(general_routes);

    let app_url = env::var("APP_URL").expect("APP_URL is not set in the .env file.");

    println!("Server is running on: {}", app_url);

    // start the HTTP server
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
