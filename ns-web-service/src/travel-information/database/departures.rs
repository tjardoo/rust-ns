use crate::errors::RustNSError;
use crate::models::departure::{Departure, TrainCategory};
use sqlx::mysql::MySqlPool;

pub async fn db_get_departures(pool: &MySqlPool) -> Result<Vec<Departure>, RustNSError> {
    let departures = sqlx::query_as!(
         Departure,
         r#"SELECT id, direction, train_name, planned_track, train_category as "train_category: TrainCategory" FROM departures"#
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
