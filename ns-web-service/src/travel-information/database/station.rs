use crate::errors::RustNSError;
use crate::models::departure::SimpleDeparture;
use crate::models::station_data::{StationData, StationDataDepartures, StationDataDetails};
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;

pub async fn db_get_departures_by_station(
    pool: &MySqlPool,
    station_code: String,
    limit: u32,
) -> Result<StationData, RustNSError> {
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
        LIMIT ?
        "#,
        station_code,
        limit
    )
    .fetch_all(pool)
    .await
    .expect("Failed to execute query");

    Ok(StationData {
        data: StationDataDepartures { departures },
        details: StationDataDetails {
            station_code,
            current_date_time: Utc::now(),
        },
    })
}
