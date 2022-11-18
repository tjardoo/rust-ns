use super::departure::SimpleDeparture;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[path = "../../travel-information/helpers/date_time_helper.rs"]
pub mod date_time_helper;

#[derive(Serialize, Deserialize, Debug)]
pub struct StationData {
    pub data: StationDataDepartures,
    pub details: StationDataDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StationDataDepartures {
    pub departures: Vec<SimpleDeparture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StationDataDetails {
    pub station_code: String,

    #[serde(with = "date_time_helper::readable_date_format")]
    pub current_date_time: DateTime<Utc>,
}
