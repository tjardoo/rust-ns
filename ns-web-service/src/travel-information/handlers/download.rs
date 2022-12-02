use crate::download::departures::api_download_departures_by_station;
use crate::download::departures::update_departures_by_station_in_database;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use std::error::Error;
use tracing::instrument;

#[instrument(skip(app_state))]
pub async fn download_departures_by_station(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let station_code = params.into_inner();

    let api_departures_result = api_download_departures_by_station(&station_code).await;

    if let Err(error) = api_departures_result {
        return Ok(HttpResponse::BadRequest().json(error.to_string()));
    }

    let departures = api_departures_result.unwrap();

    let result = departures.clone();

    update_departures_by_station_in_database(&app_state.pool, station_code, departures).await;

    Ok(HttpResponse::Ok().json(result))
}
