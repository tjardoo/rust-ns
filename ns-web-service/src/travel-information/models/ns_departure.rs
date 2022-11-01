use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Departure {
    direction: String,
    name: String,
    plannedDateTime: String,
    plannedTimeZoneOffset: i32,
    actualDateTime: String,
    actualTimeZoneOffset: i32,
    plannedTrack: String,
    product: Vec<Product>,
    trainCategory: String,
    cancelled: bool,
    routeStations: Vec<RouteStation>,
    messages: Vec<String>,
    departureStatus: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Product {
    number: String,
    categoryCode: String,
    shortCategoryName: String,
    longCategoryName: String,
    operatorCode: String,
    operatorName: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct RouteStation {
    uicCode: String,
    mediumName: String,
}
