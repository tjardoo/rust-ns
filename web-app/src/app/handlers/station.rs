use actix_web::{
    http::{header::ContentType, StatusCode},
    web, Error, HttpResponse,
};
use awc::Client;
use chrono::{FixedOffset, Utc};
use serde_json::Value;
use std::env;

use crate::app::{errors::WebAppError, models::departure::Departure};

pub async fn show_station_display(template: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let awc_client = Client::default();

    let target_url = env::var("TARGET_URL").expect("TARGET_URL is not set in the .env file.");
    let station_code = env::var("STATION_CODE").expect("STATION_CODE is not set in the .env file.");
    let limit = env::var("SCREEN_ROW_SIZE").expect("SCREEN_ROW_SIZE is not set in the .env file.");
    let page_reload_in_seconds =
        env::var("PAGE_RELOAD_IN_SEC").expect("PAGE_RELOAD_IN_SEC is not set in the .env file.");

    let url = format!("{}/station/{}?limit={}", target_url, station_code, limit);

    let response_result = awc_client.get(url).send().await;

    if let Err(error) = response_result {
        return Ok(HttpResponse::ServiceUnavailable().body(error.to_string()));
    }

    let mut response = response_result.unwrap();

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(HttpResponse::NotFound()
            .body("Incorrect TARGET_URL provided. Could not fetch the data."));
    }

    let json_response = response.body().await?;

    let value: Value = serde_json::from_str(&std::str::from_utf8(&json_response)?)?;

    let inner_value = &value["data"]["departures"];

    let departures: Vec<Departure> = serde_json::from_value(inner_value.clone()).unwrap();

    let mut view_file_name = "station.html";

    if departures.len() == 0 {
        view_file_name = "no_data.html";
    }

    let current_time = Utc::now()
        .with_timezone(&FixedOffset::east(3600))
        .format("%H:%M")
        .to_string();

    let mut ctx = tera::Context::new();

    ctx.insert("current_time", &current_time);
    ctx.insert("departures", &departures);
    ctx.insert("page_reload_in_seconds", &page_reload_in_seconds);

    let rendered = template
        .render(view_file_name, &ctx)
        .map_err(|error| WebAppError::TeraError(error.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
