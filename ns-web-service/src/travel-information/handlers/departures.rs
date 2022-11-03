use std::error::Error;
use std::str::FromStr;

use crate::models::api_departure::ApiDeparture;
use crate::models::departure::{FullDeparture, TrainCategory};
use crate::state::AppState;
use crate::{database::departures::*, errors::RustNSError};
use actix_web::{web, HttpResponse};
use openssl::ssl::{SslConnector, SslVerifyMode};
use serde_json::Value;
use std::env;

pub async fn get_departures(app_state: web::Data<AppState>) -> Result<HttpResponse, RustNSError> {
    db_get_departures(&app_state.pool)
        .await
        .map(|departures| HttpResponse::Ok().json(departures))
}

pub async fn get_departure(
    app_state: web::Data<AppState>,
    params: web::Path<u32>,
) -> Result<HttpResponse, RustNSError> {
    let departure_id: u32 = params.into_inner();

    let departure = db_get_departure_by_id(&app_state.pool, departure_id)
        .await
        .unwrap();

    let product = db_get_product_by_id(&app_state.pool, departure_id)
        .await
        .unwrap();

    let route_stations = db_get_route_stations_by_departure_id(&app_state.pool, departure_id)
        .await
        .unwrap();

    let messages = db_get_messages_by_departure_id(&app_state.pool, departure_id)
        .await
        .unwrap();

    let full_departure = FullDeparture {
        id: departure.id,
        direction: departure.direction,
        name: departure.name,
        plannedDateTime: departure.plannedDateTime,
        plannedTimeZoneOffset: departure.plannedTimeZoneOffset,
        actualDateTime: departure.actualDateTime,
        actualTimeZoneOffset: departure.actualTimeZoneOffset,
        plannedTrack: departure.plannedTrack,
        product,
        trainCategory: TrainCategory::from_str(&departure.trainCategory).unwrap(),
        cancelled: departure.cancelled,
        routeStations: route_stations,
        messages: Some(messages),
        departureStatus: departure.departureStatus,
    };

    Ok(HttpResponse::Ok().json(full_departure))
}

pub async fn download_departures(
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let ssl = {
        let mut ssl = SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();
        ssl.set_verify(SslVerifyMode::NONE);
        ssl.build()
    };

    let connector = awc::Connector::new().openssl(ssl);

    let ns_api_key = env::var("NS_API_KEY").expect("NS_API_KEY is not set in the .env file.");

    let response = awc::ClientBuilder::new()
        .connector(connector)
        .finish()
        .get("https://gateway.apiportal.ns.nl/reisinformatie-api/api/v2/departures?station=HN")
        .insert_header(("Ocp-Apim-Subscription-Key", ns_api_key))
        .insert_header(("User-Agent", "Actix-web"))
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    let value: Value = serde_json::from_str(&std::str::from_utf8(&response)?)?;

    let inner_value = &value["payload"]["departures"];

    println!("{:#?}", inner_value);

    let departures: Vec<ApiDeparture> = serde_json::from_value(inner_value.clone()).unwrap();

    println!("{:#?}", departures);

    Ok(HttpResponse::Ok().json(departures))
}
