use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::{self, FromStr};

#[derive(Debug, Deserialize, Serialize)]
pub enum TrainCategory {
    SPR,
    IC,
    ICD,
    ICE,
    THA,
    UNKNOWN,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum DepartureStatus {
    INCOMING,
    ON_STATION,
}

#[derive(Serialize, Debug)]
pub struct DepartureList {
    pub direction: String,
    pub name: String,
    pub planned_date_time: String,
    pub actual_date_time: String,
    pub planned_track: String,
    pub train_category: TrainCategory,
    pub is_cancelled: bool,
    pub departure_status: String,
}

impl<'de> Deserialize<'de> for DepartureList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Outer {
            direction: String,
            name: String,
            planned_date_time: String,
            actual_date_time: String,
            planned_track: String,
            train_category: String,
            is_cancelled: bool,
            departure_status: String,
        }

        let helper = Outer::deserialize(deserializer)?;

        let actual_date_time =
            NaiveDateTime::parse_from_str(&helper.planned_date_time, "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

        let planned_date_time =
            NaiveDateTime::parse_from_str(&helper.actual_date_time, "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

        Ok(DepartureList {
            direction: helper.direction,
            name: helper.name,
            planned_date_time,
            actual_date_time,
            planned_track: helper.planned_track,
            train_category: TrainCategory::from_str(&helper.train_category).unwrap(),
            is_cancelled: helper.is_cancelled,
            departure_status: helper.departure_status,
        })
    }
}

impl str::FromStr for TrainCategory {
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
