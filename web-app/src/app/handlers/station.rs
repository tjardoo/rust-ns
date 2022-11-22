use actix_web::{
    http::{header::ContentType, StatusCode},
    web, Error, HttpResponse,
};
use awc::Client;
use chrono::Utc;
use serde_json::Value;
use std::env;

use crate::app::{errors::WebAppError, models::departure::Departure};

pub async fn show_station_display(template: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let awc_client = Client::default();

    let target_url = env::var("TARGET_URL").expect("TARGET_URL is not set in the .env file.");
    let station_code = env::var("STATION_CODE").expect("STATION_CODE is not set in the .env file.");
    let limit = env::var("SCREEN_ROW_SIZE").expect("SCREEN_ROW_SIZE is not set in the .env file.");

    let url = format!("{}/station/{}?limit={}", target_url, station_code, limit);

    let mut response = awc_client.get(url).send().await.unwrap();

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(HttpResponse::NotFound()
            .body("Incorrect TARGET_URL provided. Could not fetch the data."));
    }

    let json_response = response.body().await?;

    let value: Value = serde_json::from_str(&std::str::from_utf8(&json_response)?)?;

    let inner_value = &value["data"]["departures"];

    let departures: Vec<Departure> = serde_json::from_value(inner_value.clone()).unwrap();

    println!("{:?}", departures);

    let mut view_file_name = "station.html";

    if departures.len() == 0 {
        view_file_name = "no_data.html";
    }

    let current_time = Utc::now().format("%H:%M").to_string();

    let mut ctx = tera::Context::new();

    ctx.insert("current_time", &current_time);
    ctx.insert("departures", &departures);

    let rendered = template
        .render(view_file_name, &ctx)
        .map_err(|error| WebAppError::TeraError(error.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
