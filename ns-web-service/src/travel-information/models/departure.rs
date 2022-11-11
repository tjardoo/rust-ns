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
pub struct FullDeparture {
    pub id: u32,
    pub station_code: String,
    pub direction: String,
    pub name: String,
    pub planned_date_time: NaiveDateTime,
    pub actual_date_time: NaiveDateTime,
    pub planned_track: String,
    pub product: Product,
    pub train_category: TrainCategory,
    pub is_cancelled: bool,
    pub route_stations: Vec<RouteStation>,
    pub messages: Option<Vec<Message>>,
    pub departure_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    pub id: u32,
    pub station_code: String,
    pub direction: String,
    pub name: String,
    pub planned_date_time: NaiveDateTime,
    pub actual_date_time: NaiveDateTime,
    pub planned_track: String,
    pub product_id: i32,
    pub train_category: String,
    pub is_cancelled: bool,
    pub departure_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: u32,
    pub number: String,
    pub category_code: String,
    pub short_category_name: String,
    pub long_category_name: String,
    pub operator_code: String,
    pub operator_name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteStation {
    pub id: u32,
    pub departure_id: u32,
    pub uic_code: String,
    pub medium_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: u32,
    pub departure_id: u32,
    pub content: String,
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
            station_code: row.get("station_code"),
            direction: row.get("direction"),
            name: row.get("train_name"),
            planned_date_time,
            actual_date_time,
            planned_track: row.get("planned_track"),
            product_id: row.get("product_id"),
            train_category: row.get("train_category"),
            is_cancelled: row.get("is_cancelled"),
            departure_status: row.get("departure_status"),
        })
    }
}

impl<'a> FromRow<'a, MySqlRow> for Product {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Product {
            id: row.get("id"),
            number: row.get("product_number"),
            category_code: row.get("category_code"),
            short_category_name: row.get("short_category_name"),
            long_category_name: row.get("long_category_name"),
            operator_code: row.get("operator_code"),
            operator_name: row.get("operator_name"),
            r#type: row.get("product_type"),
        })
    }
}

impl<'a> FromRow<'a, MySqlRow> for RouteStation {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(RouteStation {
            id: row.get("id"),
            departure_id: row.get("departure_id"),
            uic_code: row.get("uic_code"),
            medium_name: row.get("medium_name"),
        })
    }
}

impl<'a> FromRow<'a, MySqlRow> for Message {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Message {
            id: row.get("id"),
            departure_id: row.get("departure_id"),
            content: row.get("content"),
            style: row.get("style"),
        })
    }
}
