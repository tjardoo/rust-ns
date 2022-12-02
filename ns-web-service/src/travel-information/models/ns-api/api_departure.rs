use super::{api_message::ApiMessage, api_product::ApiProduct, api_station::ApiRouteStation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ApiDeparture {
    pub direction: String,
    pub name: String,
    pub plannedDateTime: String,
    pub plannedTimeZoneOffset: i32,
    pub actualDateTime: String,
    pub actualTimeZoneOffset: i32,
    pub plannedTrack: Option<String>,
    pub product: ApiProduct,
    pub trainCategory: String,
    pub cancelled: bool,
    pub routeStations: Vec<ApiRouteStation>,
    pub messages: Option<Vec<ApiMessage>>,
    pub departureStatus: String,
}
