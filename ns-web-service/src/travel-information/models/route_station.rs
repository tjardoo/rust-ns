use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteStation {
    pub id: u32,
    pub departure_id: u32,
    pub station_id: u32,
}

impl<'a> FromRow<'a, MySqlRow> for RouteStation {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(RouteStation {
            id: row.get("id"),
            departure_id: row.get("departure_id"),
            station_id: row.get("station_id"),
        })
    }
}
