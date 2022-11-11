use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

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
pub struct FullDeparture {
    pub id: u32,
    pub stationCode: String,
    pub direction: String,
    pub name: String,
    pub plannedDateTime: NaiveDateTime,
    pub actualDateTime: NaiveDateTime,
    pub plannedTrack: String,
    pub product: Product,
    pub trainCategory: TrainCategory,
    pub cancelled: bool,
    pub routeStations: Vec<RouteStation>,
    pub messages: Option<Vec<Message>>,
    pub departureStatus: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Departure {
    pub id: u32,
    pub stationCode: String,
    pub direction: String,
    pub name: String,
    pub plannedDateTime: NaiveDateTime,
    pub actualDateTime: NaiveDateTime,
    pub plannedTrack: String,
    pub productId: i32,
    pub trainCategory: String,
    pub cancelled: bool,
    pub departureStatus: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Product {
    pub id: u32,
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
pub struct RouteStation {
    pub id: u32,
    pub departure_id: u32,
    pub uicCode: String,
    pub mediumName: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Message {
    pub id: u32,
    pub departure_id: u32,
    pub message: String,
    pub style: String,
}

impl std::str::FromStr for TrainCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SPR" => Ok(TrainCategory::SPR),
            "IC" => Ok(TrainCategory::IC),
            "ICD" => Ok(TrainCategory::ICD),
            "ICE" => Ok(TrainCategory::ICE),
            "THA" => Ok(TrainCategory::THA),
            "UNKOWN" => Ok(TrainCategory::UNKNOWN),
            _ => Err(format!("'{}' is not a valid value for TrainCategory", s)),
        }
    }
}

impl<'a> FromRow<'a, MySqlRow> for Departure {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        let planned_date_time =
            NaiveDateTime::parse_from_str(row.get("planned_date_time"), "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let actual_date_time =
            NaiveDateTime::parse_from_str(row.get("acutal_date_time"), "%Y-%m-%d %H:%M:%S")
                .unwrap();

        Ok(Departure {
            id: row.get("id"),
            stationCode: row.get("station_code"),
            direction: row.get("direction"),
            name: row.get("train_name"),
            plannedDateTime: planned_date_time,
            actualDateTime: actual_date_time,
            plannedTrack: row.get("planned_track"),
            productId: row.get("product_id"),
            trainCategory: row.get("train_category"),
            cancelled: row.get("is_cancelled"),
            departureStatus: row.get("departure_status"),
        })
    }
}

impl<'a> FromRow<'a, MySqlRow> for Product {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Product {
            id: row.get("id"),
            number: row.get("product_number"),
            categoryCode: row.get("category_code"),
            shortCategoryName: row.get("short_category_code"),
            longCategoryName: row.get("long_category_code"),
            operatorCode: row.get("operator_code"),
            operatorName: row.get("operator_name"),
            r#type: row.get("product_type"),
        })
    }
}

impl<'a> FromRow<'a, MySqlRow> for RouteStation {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(RouteStation {
            id: row.get("id"),
            departure_id: row.get("departure_id"),
            uicCode: row.get("uic_code"),
            mediumName: row.get("medium_name"),
        })
    }
}

impl<'a> FromRow<'a, MySqlRow> for Message {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Message {
            id: row.get("id"),
            departure_id: row.get("departure_id"),
            message: row.get("content"),
            style: row.get("style"),
        })
    }
}
