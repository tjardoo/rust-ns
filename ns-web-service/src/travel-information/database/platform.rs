use crate::errors::RustNSError;
use crate::models::departure::SimpleDeparture;
use crate::models::platform_data::{PlatformData, PlatformDataDepartures, PlatformDataDetails};
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;

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
