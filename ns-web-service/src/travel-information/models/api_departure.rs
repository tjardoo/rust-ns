use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TrainCategory {
    SPR,
    IC,
    ICD,
    ICE,
    THA,
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiDeparture {
    pub direction: String,
    pub name: String,
    pub plannedDateTime: String,
    pub plannedTimeZoneOffset: i32,
    pub actualDateTime: String,
    pub actualTimeZoneOffset: i32,
    pub plannedTrack: String,
    pub product: ApiProduct,
    pub trainCategory: String,
    pub cancelled: bool,
    pub routeStations: Vec<ApiRouteStation>,
    pub messages: Option<Vec<ApiMessage>>,
    pub departureStatus: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiProduct {
    pub number: String,
    pub categoryCode: String,
    pub shortCategoryName: String,
    pub longCategoryName: String,
    pub operatorCode: String,
    pub operatorName: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiRouteStation {
    pub uicCode: String,
    pub mediumName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiMessage {
    pub message: String,
    pub style: String,
}
