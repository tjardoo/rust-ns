use crate::errors::RustNSError;
use crate::models::departure::{Departure, Message, Product, RouteStation};
use sqlx::mysql::MySqlPool;

pub async fn db_get_departures(pool: &MySqlPool) -> Result<Vec<Departure>, RustNSError> {
    let departures = sqlx::query_as!(
        Departure,
        r#"SELECT 
        id,
        direction,
        train_name as name,
        planned_date_time as plannedDateTime,
        planned_time_zone_offset as plannedTimeZoneOffset,
        actual_date_time as actualDateTime,
        actual_time_zone_offset as actualTimeZoneOffset,
        planned_track as plannedTrack,
        product_id as productId,
        train_category as trainCategory,
        is_cancelled as "cancelled: bool",
        departure_status as departureStatus
        FROM departures
        "#
    )
    .fetch_all(pool)
    .await
    .expect("Failed to execute query");

    if departures.len() > 0 {
        Ok(departures)
    } else {
        Err(RustNSError::DatabaseError(
            "No departures found in the database.".into(),
        ))
    }
}

pub async fn db_get_departure_by_id(pool: &MySqlPool, id: u32) -> Result<Departure, RustNSError> {
    let departure_row = sqlx::query_as!(
        Departure,
        r#"SELECT 
        id,
        direction,
        train_name as name,
        planned_date_time as plannedDateTime,
        planned_time_zone_offset as plannedTimeZoneOffset,
        actual_date_time as actualDateTime,
        actual_time_zone_offset as actualTimeZoneOffset,
        planned_track as plannedTrack,
        product_id as productId,
        train_category as trainCategory,
        is_cancelled as "cancelled: bool",
        departure_status as departureStatus
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
        category_code as categoryCode,
        short_category_code as shortCategoryName,
        long_category_code as longCategoryName,
        operator_code as operatorCode,
        operator_name as operatorName,
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

pub async fn db_get_route_stations_by_departure_id(
    pool: &MySqlPool,
    departure_id: u32,
) -> Result<Vec<RouteStation>, RustNSError> {
    let route_stations = sqlx::query_as!(
        RouteStation,
        r#"SELECT 
        id,
        departure_id,
        uic_code as uicCode,
        medium_name as mediumName
        FROM route_stations
        WHERE departure_id = ?
        "#,
        departure_id
    )
    .fetch_all(pool)
    .await
    .expect("Failed to execute query");

    if route_stations.len() > 0 {
        Ok(route_stations)
    } else {
        Err(RustNSError::DatabaseError(
            "No route stations found with given departure id.".into(),
        ))
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
        content as message,
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
