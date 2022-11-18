use std::error::Error;
use std::str::FromStr;

use crate::models::api_departure::ApiDeparture;
use crate::models::departure::{FullDeparture, TrainCategory};
use crate::state::AppState;
use crate::{database::departures::*, errors::RustNSError};
use actix_web::{web, HttpResponse};
use log::info;
use openssl::ssl::{SslConnector, SslVerifyMode};
use serde_json::Value;
use std::env;

pub async fn get_departures_by_station_code(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, RustNSError> {
    let station_code = params.into_inner();

    db_get_departures_by_station_code(&app_state.pool, station_code)
        .await
        .map(|departures| HttpResponse::Ok().json(departures))
}

pub async fn get_departure_by_station_code_and_id(
    app_state: web::Data<AppState>,
    params: web::Path<(String, u32)>,
) -> Result<HttpResponse, RustNSError> {
    let (_station_code, departure_id) = params.into_inner();

    let departure = db_get_departure_by_id(&app_state.pool, departure_id).await?;

    let product = db_get_product_by_id(&app_state.pool, departure_id).await?;

    let stations = db_get_stations_by_departure_id(&app_state.pool, departure_id).await?;

    let messages = db_get_messages_by_departure_id(&app_state.pool, departure_id).await?;

    let full_departure = FullDeparture {
        id: departure.id,
        station_code: departure.station_code,
        direction: departure.direction,
        name: departure.name,
        planned_date_time: departure.planned_date_time,
        actual_date_time: departure.actual_date_time,
        planned_track: departure.planned_track,
        product,
        train_category: TrainCategory::from_str(&departure.train_category).unwrap(),
        is_cancelled: departure.is_cancelled,
        route_stations: stations,
        messages: Some(messages),
        departure_status: departure.departure_status,
    };

    Ok(HttpResponse::Ok().json(full_departure))
}

pub async fn get_departure_by_station_code_and_platform_code(
    app_state: web::Data<AppState>,
    params: web::Path<(String, String)>,
) -> Result<HttpResponse, RustNSError> {
    let (station_code, platform_code) = params.into_inner();

    let departure_row = db_get_departure_by_station_code_and_platform_code(
        &app_state.pool,
        station_code,
        platform_code,
    )
    .await?;

    if let None = departure_row {
        return Ok(HttpResponse::Ok().body("Er is momenteel geen reisinformatie beschikbaar"));
    }

    let departure = departure_row.unwrap();

    let product = db_get_product_by_id(&app_state.pool, departure.id).await?;

    let stations = db_get_stations_by_departure_id(&app_state.pool, departure.id).await?;

    let messages = db_get_messages_by_departure_id(&app_state.pool, departure.id).await?;

    let full_departure = FullDeparture {
        id: departure.id,
        station_code: departure.station_code,
        direction: departure.direction,
        name: departure.name,
        planned_date_time: departure.planned_date_time,
        actual_date_time: departure.actual_date_time,
        planned_track: departure.planned_track,
        product,
        train_category: TrainCategory::from_str(&departure.train_category).unwrap(),
        is_cancelled: departure.is_cancelled,
        route_stations: stations,
        messages: Some(messages),
        departure_status: departure.departure_status,
    };

    Ok(HttpResponse::Ok().json(full_departure))
}

pub async fn fetch_departures_by_station_code(
    _app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let station_code = params.into_inner();

    let ssl = {
        let mut ssl = SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();
        ssl.set_verify(SslVerifyMode::NONE);
        ssl.build()
    };

    let connector = awc::Connector::new().openssl(ssl);

    let ns_api_key = env::var("NS_API_KEY").expect("NS_API_KEY is not set in the .env file.");

    let max_journeys =
        env::var("NS_API_MAX_JOURNEYS").expect("NS_API_MAX_JOURNEYS is not set in the .env file.");

    let url = format!(
        "https://gateway.apiportal.ns.nl/reisinformatie-api/api/v2/departures?station={}&maxJourneys={}",
        station_code,
        max_journeys
    );

    let response = awc::ClientBuilder::new()
        .connector(connector)
        .finish()
        .get(url)
        .insert_header(("Ocp-Apim-Subscription-Key", ns_api_key))
        .insert_header(("User-Agent", "Actix-web"))
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    let value: Value = serde_json::from_str(&std::str::from_utf8(&response)?)?;

    if &value["code"] == 404 {
        return Ok(
            HttpResponse::Ok().body(format!("Station code `{}` is not valid.", station_code))
        );
    }

    let inner_value = &value["payload"]["departures"];

    println!("{:#?}", inner_value);

    let departures: Vec<ApiDeparture> = serde_json::from_value(inner_value.clone()).unwrap();

    println!("{:#?}", departures);

    Ok(HttpResponse::Ok().json(departures))
}

pub async fn download_departures_by_station_code(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let station_code = params.into_inner();

    let ssl = {
        let mut ssl = SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();
        ssl.set_verify(SslVerifyMode::NONE);
        ssl.build()
    };

    let connector = awc::Connector::new().openssl(ssl);

    let ns_api_key = env::var("NS_API_KEY").expect("NS_API_KEY is not set in the .env file.");

    let max_journeys =
        env::var("NS_API_MAX_JOURNEYS").expect("NS_API_MAX_JOURNEYS is not set in the .env file.");

    let url = format!(
        "https://gateway.apiportal.ns.nl/reisinformatie-api/api/v2/departures?station={}&maxJourneys={}",
        station_code,
        max_journeys
    );

    info!("NS API: {}", url);

    let response = awc::ClientBuilder::new()
        .connector(connector)
        .finish()
        .get(url)
        .insert_header(("Ocp-Apim-Subscription-Key", ns_api_key))
        .insert_header(("User-Agent", "Actix-web"))
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    let value: Value = serde_json::from_str(&std::str::from_utf8(&response)?)?;

    if &value["code"] == 404 {
        return Ok(
            HttpResponse::Ok().body(format!("Station code `{}` is not valid.", station_code))
        );
    }

    let inner_value = &value["payload"]["departures"];

    println!("{:?}", inner_value);

    let departures: Vec<ApiDeparture> = serde_json::from_value(inner_value.clone()).unwrap();

    let result = db_insert_downloaded_api_data(&app_state.pool, station_code, departures).await;

    Ok(HttpResponse::Ok().body(result.unwrap()))
}
