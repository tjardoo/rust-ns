use crate::api_models::api_departure::ApiDeparture;
use crate::api_models::api_product::ApiProduct;
use crate::api_models::api_station::ApiRouteStation;
use crate::models::product::Product;
use crate::models::station::Station;
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use log::info;
use openssl::ssl::{SslConnector, SslVerifyMode};
use serde_json::Value;
use sqlx::mysql::MySqlPool;
use std::env;
use std::error::Error;

pub async fn api_download_departures_by_station(
    pool: &MySqlPool,
    station_code: String,
) -> Result<HttpResponse, Box<dyn Error>> {
    let ssl = {
        let mut ssl = SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();
        ssl.set_verify(SslVerifyMode::NONE);
        ssl.build()
    };

    let connector = awc::Connector::new().openssl(ssl);

    let ns_api_key = env::var("NS_API_KEY").expect("NS_API_KEY is not set in the .env file.");

    let max_journeys =
        env::var("NS_API_MAX_JOURNEYS").expect("NS_API_MAX_JOURNEYS is not set in the .env file.");

    let url = format!(
        "https://gateway.apiportal.ns.nl/reisinformatie-api/api/v2/departures?station={}&maxJourneys={}",
        station_code,
        max_journeys
    );

    info!("NS API: {}", url);

    let response = awc::ClientBuilder::new()
        .connector(connector)
        .finish()
        .get(url)
        .insert_header(("Ocp-Apim-Subscription-Key", ns_api_key))
        .insert_header(("User-Agent", "Actix-web"))
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    let value: Value = serde_json::from_str(&std::str::from_utf8(&response)?)?;

    if &value["code"] == 404 {
        return Ok(
            HttpResponse::Ok().body(format!("Station code `{}` is not valid.", station_code))
        );
    }

    let inner_value = &value["payload"]["departures"];

    println!("{:?}", inner_value);

    let departures: Vec<ApiDeparture> = serde_json::from_value(inner_value.clone()).unwrap();

    update_departures_by_station_in_database(&pool, station_code, departures).await;

    Ok(HttpResponse::Ok().json(value))
}

pub async fn update_departures_by_station_in_database(
    pool: &MySqlPool,
    station_code: String,
    departures: Vec<ApiDeparture>,
) {
    for departure in departures {
        let product_id = update_or_create_product(pool, departure.product).await;

        let planned_track = match departure.plannedTrack {
            Some(p) => p,
            None => "N/A".to_string(),
        };

        delete_existing_departure(
            pool,
            &station_code,
            &departure.name,
            &departure.plannedDateTime,
            &planned_track,
        )
        .await;

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
        .await
        .unwrap()
        .last_insert_id();

        for route_station in departure.routeStations {
            let station_id = update_or_create_station(&pool, route_station).await;

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
            .await
            .unwrap();
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
            .await
            .unwrap()
            .last_insert_id();
        }
    }
}

pub async fn update_or_create_product(pool: &MySqlPool, product: ApiProduct) -> u32 {
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
            WHERE product_number = ?
            AND category_code = ?
            AND operator_code = ?
            "#,
        product.number,
        product.categoryCode,
        product.operatorCode
    )
    .fetch_optional(pool)
    .await
    .expect("Failed to execute query");

    if let Some(product) = product_row {
        return product.id;
    }

    sqlx::query_as!(
        Product,
        "INSERT INTO products (
        product_number,
        category_code,
        short_category_name,
        long_category_name,
        operator_code,
        operator_name,
        product_type
    ) values (?, ?, ?, ?, ?, ?, ?)",
        product.number,
        product.categoryCode,
        product.shortCategoryName,
        product.longCategoryName,
        product.operatorCode,
        product.operatorName,
        product.r#type,
    )
    .execute(pool)
    .await
    .unwrap()
    .last_insert_id() as u32
}

pub async fn update_or_create_station(pool: &MySqlPool, route_station: ApiRouteStation) -> u32 {
    let station_row = sqlx::query_as!(
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

    if let Some(station) = station_row {
        return station.id;
    }

    sqlx::query_as!(
        Station,
        "INSERT INTO stations (
            uic_code,
            medium_name
        ) values (?, ?)",
        route_station.uicCode,
        route_station.mediumName
    )
    .execute(pool)
    .await
    .unwrap()
    .last_insert_id() as u32
}

pub async fn delete_existing_departure(
    pool: &MySqlPool,
    station_code: &String,
    name: &String,
    planned_date_time: &String,
    planned_track: &String,
) {
    let planned_date_time =
        NaiveDateTime::parse_from_str(&planned_date_time, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap();

    sqlx::query(
        r#"DELETE
        FROM departures
        WHERE station_code = ?
        AND train_name = ?
        AND planned_date_time = ?
        AND planned_track = ?
        "#,
    )
    .bind(&station_code)
    .bind(&name)
    .bind(planned_date_time)
    .bind(&planned_track)
    .execute(pool)
    .await
    .expect("Failed to execute query");
}
