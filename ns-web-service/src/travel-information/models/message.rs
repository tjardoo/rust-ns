use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: u32,
    pub departure_id: u32,
    pub content: String,
    pub style: String,
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
