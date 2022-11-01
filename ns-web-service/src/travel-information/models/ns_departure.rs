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
    product: Product,
    trainCategory: String,
    cancelled: bool,
    routeStations: Vec<RouteStation>,
    messages: Option<Vec<Message>>,
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

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Message {
    message: String,
    style: String,
}
