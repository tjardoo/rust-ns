use actix_web::web;
use actix_web::HttpResponse;
use awc::Client;
use std::env;

use crate::app::errors::WebAppError;
use crate::app::models::departure::Departure;
use crate::app::models::departure_list::DepartureList;

pub async fn get_station_departure_overview() -> Result<HttpResponse, WebAppError> {
    let awc_client = Client::default();

    let target_url = env::var("TARGET_URL").expect("TARGET_URL is not set in the .env file.");
    let station_code = env::var("STATION_CODE").expect("STATION_CODE is not set in the .env file.");
    let url = format!("{}/station/{}/departures", target_url, station_code);

    println!("Visiting {:#?}", url);

    let response = awc_client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<DepartureList>>()
        .await
        .unwrap();

    let json_response = web::Json(response);

    println!("{:#?}", json_response);

    Ok(HttpResponse::Ok().json(json_response))
}

pub async fn get_station_departure_by_id(
    params: web::Path<i32>,
) -> Result<HttpResponse, WebAppError> {
    let departure_id = params.into_inner();

    let awc_client = Client::default();

    let target_url = env::var("TARGET_URL").expect("TARGET_URL is not set in the .env file.");
    let station_code = env::var("STATION_CODE").expect("STATION_CODE is not set in the .env file.");
    let url = format!(
        "{}/station/{}/departures/{}",
        target_url, station_code, departure_id
    );

    println!("Visiting {:#?}", url);

    // TODO catch error message "error_message	"Departure with given id not found.""

    let response = awc_client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Departure>()
        .await
        .unwrap();

    let json_response = web::Json(response);

    println!("{:#?}", json_response);

    Ok(HttpResponse::Ok().json(json_response))
}
