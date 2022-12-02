use crate::models::departure::{Departure, SimpleDeparture};
use crate::models::message::Message;
use crate::models::product::Product;
use crate::models::station::Station;
use crate::{errors::RustNSError, models::train_category::TrainCategory};
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;
use std::str::FromStr;

pub async fn db_get_departure_by_id(
    pool: &MySqlPool,
    id: u32,
) -> Result<SimpleDeparture, RustNSError> {
    let departure_row = sqlx::query_as!(
        SimpleDeparture,
        r#"SELECT
        id,
        station_code,
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
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .expect("Failed to execute query");

    if let Some(departure) = departure_row {
        Ok(departure)
    } else {
        Err(RustNSError::DatabaseError(
            "Departure with given id not found.".into(),
        ))
    }
}

pub async fn db_get_product_by_id(
    pool: &MySqlPool,
    product_id: u32,
) -> Result<Product, RustNSError> {
    let product_row = sqlx::query_as!(
        Product,
        r#"SELECT
        id,
        product_number as number,
        category_code,
        short_category_name,
        long_category_name,
        operator_code,
        operator_name,
        product_type as type
        FROM products
        WHERE id = ?
        "#,
        product_id
    )
    .fetch_one(pool)
    .await
    .expect("Failed to execute query");

    Ok(product_row)
}

pub async fn db_get_stations_by_departure_id(
    pool: &MySqlPool,
    departure_id: u32,
) -> Result<Vec<Station>, RustNSError> {
    let stations = sqlx::query_as!(
        Station,
        r#"SELECT
        stations.id,
        uic_code,
        medium_name
        FROM stations
        JOIN route_stations ON route_stations.station_id = stations.id
        WHERE route_stations.departure_id = ?
        "#,
        departure_id
    )
    .fetch_all(pool)
    .await
    .expect("Failed to execute query");

    if stations.len() > 0 {
        Ok(stations)
    } else {
        Ok(vec![])
    }
}

pub async fn db_get_messages_by_departure_id(
    pool: &MySqlPool,
    departure_id: u32,
) -> Result<Vec<Message>, RustNSError> {
    let message_rows = sqlx::query_as!(
        Message,
        r#"SELECT
        id,
        departure_id,
        content,
        style
        FROM messages
        WHERE departure_id = ?
        LIMIT 1
        "#,
        departure_id
    )
    .fetch_all(pool)
    .await?;

    Ok(message_rows)
}

pub async fn enhance_simple_departure(
    pool: &MySqlPool,
    simple_departure: Option<SimpleDeparture>,
) -> Option<Departure> {
    if simple_departure.is_none() {
        return None;
    }

    let simple_departure = simple_departure.unwrap();

    let product = db_get_product_by_id(pool, simple_departure.product_id)
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
