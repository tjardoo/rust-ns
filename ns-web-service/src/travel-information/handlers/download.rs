use crate::download::departures::api_download_departures_by_station;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use std::error::Error;

pub async fn download_departures_by_station(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let station_code = params.into_inner();

    api_download_departures_by_station(&app_state.pool, station_code).await
}
