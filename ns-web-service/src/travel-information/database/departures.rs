use crate::errors::RustNSError;
use crate::models::api_departure::ApiDeparture;
use crate::models::departure::{Departure, Message, Product, RouteStation};
use sqlx::mysql::MySqlPool;

pub async fn db_get_departures_by_station_code(
    pool: &MySqlPool,
    station_code: String,
) -> Result<Vec<Departure>, RustNSError> {
    let departures = sqlx::query_as!(
        Departure,
        r#"SELECT 
        id,
        station_code as stationCode,
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
        WHERE station_code = ?
        "#,
        station_code
    )
    .fetch_all(pool)
    .await
    .expect("Failed to execute query");

    if departures.len() > 0 {
        Ok(departures)
    } else {
        Err(RustNSError::DatabaseError(format!(
            "No departures found in the database for station_code = {}",
            station_code
        )))
    }
}

pub async fn db_get_departure_by_id(pool: &MySqlPool, id: u32) -> Result<Departure, RustNSError> {
    let departure_row = sqlx::query_as!(
        Departure,
        r#"SELECT 
        id,
        station_code as stationCode,
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

pub async fn db_insert_downloaded_api_data(
    pool: &MySqlPool,
    station_code: String,
    departures: Vec<ApiDeparture>,
) -> Result<String, RustNSError> {
    for departure in departures {
        let product_id = sqlx::query_as!(
            Departure,
            "INSERT INTO products (
            product_number,
            category_code,
            short_category_code,
            long_category_code,
            operator_code,
            operator_name,
            product_type
        ) values (?, ?, ?, ?, ?, ?, ?)",
            departure.product.number,
            departure.product.categoryCode,
            departure.product.shortCategoryName,
            departure.product.longCategoryName,
            departure.product.operatorCode,
            departure.product.operatorName,
            departure.product.r#type,
        )
        .execute(pool)
        .await?
        .last_insert_id();

        let departure_id = sqlx::query_as!(
            Departure,
            "INSERT INTO departures (
            station_code,
            direction,
            train_name,
            planned_date_time,
            planned_time_zone_offset,
            actual_date_time,
            actual_time_zone_offset,
            planned_track,
            product_id,
            train_category,
            is_cancelled,
            departure_status
        ) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            station_code,
            departure.direction,
            departure.name,
            departure.plannedDateTime,
            departure.plannedTimeZoneOffset,
            departure.actualDateTime,
            departure.actualTimeZoneOffset,
            departure.plannedTrack,
            product_id,
            departure.trainCategory,
            departure.cancelled,
            departure.departureStatus,
        )
        .execute(pool)
        .await?
        .last_insert_id();

        for route_station in departure.routeStations {
            sqlx::query_as!(
                RouteStation,
                "INSERT INTO route_stations (
                departure_id,
                uic_code,
                medium_name
            ) values (?, ?, ?)",
                departure_id,
                route_station.uicCode,
                route_station.mediumName,
            )
            .execute(pool)
            .await?
            .last_insert_id();
        }

        for message in departure.messages.unwrap() {
            sqlx::query_as!(
                Departure,
                "INSERT INTO messages (
                departure_id,
                content,
                style
            ) values (?, ?, ?)",
                departure_id,
                message.message,
                message.style,
            )
            .execute(pool)
            .await?
            .last_insert_id();
        }
    }

    Ok("Database updated.".into())
}
