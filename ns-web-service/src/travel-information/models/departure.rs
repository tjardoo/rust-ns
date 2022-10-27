use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "train_category")]
pub enum TrainCategory {
    #[sqlx(rename = "spr")]
    SPR,
    #[sqlx(rename = "ic")]
    IC,
    #[sqlx(rename = "unknown")]
    UNKNOWN,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Departure {
    pub id: i32,
    pub direction: String,
    pub train_name: String,
    pub planned_track: String,
    pub train_category: TrainCategory,
}

impl std::str::FromStr for TrainCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spr" => Ok(TrainCategory::SPR),
            "ic" => Ok(TrainCategory::IC),
            "unknown" => Ok(TrainCategory::UNKNOWN),
            _ => Err(format!("'{}' is not a valid value for TrainCategory", s)),
        }
    }
}

impl<'a> FromRow<'a, MySqlRow> for Departure {
    fn from_row(row: &'a MySqlRow) -> Result<Self, sqlx::Error> {
        let tc: String = row.get("train_category");

        let train_category = match TrainCategory::from_str(&tc) {
            Ok(t) => t,
            Err(_) => TrainCategory::UNKNOWN,
        };

        Ok(Departure {
            id: row.get("index"),
            direction: row.get("direction"),
            train_name: row.get("train_name"),
            planned_track: row.get("planned_track"),
            train_category: train_category,
        })
    }
}
