use crate::errors::RustNSError;
use crate::models::api_departure::ApiDeparture;
use crate::models::departure::{Departure, Message, Product, Station};
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;

pub async fn db_get_departures_by_station_code(
    pool: &MySqlPool,
    station_code: String,
) -> Result<Vec<Departure>, RustNSError> {
    let departures = sqlx::query_as!(
        Departure,
        r#"SELECT 
        id,
        station_code as station_code,
        direction,
        train_name as name,
        planned_date_time as "planned_date_time: NaiveDateTime",
        actual_date_time as "actual_date_time: NaiveDateTime",
        planned_track,
        product_id,
        train_category,
        is_cancelled as "is_cancelled: bool",
        departure_status
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
        station_code,
        direction,
        train_name as name,
        planned_date_time as "planned_date_time: NaiveDateTime",
        actual_date_time as "actual_date_time: NaiveDateTime",
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

pub async fn db_get_departure_by_station_code_and_platform_code(
    pool: &MySqlPool,
    station_code: String,
    platform_code: String,
) -> Result<Option<Departure>, RustNSError> {
    let departure_row = sqlx::query_as!(
        Departure,
        r#"SELECT 
        id,
        station_code,
        direction,
        train_name as name,
        planned_date_time as "planned_date_time: NaiveDateTime",
        actual_date_time as "actual_date_time: NaiveDateTime",
        planned_track,
        product_id,
        train_category,
        is_cancelled as "is_cancelled: bool",
        departure_status
        FROM departures
        WHERE station_code = ?
        AND
        planned_track = ?
        "#,
        station_code,
        platform_code,
    )
    .fetch_optional(pool)
    .await
    .expect("Failed to execute query");

    if let Some(departure) = departure_row {
        Ok(Some(departure))
    } else {
        Ok(None)
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
            short_category_name,
            long_category_name,
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

        let planned_track = match departure.plannedTrack {
            Some(p) => p,
            None => String::from("N/A"),
        };

        let departure_id = sqlx::query_as!(
            Departure,
            "INSERT INTO departures (
            station_code,
            direction,
            train_name,
            planned_date_time,
            actual_date_time,
            planned_track,
            product_id,
            train_category,
            is_cancelled,
            departure_status
        ) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            station_code,
            departure.direction,
            departure.name,
            NaiveDateTime::parse_from_str(&departure.plannedDateTime, "%Y-%m-%dT%H:%M:%S%.f%z")
                .unwrap(),
            NaiveDateTime::parse_from_str(&departure.actualDateTime, "%Y-%m-%dT%H:%M:%S%.f%z")
                .unwrap(),
            planned_track,
            product_id,
            departure.trainCategory,
            departure.cancelled,
            departure.departureStatus,
        )
        .execute(pool)
        .await?
        .last_insert_id();

        for route_station in departure.routeStations {
            let optional_station = sqlx::query_as!(
                Station,
                r#"SELECT 
                id,
                uic_code,
                medium_name
                FROM stations
                WHERE uic_code = ?
                "#,
                route_station.uicCode
            )
            .fetch_optional(pool)
            .await
            .expect("Failed to execute query");

            let station_id: u32;

            if let Some(station) = optional_station {
                station_id = station.id;
            } else {
                station_id = sqlx::query_as!(
                    Station,
                    "INSERT INTO stations (
                        uic_code,
                        medium_name
                    ) values (?, ?)",
                    route_station.uicCode,
                    route_station.mediumName
                )
                .execute(pool)
                .await?
                .last_insert_id() as u32;
            };

            sqlx::query_as!(
                RouteStation,
                "INSERT INTO route_stations (
                departure_id,
                station_id
            ) values (?, ?)",
                departure_id,
                station_id
            )
            .execute(pool)
            .await?;
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
