use serde::{Deserialize, Serialize};

use super::departure::SimpleDeparture;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformData {
    pub data: PlatformDataDepartures,
    pub details: PlatformDataDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformDataDepartures {
    pub current: Option<SimpleDeparture>,
    pub next: Option<SimpleDeparture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformDataDetails {
    pub station_code: String,
    pub current_date_time: String,
}
