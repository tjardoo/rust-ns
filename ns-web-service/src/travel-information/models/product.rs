use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

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
