use crate::state::AppState;
use crate::{database::station::*, errors::RustNSError};
use actix_web::{web, HttpResponse};

pub async fn get_departures_by_station(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, RustNSError> {
    let station_code = params.into_inner();

    db_get_departures_by_station(&app_state.pool, station_code)
        .await
        .map(|departures| HttpResponse::Ok().json(departures))
}
