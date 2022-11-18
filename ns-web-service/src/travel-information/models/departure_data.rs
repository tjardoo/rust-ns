use super::departure::Departure;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[path = "../../travel-information/helpers/date_time_helper.rs"]
pub mod date_time_helper;

#[derive(Serialize, Deserialize, Debug)]
pub struct DepartureData {
    pub data: Departure,
    pub details: DepartureDataDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepartureDataDetails {
    pub station_code: String,

    #[serde(with = "date_time_helper::readable_date_format")]
    pub current_date_time: DateTime<Utc>,
}
