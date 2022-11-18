use super::{message::Message, product::Product, station::Station, train_category::TrainCategory};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[path = "../../travel-information/helpers/date_time_helper.rs"]
pub mod date_time_helper;

#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    pub id: u32,
    pub station_code: String,
    pub direction: String,
    pub name: String,

    #[serde(with = "date_time_helper::readable_date_format")]
    pub planned_date_time: DateTime<Utc>,

    #[serde(with = "date_time_helper::readable_date_format")]
    pub actual_date_time: DateTime<Utc>,

    pub planned_track: String,
    pub product: Product,
    pub train_category: TrainCategory,
    pub is_cancelled: bool,
    pub route_stations: Vec<Station>,
    pub messages: Option<Vec<Message>>,
    pub departure_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleDeparture {
    pub id: u32,
    pub station_code: String,
    pub direction: String,
    pub name: String,

    #[serde(with = "date_time_helper::readable_date_format")]
    pub planned_date_time: DateTime<Utc>,

    #[serde(with = "date_time_helper::readable_date_format")]
    pub actual_date_time: DateTime<Utc>,

    pub planned_track: String,
    pub product_id: i32,
    pub train_category: String,
    pub is_cancelled: bool,
    pub departure_status: String,
}

impl<'a> FromRow<'a, MySqlRow> for SimpleDeparture {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        let planned_date_time = DateTime::from_utc(row.get("planned_date_time"), Utc);
        let actual_date_time = DateTime::from_utc(row.get("acutal_date_time"), Utc);

        Ok(SimpleDeparture {
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
