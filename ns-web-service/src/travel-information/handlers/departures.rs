use crate::models::departure::Departure;
use crate::models::train_category::TrainCategory;
use crate::state::AppState;
use crate::{database::departures::*, errors::RustNSError};
use actix_web::{web, HttpResponse};
use std::str::FromStr;

pub async fn get_departure_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<(String, u32)>,
) -> Result<HttpResponse, RustNSError> {
    let (_station_code, departure_id) = params.into_inner();

    let departure = db_get_departure_by_id(&app_state.pool, departure_id).await?;

    let product = db_get_product_by_id(&app_state.pool, departure_id).await?;

    let stations = db_get_stations_by_departure_id(&app_state.pool, departure_id).await?;

    let messages = db_get_messages_by_departure_id(&app_state.pool, departure_id).await?;

    let full_departure = Departure {
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
