use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub id: u32,
    pub uic_code: String,
    pub medium_name: String,
}

impl<'a> FromRow<'a, MySqlRow> for Station {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Station {
            id: row.get("id"),
            uic_code: row.get("uic_code"),
            medium_name: row.get("medium_name"),
        })
    }
}
