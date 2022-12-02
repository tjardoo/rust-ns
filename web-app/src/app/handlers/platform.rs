use actix_web::{
    http::{header::ContentType, StatusCode},
    web, Error, HttpResponse,
};
use awc::Client;
use chrono::{FixedOffset, Utc};
use serde_json::Value;
use std::env;

use crate::app::{errors::WebAppError, models::departure::Departure};

pub async fn show_platform_display(template: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let awc_client = Client::default();

    let target_url = env::var("TARGET_URL").expect("TARGET_URL is not set in the .env file.");
    let station_code = env::var("STATION_CODE").expect("STATION_CODE is not set in the .env file.");
    let platform_code =
        env::var("PLATFORM_CODE").expect("PLATFORM_CODE is not set in the .env file.");

    let url = format!(
        "{}/station/{}/platform/{}",
        target_url, station_code, platform_code
    );

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

    let inner_value = &value["data"]["current"];

    let departure: Option<Departure> = serde_json::from_value(inner_value.clone()).unwrap();

    let inner_value = &value["data"]["next"];

    let next_departure: Option<Departure> = serde_json::from_value(inner_value.clone()).unwrap();

    let mut view_file_name = "platform.html";

    if departure.is_none() {
        view_file_name = "no_data.html";
    }

    let current_time = Utc::now()
        .with_timezone(&FixedOffset::east(3600))
        .format("%H:%M")
        .to_string();

    let mut ctx = tera::Context::new();

    ctx.insert("current_time", &current_time);
    ctx.insert("departure", &departure);
    ctx.insert("next_departure", &next_departure);

    let rendered = template
        .render(view_file_name, &ctx)
        .map_err(|error| WebAppError::TeraError(error.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
