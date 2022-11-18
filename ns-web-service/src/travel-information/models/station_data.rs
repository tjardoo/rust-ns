use serde::{Deserialize, Serialize};

use super::departure::SimpleDeparture;

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
    pub current_date_time: String,
}
