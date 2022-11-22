use crate::errors::RustNSError;
use crate::models::departure::{Departure, SimpleDeparture};
use crate::models::platform_data::{PlatformData, PlatformDataDepartures, PlatformDataDetails};
use crate::models::train_category::TrainCategory;
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;
use std::str::FromStr;

use super::departures::*;

pub async fn db_get_departures_by_platform(
    pool: &MySqlPool,
    station_code: String,
    platform_code: String,
) -> Result<PlatformData, RustNSError> {
    let departures = sqlx::query_as!(
        SimpleDeparture,
        r#"SELECT 
        id,
        station_code as station_code,
        direction,
        train_name as name,
        planned_date_time as "planned_date_time: DateTime<Utc>",
        actual_date_time as "actual_date_time: DateTime<Utc>",
        planned_track,
        product_id,
        train_category,
        is_cancelled as "is_cancelled: bool",
        departure_status
        FROM departures
        WHERE station_code = ?
        AND
        planned_track = ?
        LIMIT 2
        "#,
        station_code,
        platform_code
    )
    .fetch_all(pool)
    .await
    .expect("Failed to execute query");

    let mut iter = departures.into_iter();

    let current = enhance_simple_departure(pool, iter.next()).await;

    let next = enhance_simple_departure(pool, iter.next()).await;

    Ok(PlatformData {
        data: PlatformDataDepartures { current, next },
        details: PlatformDataDetails {
            station_code,
            current_date_time: Utc::now(),
        },
    })
}

async fn enhance_simple_departure(
    pool: &MySqlPool,
    simple_departure: Option<SimpleDeparture>,
) -> Option<Departure> {
    if simple_departure.is_none() {
        return None;
    }

    let simple_departure = simple_departure.unwrap();

    let product = db_get_product_by_id(pool, simple_departure.id)
        .await
        .unwrap();

    let stations = db_get_stations_by_departure_id(pool, simple_departure.id)
        .await
        .unwrap();

    let messages = db_get_messages_by_departure_id(pool, simple_departure.id)
        .await
        .unwrap();

    Some(Departure {
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
    })
}
