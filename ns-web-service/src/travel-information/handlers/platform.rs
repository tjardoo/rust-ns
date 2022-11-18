use crate::state::AppState;
use crate::{database::platform::*, errors::RustNSError};
use actix_web::{web, HttpResponse};

pub async fn get_departures_by_platform(
    app_state: web::Data<AppState>,
    params: web::Path<(String, String)>,
) -> Result<HttpResponse, RustNSError> {
    let (station_code, platform_code) = params.into_inner();

    db_get_departures_by_platform(&app_state.pool, station_code, platform_code)
        .await
        .map(|departures| HttpResponse::Ok().json(departures))
}
