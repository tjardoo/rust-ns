use super::departure::Departure;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[path = "../../travel-information/helpers/date_time_helper.rs"]
pub mod date_time_helper;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformData {
    pub data: PlatformDataDepartures,
    pub details: PlatformDataDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformDataDepartures {
    pub current: Option<Departure>,
    pub next: Option<Departure>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformDataDetails {
    pub station_code: String,

    #[serde(with = "date_time_helper::readable_date_format")]
    pub current_date_time: DateTime<Utc>,
}
