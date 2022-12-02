use crate::models::departure::Departure;
use crate::models::departure_data::{DepartureData, DepartureDataDetails};
use crate::models::train_category::TrainCategory;
use crate::state::AppState;
use crate::{database::departures::*, errors::RustNSError};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use std::str::FromStr;
use tracing::instrument;

#[instrument(skip(app_state))]
pub async fn get_departure_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<(String, u32)>,
) -> Result<HttpResponse, RustNSError> {
    let (station_code, departure_id) = params.into_inner();

    let simple_departure = db_get_departure_by_id(&app_state.pool, departure_id).await?;

    let product = db_get_product_by_id(&app_state.pool, departure_id).await?;

    let stations = db_get_stations_by_departure_id(&app_state.pool, departure_id).await?;

    let messages = db_get_messages_by_departure_id(&app_state.pool, departure_id).await?;

    let departure = Departure {
        id: simple_departure.id,
        station_code: simple_departure.station_code,
        direction: simple_departure.direction,
        name: simple_departure.name,
        planned_date_time: simple_departure.planned_date_time,
        actual_date_time: simple_departure.actual_date_time,
        planned_track: simple_departure.planned_track,
        product,
        train_category: TrainCategory::from_str(&simple_departure.train_category).unwrap(),
        is_cancelled: simple_departure.is_cancelled,
        route_stations: stations,
        messages: Some(messages),
        departure_status: simple_departure.departure_status,
    };

    let data = DepartureData {
        data: departure,
        details: DepartureDataDetails {
            station_code,
            current_date_time: Utc::now(),
        },
    };

    Ok(HttpResponse::Ok().json(data))
}
