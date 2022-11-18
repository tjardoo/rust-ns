use crate::errors::RustNSError;
use crate::models::departure::SimpleDeparture;
use crate::models::message::Message;
use crate::models::product::Product;
use crate::models::station::Station;
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;

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
